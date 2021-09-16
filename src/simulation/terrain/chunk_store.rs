use super::region_chunk::RegionChunk;
use super::region_chunk_state::ChunkStatus;
use super::PLANET_STORE;
use super::{strata::StrataMaterials, GameCamera};
use crate::simulation::{planet_idx, Planet, WORLD_WIDTH};
use bevy::prelude::*;
use lazy_static::*;
use parking_lot::RwLock;
use std::collections::HashMap;

lazy_static! {
    pub static ref CHUNK_STORE: RwLock<ChunkStore> = RwLock::new(ChunkStore::new());
}

pub struct ChunkStore {
    regions: HashMap<usize, RegionChunk>,
}

impl ChunkStore {
    pub fn new() -> Self {
        Self {
            regions: HashMap::new(),
        }
    }

    /// Call this once after raws have loaded
    pub fn verify_strata(&mut self) {
        PLANET_STORE.write().strata = Some(StrataMaterials::read());
    }

    /// Call this once a planet is loaded for the renderer to use.
    pub fn set_planet(&mut self, planet: Planet) {
        let height_noise = planet.get_height_noise();
        let mat_noise = planet.get_material_noise();
        PLANET_STORE.write().planet = Some(planet);
        PLANET_STORE.write().height_noise = Some(height_noise);
        PLANET_STORE.write().material_noise = Some(mat_noise);
    }

    /// Starts creating a playable region. The region will be marked to
    /// remain in memory - only swapping meshes in/out. Designed to run
    /// asynchronously, inside a write lock. It aims to bail-out fast,
    /// leaving the loading process running in the background. That integrates
    /// with the game camera, activating region chunks as they are ready.
    pub fn with_playable_region(&mut self, tile_x: usize, tile_y: usize) {
        let region_idx = planet_idx(tile_x, tile_y);
        if let Some(region) = self.regions.get_mut(&region_idx) {
            // The region exists, just need to initialize it
            region.chunks.iter_mut().for_each(|c| {
                c.required = true;
                if c.status == ChunkStatus::Expired {
                    c.status = ChunkStatus::NotLoaded;
                }
            });
            region.required = true;
        } else {
            // New region
            let mut rc = RegionChunk::new(tile_x, tile_y);
            rc.required = true;
            rc.chunks.iter_mut().for_each(|c| c.required = true);
            self.regions.insert(region_idx, rc);
        }
    }

    pub fn manage_for_camera(
        &mut self,
        camera: &GameCamera,
        mesh_assets: &mut ResMut<Assets<Mesh>>,
        commands: &mut Commands,
    ) {
        use std::collections::HashSet;

        let mut active_regions = HashSet::new();
        active_regions.insert(planet_idx(camera.tile_x, camera.tile_y));
        // Make this optional - load neighboring regions for context
        active_regions.insert(planet_idx(camera.tile_x - 1, camera.tile_y));
        active_regions.insert(planet_idx(camera.tile_x + 1, camera.tile_y));
        active_regions.insert(planet_idx(camera.tile_x, camera.tile_y + 1));
        active_regions.insert(planet_idx(camera.tile_x, camera.tile_y - 1));
        active_regions.insert(planet_idx(camera.tile_x - 1, camera.tile_y - 1));
        active_regions.insert(planet_idx(camera.tile_x + 1, camera.tile_y - 1));
        active_regions.insert(planet_idx(camera.tile_x - 1, camera.tile_y + 1));
        active_regions.insert(planet_idx(camera.tile_x + 1, camera.tile_y + 1));

        for pidx in active_regions.iter() {
            if let Some(r) = self.regions.get_mut(pidx) {
                //println!("Found active region: {}", pidx);
                r.distance_activate(camera, mesh_assets, commands);
            } else {
                //println!("Must activate new region: {}", pidx);
                let mut activate = RegionChunk::new(pidx % WORLD_WIDTH, pidx / WORLD_WIDTH);
                activate.distance_activate(camera, mesh_assets, commands);
                self.regions.insert(*pidx, activate);
            }
        }

        let mut to_destroy = HashSet::new();
        self.regions.iter().for_each(|(pidx, r)| {
            if !r.required && !active_regions.contains(pidx) {
                //println!("Can deactivate {}", pidx);
                to_destroy.insert(*pidx);
            }
        });
        to_destroy.iter().for_each(|pidx| {
            self.regions.remove(pidx);
        });
    }
}
