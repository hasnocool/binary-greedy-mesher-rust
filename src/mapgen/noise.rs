// Procedural noise functions
use std::f32::consts::SQRT_2;

pub fn hash_u32(x: u32, seed: u32) -> u32 {
    let mut h = x ^ seed;
    h = h.wrapping_mul(0x9E3779B1);
    h ^= h >> 16;
    h = h.wrapping_mul(0x85EBCA6B);
    h ^= h >> 13;
    h = h.wrapping_mul(0xC2B2AE35);
    h ^= h >> 16;
    h
}

pub fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

pub fn perlin3d(x: f32, y: f32, z: f32, seed: u32) -> f32 {
    let x0 = x.floor() as i32;
    let y0 = y.floor() as i32;
    let z0 = z.floor() as i32;

    let xf = x - x0 as f32;
    let yf = y - y0 as f32;
    let zf = z - z0 as f32;

    let h = |ix: i32, iy: i32, iz: i32| -> u32 {
        let key = (ix as u32)
            .wrapping_mul(73856093)
            .wrapping_add((iy as u32).wrapping_mul(19349663))
            .wrapping_add((iz as u32).wrapping_mul(83492791));
        hash_u32(key, seed)
    };

    // 12 gradient directions (normalized)
    let grads: [(f32, f32, f32); 12] = [
        (1.0, 1.0, 0.0),
        (-1.0, 1.0, 0.0),
        (1.0, -1.0, 0.0),
        (-1.0, -1.0, 0.0),
        (1.0, 0.0, 1.0),
        (-1.0, 0.0, 1.0),
        (1.0, 0.0, -1.0),
        (-1.0, 0.0, -1.0),
        (0.0, 1.0, 1.0),
        (0.0, -1.0, 1.0),
        (0.0, 1.0, -1.0),
        (0.0, -1.0, -1.0),
    ];

    let grad_dot = |hashv: u32, dx: f32, dy: f32, dz: f32| -> f32 {
        let gi = (hashv % 12) as usize;
        let (gx, gy, gz) = grads[gi];
        (gx * dx + gy * dy + gz * dz) / SQRT_2
    };

    let h000 = h(x0, y0, z0);
    let h100 = h(x0 + 1, y0, z0);
    let h010 = h(x0, y0 + 1, z0);
    let h110 = h(x0 + 1, y0 + 1, z0);

    let h001 = h(x0, y0, z0 + 1);
    let h101 = h(x0 + 1, y0, z0 + 1);
    let h011 = h(x0, y0 + 1, z0 + 1);
    let h111 = h(x0 + 1, y0 + 1, z0 + 1);

    let n000 = grad_dot(h000, xf, yf, zf);
    let n100 = grad_dot(h100, xf - 1.0, yf, zf);
    let n010 = grad_dot(h010, xf, yf - 1.0, zf);
    let n110 = grad_dot(h110, xf - 1.0, yf - 1.0, zf);

    let n001 = grad_dot(h001, xf, yf, zf - 1.0);
    let n101 = grad_dot(h101, xf - 1.0, yf, zf - 1.0);
    let n011 = grad_dot(h011, xf, yf - 1.0, zf - 1.0);
    let n111 = grad_dot(h111, xf - 1.0, yf - 1.0, zf - 1.0);

    let u = fade(xf);
    let v = fade(yf);
    let w = fade(zf);

    let nx00 = lerp(n000, n100, u);
    let nx10 = lerp(n010, n110, u);
    let nxy0 = lerp(nx00, nx10, v);

    let nx01 = lerp(n001, n101, u);
    let nx11 = lerp(n011, n111, u);
    let nxy1 = lerp(nx01, nx11, v);

    lerp(nxy0, nxy1, w)
}

pub fn fbm3d(x: f32, y: f32, z: f32, seed: u32, octaves: u32, lacunarity: f32, gain: f32) -> f32 {
    let mut out = 0.0;
    let mut amp = 1.0;
    let mut freq = 1.0;
    for i in 0..octaves {
        out += amp * perlin3d(x * freq, y * freq, z * freq, seed + i * 1013);
        freq *= lacunarity;
        amp *= gain;
    }
    out
}

pub fn ridged_fbm3d(
    x: f32,
    y: f32,
    z: f32,
    seed: u32,
    octaves: u32,
    lacunarity: f32,
    gain: f32,
) -> f32 {
    let mut out = 0.0;
    let mut amp = 1.0;
    let mut freq = 1.0;
    for i in 0..octaves {
        let n = perlin3d(x * freq, y * freq, z * freq, seed + i * 1013);
        let r = 1.0 - n.abs();
        out += amp * r;
        freq *= lacunarity;
        amp *= gain;
    }
    out
}

pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
