use std::fmt::format;

use bevy::{
    prelude::{Color, Vec2},
    sprite::Rect,
    utils::HashMap,
};
use rand::{thread_rng, Rng};

pub const MAP_WIDTH: usize = 200;
pub const MAP_HEIGHT: usize = 100;

const LAKE_GEN_ITERATIONS: usize = 25000;
pub const LAKE_COUNT: usize = 5;
pub const TILE_SIZE: usize = 4;

// Trees generation
const TREE_NOISE_MAP_SCALE: f64 = 10.0;
const TREE_SPAWN_NOISE_TRESHOLD: f64 = 0.30;

const MAP_CHUNK_SIZE: f32 = 20.0;
use crate::{
    prelude::*, NOISE_MAP_LACUNARITY, NOISE_MAP_OCTAVES, NOISE_MAP_PERSISTENCE, NOISE_MAP_SCALE,
};

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    LAND,
    WATER,
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub tile_type: TileType,
    pub tree_noise_value: f64,
}

impl Tile {
    pub fn get_color(&self) -> Color {
        match self.tile_type {
            TileType::LAND => Color::rgb(0.0, 1.0, 0.0),
            TileType::WATER => Color::rgb(0.0, 0.0, 1.0),
        }
    }
}
pub struct Map {
    pub tiles: Vec<Tile>,
    pub tree_positions: Vec<Vec2>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            tiles: vec![
                Tile {
                    tile_type: TileType::LAND,
                    tree_noise_value: 0.0,
                };
                MAP_WIDTH as usize * MAP_HEIGHT as usize
            ],
            tree_positions: vec![],
        }
    }

    pub fn fill(&mut self, tile_type: TileType) {
        self.tiles.iter_mut().for_each(|t| t.tile_type = tile_type);
    }

    fn size(&self) -> usize {
        MAP_WIDTH * MAP_HEIGHT
    }

    pub fn generate_lake(&mut self) {
        let mut rng = thread_rng();
        let river_start = rng.gen_range(0..self.size());

        let mut river_tiles = vec![river_start];
        let mut current_tile = idx_to_vec2(river_start as i32);
        for _j in 0..LAKE_GEN_ITERATIONS {
            let next_tile_diff = match rng.gen_range(0..4) {
                0 => Vec2::new(1.0, 0.0),
                1 => Vec2::new(0.0, 1.0),
                2 => Vec2::new(-1.0, 0.0),
                3 => Vec2::new(0.0, -1.0),
                _ => Vec2::new(0.0, 0.0),
            };
            let next_tile = current_tile + next_tile_diff;
            if self.in_bounds(next_tile) {
                river_tiles.push(vec2_to_idx(next_tile) as usize);
                current_tile = next_tile;
            }
        }

        for tile_idx in river_tiles {
            self.tiles[tile_idx as usize].tile_type = TileType::WATER;
        }

    }

    // For debugging - remove later
    fn tile_count_by_type(&self, tile_type: TileType) -> i32 {
        let mut count = 0;
        for tile in self.tiles.iter() {
            if tile.tile_type == tile_type {
                count += 1;
            }
        }
        count
    }

    fn in_bounds(&self, point: Vec2) -> bool {
        point.x < (MAP_WIDTH * TILE_SIZE) as f32
            && point.x >= 0.0
            && point.y < (MAP_HEIGHT * TILE_SIZE) as f32
            && point.y >= 0.0
    }

    pub fn spawn_trees(&mut self) {
        let noise_map = generate_noise_map(
            MAP_WIDTH,
            MAP_HEIGHT,
            NOISE_MAP_SCALE,
            NOISE_MAP_OCTAVES,
            NOISE_MAP_PERSISTENCE,
            NOISE_MAP_LACUNARITY,
        );
        let chunks = self.get_chunks();

        // for c in chunks.iter() {
        //     let mut average_noise_value = 0.0;
        //     for y in c.min.y as usize..c.max.y as usize {
        //         for x in c.min.x as usize..c.max.x as usize {
        //             average_noise_value += noise_map[y][x];
        //         }
        //     }
        //     average_noise_value /= (c.max.x as f64 - c.min.x as f64) * (c.max.y as f64 - c.min.y as f64);
        //
        //     println!("{}", average_noise_value);
        // }

        let mut tile_tree_map: HashMap<String, bool> = HashMap::new();

        for y in 0..noise_map.len() {
            for x in 0..noise_map[y].len() {
                let noise_value = noise_map[y][x];
                let mut tile = self.tiles[vec2_to_idx(Vec2::new((x * TILE_SIZE)as f32, (y * TILE_SIZE )as f32))];
                tile.tree_noise_value = noise_value;

                let should_spawn = noise_value < TREE_SPAWN_NOISE_TRESHOLD && tile.tile_type == TileType::LAND;
                if should_spawn {
                    let key = format!("{}-{}", x as i32, y as i32);

                    if tile.tile_type == TileType::WATER {
                        println!("Spawning tree in water");
                    }

                    if let None = tile_tree_map.get(&key) {
                        self.tree_positions.push(Vec2::new(x as f32, y as f32));
                        tile_tree_map.insert(key, true);
                    }
                }
            }
        }
    }

    // Break into chunks 20 x 20 DONE
    //
    // Get average noise value per chunk
    //
    // Based on the value, calculate the gap between trees, the greater the value - the farther the
    // tress are from one another
    //
    // On each chunk, spawn tree at the 0x0 relative to chunk, add gap vector to tree_pos vector
    // and spawn another one
    //
    // Do this for each chunk
    //
    // Update: found a different solution, will keep it for now in case I change my mind

    fn get_chunks(&self) -> Vec<Rect> {
        let mut current_chunk_origin = Vec2::new(0.0, 0.0);
        let mut chunks = vec![];

        loop {
            let chunk_dimensions = Vec2::new(
                f32::min(MAP_WIDTH as f32 - current_chunk_origin.x, MAP_CHUNK_SIZE),
                f32::min(MAP_HEIGHT as f32 - current_chunk_origin.y, MAP_CHUNK_SIZE),
            );
            let chunk = Rect {
                min: current_chunk_origin,
                max: current_chunk_origin + chunk_dimensions,
            };
            let chunk = Rect {
                min: current_chunk_origin,
                max: current_chunk_origin + chunk_dimensions,
            };
            chunks.push(chunk);

            current_chunk_origin.x += chunk_dimensions.x;
            if current_chunk_origin.x >= MAP_WIDTH as f32 {
                current_chunk_origin.x = 0.0;
                current_chunk_origin.y += chunk_dimensions.y;
                if current_chunk_origin.y >= MAP_HEIGHT as f32 {
                    break;
                }
            }
        }
        for (idx, chunk) in chunks.iter().enumerate() {
            println!(
                "Chunk {}: Rect({}, {}, {}, {})",
                idx,
                chunk.min.x,
                chunk.min.y,
                chunk.max.x - chunk.min.x,
                chunk.max.y - chunk.min.y
            );
        }
        chunks
    }
}

pub fn idx_to_vec2(idx: i32) -> Vec2 {
    Vec2 {
        x: (idx % MAP_WIDTH as i32) as f32 * TILE_SIZE as f32,
        y: (idx / MAP_WIDTH as i32) as f32 * TILE_SIZE as f32,
    }
}

pub fn vec2_to_idx(point: Vec2) -> usize {
    let scaled_point = point * 1.0 / TILE_SIZE as f32;
    scaled_point.y as usize * MAP_WIDTH as usize + scaled_point.x as usize
}
