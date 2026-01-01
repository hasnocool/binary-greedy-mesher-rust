# binary-greedy-mesher-rust

Rust demo + tools for **binary greedy meshing** and simple voxel terrain generation.

This repo contains:
- A real-time OpenGL viewer that loads a pre-generated voxel level file and renders it using a greedy mesher.
- A procedural map generation viewer (`mapgen_view`) driven by TOML configs.
- PowerShell helper scripts under `scripts/` for creating/pushing a GitHub repo.

## Requirements

- Rust toolchain (edition 2024)
- A GPU/driver that supports modern OpenGL (the shaders use `#version 460 core`)

On Windows, make sure your graphics driver is up to date.

## Quick start

### Run the main demo (loads `levels/demo_terrain_96`)

```bash
cargo run --bin binary_greedy_mesher_demo_rs
```

The demo looks for the level file in these locations (first match wins):
- `./levels/demo_terrain_96`
- `../levels/demo_terrain_96` (legacy layout)
- `$PWD/levels/demo_terrain_96`

Level files included:
- `levels/demo_terrain_64`
- `levels/demo_terrain_96`

### Run the procedural mapgen viewer

```bash
cargo run --bin mapgen_view
```

Optional: choose a config file:

```bash
cargo run --bin mapgen_view -- --config mapgen_configs/mountains.toml
```

## Repo structure

- `src/mesher.rs`: greedy meshing implementation
- `src/data/`: level file parsing + RLE utilities
- `src/rendering/`: chunk renderer
- `src/mapgen/`: procedural generation (noise, generators, config)
- `mapgen_configs/`: example mapgen config presets
- `levels/`: demo level files
- `scripts/`: PowerShell scripts for GitHub automation

## GitHub scripts (PowerShell)

These scripts use a GitHub token from a local `.env` file (ignored by git).

1. Create `.env` in the repo root with one of:

```text
GITHUB_TOKEN=YOUR_TOKEN_HERE
# or
GITHUB_API_KEY=YOUR_TOKEN_HERE
# or
GITHUB_CLASSIC_TOKEN=YOUR_TOKEN_HERE
```

2. Create a new repo + push:

```powershell
./scripts/git-create-repo-and-push.ps1 -RepoName "binary-greedy-mesher-rust" -ForceSetRemote
```

## Large files note

GitHub warned that `levels/demo_terrain_96` is ~76MB (above GitHub’s *recommended* 50MB limit). It currently pushes fine, but if you plan to add more large assets, consider Git LFS.

## Known warnings

- `winit` API deprecation warning is currently emitted during build (non-fatal).
