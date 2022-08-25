use bevy::{prelude::{Vec2, Color}, utils::hashbrown::HashMap};

pub const MAP_WIDTH: i32 = 100;
pub const MAP_HEIGHT: i32 = 50;

pub const TILE_SIZE: i32 = 4;

#[derive(Clone, Copy)]
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
            tiles: vec![Tile {
                tile_type: TileType::LAND,
            }; MAP_WIDTH as usize * MAP_HEIGHT as usize],
        }
    }

    pub fn fill(&mut self, tile_type: TileType) {
        self.tiles.iter_mut().for_each(|t| t.tile_type = tile_type);
    }

}

pub fn idx_to_vec2(idx: i32) -> Vec2{
    Vec2 {
        x: (idx % MAP_WIDTH) as f32 * TILE_SIZE as f32,
        y: (idx / MAP_WIDTH) as f32 * TILE_SIZE as f32,
    }
}
