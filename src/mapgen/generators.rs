use super::config::MapGenConfig;
use super::noise::{fbm3d, ridged_fbm3d, smoothstep};
use glam::IVec3;
use crate::CS;

pub trait Generator: Send + Sync {
    fn generate_density(&self, wx: f32, wy: f32, wz: f32) -> f32;
    fn config(&self) -> &MapGenConfig;
}

pub struct MultiNoiseGenerator {
    config: MapGenConfig,
    erosion_flattens: f32,
}

impl MultiNoiseGenerator {
    pub fn new(config: MapGenConfig) -> Self {
        Self {
            config,
            erosion_flattens: 0.85,
        }
    }
}

impl Generator for MultiNoiseGenerator {
    fn config(&self) -> &MapGenConfig {
        &self.config
    }

    fn generate_density(&self, wx: f32, wy: f32, wz: f32) -> f32 {
        let cfg = &self.config;

        // Sample each layer
        let cont_raw = fbm3d(
            wx / cfg.continentalness.scale,
            wy / cfg.continentalness.scale,
            wz / cfg.continentalness.scale,
            cfg.seed + 11,
            cfg.continentalness.octaves,
            cfg.continentalness.lacunarity,
            cfg.continentalness.gain,
        );

        let eros_raw = fbm3d(
            wx / cfg.erosion.scale,
            wy / cfg.erosion.scale,
            wz / cfg.erosion.scale,
            cfg.seed + 23,
            cfg.erosion.octaves,
            cfg.erosion.lacunarity,
            cfg.erosion.gain,
        );

        let pv_ridged = ridged_fbm3d(
            wx / cfg.peaks_valleys.scale,
            wy / cfg.peaks_valleys.scale,
            wz / cfg.peaks_valleys.scale,
            cfg.seed + 37,
            cfg.peaks_valleys.octaves,
            cfg.peaks_valleys.lacunarity,
            cfg.peaks_valleys.gain,
        );

        // Normalize/remap (approximate)
        let cont = cont_raw.clamp(-2.0, 2.0) / 2.0; // ~[-1,1]
        let eros01 = (eros_raw.clamp(-2.0, 2.0) / 4.0 + 0.5).clamp(0.0, 1.0);
        let pv01 = (pv_ridged / 4.0).clamp(0.0, 1.0);

        // Convert ridges to peaks+valleys
        let pv_centered = pv01 * 2.0 - 1.0;
        let pv = pv_centered.signum() * pv_centered.abs().powf(1.35);

        // Landmask from continentalness
        let cont01 = (cont + 1.0) / 2.0;
        let landmask = smoothstep(0.45, 0.65, cont01);

        // Erosion flattens peaks
        let peak_amp = 1.0 - self.erosion_flattens * eros01;

        // Vertical gradient (Y is up)
        let ynorm = ((wy - cfg.vertical.min_y) / (cfg.vertical.max_y - cfg.vertical.min_y))
            .clamp(0.0, 1.0);
        let vertical = (1.0 - ynorm) * cfg.vertical.bias;

        // Stack
        let density = cfg.weights.continentalness * cont
            + cfg.weights.peaks_valleys * (pv * landmask * peak_amp)
            - cfg.weights.erosion * (eros01 * 0.5)
            + vertical;

        density
    }
}

// Helper function for generating chunk voxels
pub fn generate_chunk_voxels<G: Generator>(
    chunk_pos: IVec3,
    generator: &G,
) -> (Vec<u8>, usize) {
    let cfg = generator.config();
    let cs_p3 = (CS + 2) * (CS + 2) * (CS + 2);
    let mut voxels = vec![0u8; cs_p3];
    let mut solid_count = 0;

    // Chunk origin in world space
    let origin = chunk_pos * CS as i32;

    for lz in 0..CS as i32 + 2 {
        for ly in 0..CS as i32 + 2 {
            for lx in 0..CS as i32 + 2 {
                let wx = (origin.x + lx - 1) as f32;
                let wy = (origin.y + ly - 1) as f32;
                let wz = (origin.z + lz - 1) as f32;

                let density = generator.generate_density(wx, wy, wz);

                let voxel = if density > cfg.density_threshold {
                    solid_count += 1;
                    // Pick material based on height
                    let h = wy as i32;
                    if h < cfg.material_thresholds.stone_max {
                        1 // stone-like
                    } else if h < cfg.material_thresholds.grass_max {
                        3 // grass-like
                    } else {
                        2 // dirt-like
                    }
                } else {
                    0 // air
                };

                let idx = (lz as usize) * (CS + 2) * (CS + 2) + (ly as usize) * (CS + 2) + (lx as usize);
                voxels[idx] = voxel;
            }
        }
    }

    (voxels, solid_count)
}
