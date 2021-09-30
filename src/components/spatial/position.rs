use crate::simulation::{REGION_HEIGHT, REGION_WIDTH};
use bevy::prelude::Vec3;
use super::PlanetLocation;

/// Represents a location in the world
pub struct Position {
    pub region: PlanetLocation,
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Position {
    /// Create with a specific region identifier and tile coordinates
    pub fn new(region: PlanetLocation, x: usize, y: usize, z: usize) -> Self {
        Self { region, x, y, z }
    }

    /// Convert to a region tile index
    pub fn to_tile_index(&self) -> usize {
        (self.z * REGION_HEIGHT as usize * REGION_WIDTH as usize)
            + (self.y * REGION_WIDTH as usize)
            + self.x
    }

    /// Convert to render-space world coordinates
    pub fn to_world(&self) -> Vec3 {
        let (x, y, z) = self.region.to_world();
        Vec3::new(x + self.x as f32, y + self.y as f32, z + self.z as f32)
    }
}
