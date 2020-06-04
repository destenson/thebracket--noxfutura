use super::{super::TileType, Region};
use crate::planet::{REGION_DEPTH, REGION_HEIGHT, REGION_WIDTH};
use crate::utils::mapidx;
use crate::raws::*;
use bracket_random::prelude::*;
use bracket_noise::prelude::*;

fn get_strata_indices(st: MaterialLayer) -> Vec<usize> {
    let mlock = crate::raws::RAWS.lock();
    mlock
        .materials
        .materials
        .iter()
        .enumerate()
        .filter(|(_,m)| { m.layer == st })
        .map(|(i, _)| i)
        .collect()
}

fn get_strata_materials() -> (Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>) {
    (
        get_strata_indices(MaterialLayer::Soil),
        get_strata_indices(MaterialLayer::Sand),
        get_strata_indices(MaterialLayer::Sedimentary),
        get_strata_indices(MaterialLayer::Igneous),
    )
}

pub struct Strata {
    pub map : Vec<usize>,
    pub material_idx : Vec<usize>,
    pub counts : Vec<(usize, usize, usize, usize)>
}

pub fn build_strata(rng: &mut RandomNumberGenerator, hm: &[u8], biome: &BiomeType, perlin_seed: u64) -> Strata {
    const REGION_TILES_COUNT : usize = REGION_WIDTH * REGION_HEIGHT * REGION_DEPTH;
    let (soils, sands, sedimentaries, igeneouses) = get_strata_materials();
    let n_strata = (REGION_WIDTH * REGION_HEIGHT) * 4 + rng.roll_dice(1, 64) as usize;
    let mut result = Strata {
        map : vec![1; REGION_TILES_COUNT],
        material_idx : vec![1; n_strata],
        counts: vec![(0,0,0,0); n_strata]
    };

    let mut biome_noise = FastNoise::seeded(perlin_seed);
    biome_noise.set_noise_type(NoiseType::SimplexFractal);
    biome_noise.set_fractal_type(FractalType::FBM);
    biome_noise.set_fractal_octaves(10);
    biome_noise.set_fractal_gain(0.4);
    biome_noise.set_fractal_lacunarity(2.0);
    biome_noise.set_frequency(0.01);
    for z in 0..REGION_DEPTH {
        let nz = (z as f32) / REGION_DEPTH as f32;
        for y in 0..REGION_WIDTH {
            let ny = (y as f32) / REGION_HEIGHT as f32;
            for x in 0..REGION_WIDTH {
                let nx = (x as f32) / REGION_WIDTH as f32;
                let cell_noise = biome_noise.get_noise3d(nx, ny, nz);
                let biome_ramp = (cell_noise + 1.0) / 2.0;
                let biome_idx = (biome_ramp * n_strata as f32) as usize;
                result.counts[biome_idx].0 += 1;
                result.counts[biome_idx].1 += x;
                result.counts[biome_idx].2 += y;
                result.counts[biome_idx].3 += z;
                let map_idx = mapidx(x, y, z);
                result.map[map_idx] = biome_idx;
            }
        }
    }    

    //let mut count_used = 0;
    for i in 0..n_strata {
        if result.counts[i].0 > 0 {
            //count_used += 1;
            result.counts[i].1 /= result.counts[i].0;
            result.counts[i].2 /= result.counts[i].0;
            result.counts[i].3 /= result.counts[i].0;

            let (_n, x, y, z) = result.counts[i];
            let altitude_at_center = hm[(y * REGION_WIDTH) + x] + REGION_DEPTH as u8/2;
            let mat_idx = if z as u8 > altitude_at_center - (1 + rng.roll_dice(1, 4) as u8) {
                // Soil
                let roll = rng.roll_dice(1, 100);
                if roll < biome.soils.soil {
                    rng.random_slice_entry(&soils)
                } else {
                    rng.random_slice_entry(&sands)
                }
            } else if z as u8 > ((altitude_at_center - 10) / 2) {
                // Sedimentary
                rng.random_slice_entry(&sedimentaries)
            } else {
                // Igneous
                rng.random_slice_entry(&igeneouses)
            };
            result.material_idx[i] = *mat_idx.unwrap();

        } else {
            result.material_idx[i] = *rng.random_slice_entry(&igeneouses).unwrap();
        }
    }

    result
}

pub fn layer_cake(hm: &[u8], region: &mut Region, strata: &Strata) {
    // Clear it
    region
        .tile_types
        .iter_mut()
        .for_each(|tt| *tt = TileType::Empty);

    // Build layered tiles
    //let x = 4;
    for x in 0..REGION_WIDTH {
        for y in 0..REGION_HEIGHT {
            let mut altitude = hm[(y * REGION_WIDTH) + x] as usize;
            if altitude > REGION_DEPTH-10 { altitude = REGION_DEPTH-1 };
            let mut wet = false;
            if altitude < 5 { wet = true; }

            // Bottom layer is always SMR
            region.tile_types[mapidx(x, y, 0)] = TileType::SemiMoltenRock;

            // Add lava above the bottom
            let mut z = 1;
            while z < altitude/3 {
                let cell_idx = mapidx(x, y, z);
                if x == 0 || x == REGION_WIDTH-1 || y == 0 || y == REGION_HEIGHT-1 {
                    region.tile_types[cell_idx] = TileType::SemiMoltenRock;
                } else {
                    region.tile_types[cell_idx] = TileType::Empty;
                    // Just add magma
                    region.material_idx[cell_idx] = 0;
                }
                z += 1;
            }

            // Next is rock until the soil layer
            while z < altitude {
                let cell_idx = mapidx(x, y, z);
                region.tile_types[cell_idx] = TileType::Solid;
                let mat_idx = strata.map[cell_idx];
                region.material_idx[cell_idx] = strata.material_idx[mat_idx];
                z += 1;
            }
        }
    }
}