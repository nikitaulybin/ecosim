use bevy::prelude::{Color, Vec2};
use rand::{thread_rng, Rng};

pub const MAP_WIDTH: i32 = 200;
pub const MAP_HEIGHT: i32 = 100;

const LAKE_GEN_ITERATIONS: usize = 25000;
pub const LAKE_COUNT: usize = 5;
pub const TILE_SIZE: i32 = 4;

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    LAND,
    WATER,
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub tile_type: TileType,
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
}

impl Map {
    pub fn new() -> Self {
        Map {
            tiles: vec![
                Tile {
                    tile_type: TileType::LAND,
                };
                MAP_WIDTH as usize * MAP_HEIGHT as usize
            ],
        }
    }

    pub fn fill(&mut self, tile_type: TileType) {
        self.tiles.iter_mut().for_each(|t| t.tile_type = tile_type);
    }

    fn size(&self) -> i32 {
        MAP_WIDTH * MAP_HEIGHT
    }

    pub fn generate_lake(&mut self) {
        let mut rng = thread_rng();
        let river_start = rng.gen_range(0..self.size());

        println!(
            "Water count before: {}",
            self.tile_count_by_type(TileType::WATER)
        );
        let mut river_tiles = vec![river_start];
        let mut current_tile = idx_to_vec2(river_start as i32);
        println!("Start tile: {}, {}", current_tile.x, current_tile.y);
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
                river_tiles.push(vec2_to_idx(next_tile));
                current_tile = next_tile;

                println!("Current Tile: {}, {}", current_tile.x, current_tile.y);
            }
        }

        for tile_idx in river_tiles {
            self.tiles[tile_idx as usize].tile_type = TileType::WATER;
        }

        println!(
            "Water count after: {}",
            self.tile_count_by_type(TileType::WATER)
        );
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
        // println!("{}, {}", point.x, point.y);
        // println!(
        //     "{}",
        //     point.x < (MAP_WIDTH * TILE_SIZE) as f32
        //         && point.x >= 0.0
        //         && point.y < (MAP_HEIGHT * TILE_SIZE) as f32
        //         && point.y >= 0.0
        // );
        point.x < (MAP_WIDTH * TILE_SIZE) as f32
            && point.x >= 0.0
            && point.y < (MAP_HEIGHT * TILE_SIZE) as f32
            && point.y >= 0.0
    }
}

pub fn idx_to_vec2(idx: i32) -> Vec2 {
    Vec2 {
        x: (idx % MAP_WIDTH) as f32 * TILE_SIZE as f32,
        y: (idx / MAP_WIDTH) as f32 * TILE_SIZE as f32,
    }
}

pub fn vec2_to_idx(point: Vec2) -> i32 {
    let scaled_point = point * 1.0 / TILE_SIZE as f32;
    scaled_point.y as i32 * MAP_WIDTH + scaled_point.x as i32
}
