use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use anyhow::{Context, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoiseLayer {
    pub scale: f32,
    pub octaves: u32,
    pub lacunarity: f32,
    pub gain: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapGenConfig {
    pub name: String,
    pub description: String,
    pub seed: u32,
    
    pub continentalness: NoiseLayer,
    pub erosion: NoiseLayer,
    pub peaks_valleys: NoiseLayer,
    
    pub weights: Weights,
    pub vertical: VerticalConfig,
    pub material_thresholds: MaterialThresholds,
    pub density_threshold: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weights {
    pub continentalness: f32,
    pub peaks_valleys: f32,
    pub erosion: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerticalConfig {
    pub bias: f32,
    pub min_y: f32,
    pub max_y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialThresholds {
    pub stone_max: i32,
    pub grass_max: i32,
}

impl MapGenConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = fs::read_to_string(path.as_ref())
            .with_context(|| format!("Failed to read config file: {:?}", path.as_ref()))?;
        
        let config: MapGenConfig = toml::from_str(&contents)
            .with_context(|| format!("Failed to parse config file: {:?}", path.as_ref()))?;
        
        Ok(config)
    }
    
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let contents = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;
        
        fs::write(path.as_ref(), contents)
            .with_context(|| format!("Failed to write config file: {:?}", path.as_ref()))?;
        
        Ok(())
    }
}

impl Default for MapGenConfig {
    fn default() -> Self {
        Self {
            name: "Default Multi-Noise".to_string(),
            description: "Default stacked noise terrain generation".to_string(),
            seed: 42,
            
            continentalness: NoiseLayer {
                scale: 3.0,
                octaves: 5,
                lacunarity: 2.0,
                gain: 0.5,
            },
            
            erosion: NoiseLayer {
                scale: 8.0,
                octaves: 4,
                lacunarity: 2.0,
                gain: 0.5,
            },
            
            peaks_valleys: NoiseLayer {
                scale: 18.0,
                octaves: 5,
                lacunarity: 2.0,
                gain: 0.5,
            },
            
            weights: Weights {
                continentalness: 0.9,
                peaks_valleys: 1.0,
                erosion: 0.35,
            },
            
            vertical: VerticalConfig {
                bias: 1.8,
                min_y: -128.0,
                max_y: 128.0,
            },
            
            material_thresholds: MaterialThresholds {
                stone_max: 32,
                grass_max: 64,
            },
            
            density_threshold: -0.2,
        }
    }
}
