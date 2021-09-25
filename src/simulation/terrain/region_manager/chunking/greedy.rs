use crate::geometry::{add_cube_geometry, add_floor_geometry};
use crate::simulation::{idxmap, REGION_WIDTH};
use std::collections::HashSet;

pub type CubeMap = HashSet<usize>;

pub fn greedy_cubes(
    cube_index: &mut CubeMap,
    vertices: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uv: &mut Vec<[f32; 2]>,
    tangents: &mut Vec<[f32; 3]>,
) {
    loop {
        let min_iter = cube_index.iter().min();
        if min_iter.is_none() {
            break;
        } else {
            let idx = *min_iter.unwrap();
            cube_index.remove(&idx);

            let (x, y, z) = idxmap(idx);
            let width = grow_right(cube_index, idx);
            let height = grow_down(cube_index, idx, width);
            let depth = 1;

            add_cube_geometry(
                vertices,
                normals,
                uv,
                tangents,
                x as f32,
                y as f32,
                z as f32,
                width as f32,
                height as f32,
                depth as f32,
            );
        }
    }
}

pub fn greedy_floors(
    cube_index: &mut CubeMap,
    vertices: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uv: &mut Vec<[f32; 2]>,
    tangents: &mut Vec<[f32; 3]>,
) {
    loop {
        let min_iter = cube_index.iter().min();
        if min_iter.is_none() {
            break;
        } else {
            let idx = *min_iter.unwrap();
            cube_index.remove(&idx);

            let (x, y, z) = idxmap(idx);
            let width = grow_right(cube_index, idx);
            let height = grow_down(cube_index, idx, width);

            add_floor_geometry(
                vertices,
                normals,
                uv,
                tangents,
                x as f32,
                y as f32,
                z as f32,
                width as f32,
                height as f32,
            );
        }
    }
}

fn grow_right(cube_index: &mut CubeMap, idx: usize) -> usize {
    let mut width = 1;
    let mut candidate_idx = idx + 1;

    while cube_index.contains(&candidate_idx) {
        cube_index.remove(&candidate_idx);
        width += 1;
        candidate_idx += 1;
    }

    width
}

fn grow_down(cube_index: &mut CubeMap, idx: usize, width: usize) -> usize {
    let mut height = 1;
    let mut candidate_idx = idx + REGION_WIDTH;
    'outer: loop {
        for cidx in candidate_idx..candidate_idx + width {
            if !cube_index.contains(&cidx) {
                break 'outer;
            }
        }

        for cidx in candidate_idx..candidate_idx + width {
            cube_index.remove(&cidx);
        }
        height += 1;
        candidate_idx += REGION_WIDTH;
    }
    height
}