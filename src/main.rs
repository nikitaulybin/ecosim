mod components;
mod map;
mod noise_map_gen;
mod graphics;

mod prelude {
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::noise_map_gen::*;
    pub use bevy::prelude::*;
    pub use rand::{thread_rng, Rng};
    pub use crate::graphics::*;
}

use std::hash::Hash;

use bevy::{sprite::Anchor, utils::HashMap, window::PresentMode};
use prelude::*;

const NOISE_MAP_SCALE: f64 = 10.0;
const NOISE_MAP_OCTAVES: usize = 4;
const NOISE_MAP_PERSISTENCE: f64 = 0.5;
const NOISE_MAP_LACUNARITY: f64 = 2.0;


fn main() {
    let mut map = Map::new();
    for _i in 0..LAKE_COUNT {
        map.generate_lake();
    }
    map.spawn_trees();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GraphicsPlugin)
        .insert_resource(map)
        .insert_resource(WindowDescriptor {
            title: "Ecosystem sim".to_string(),
            width: 1280.0,
            height: 640.0,
            present_mode: PresentMode::AutoVsync,
            ..default()
        })
        .add_startup_system_to_stage(StartupStage::PreStartup, camera_init)
        .add_startup_system_to_stage(StartupStage::PreStartup, spawn_entities)
        .add_startup_system_to_stage(StartupStage::PreStartup, spawn_initial_animals)
        .add_system(mouse_button_input)
        // .add_startup_system(render_noise_map)
        .run();
}


fn mouse_button_input(
    buttons: Res<Input<MouseButton>>,
    mut map: ResMut<Map>,
    mut commands: Commands,
    query: Query<&TextureAtlasSprite>,
    tree_query: Query<(&Tree, &Pos)>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        for (idx, tile) in map.tiles.iter().enumerate() {
            let tile_pos = idx_to_vec2(idx as i32);
            if tile.tile_type == TileType::WATER {
                commands.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::BLACK,
                        custom_size: Some(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(tile_pos.x, tile_pos.y, 0.0),
                        ..default()
                    },
                    ..default()
                });
            }
        }
        println!("Number of sprites: {}", query.iter().len());
    }

    if buttons.just_pressed(MouseButton::Right) {
        tree_query.iter().for_each(|(_, pos)| {
            let idx = vec2_to_idx(pos.0);
            map.tiles[idx].tile_type = TileType::WATER;
        })
    }
}

fn camera_init(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(
                MAP_WIDTH as f32 * (TILE_SIZE as f32 / 2.0),
                MAP_HEIGHT as f32 * (TILE_SIZE as f32 / 2.0),
                900.0,
            ),
            scale: Vec3::new(0.5, 0.5, 1.0),
            ..default()
        },
        ..default()
    });
}

fn spawn_entities(mut commands: Commands, map: Res<Map>) {
    let mut rng = thread_rng();
    for tree in map.tree_positions.iter() {
        commands.spawn_bundle((Tree, Pos(*tree), RelativeTextureIndex(rng.gen_range(0..5))));
    }
}

fn spawn_initial_animals(mut commands: Commands) {
    commands.spawn_bundle((
        Animal, 
        Pos(Vec2::new((MAP_WIDTH as f32 / 2.0) * TILE_SIZE as f32, (MAP_HEIGHT as f32 / 2.0) * TILE_SIZE as f32)),
        AnimalType::Bunny,
        AnimalState::Moving,
        AnimalDirection::Left,
    ));
}

fn render_noise_map(mut commands: Commands) {
    let noise_map = generate_noise_map(
        MAP_WIDTH,
        MAP_HEIGHT,
        NOISE_MAP_SCALE,
        NOISE_MAP_OCTAVES,
        NOISE_MAP_PERSISTENCE,
        NOISE_MAP_LACUNARITY,
    );

    for y in 0..noise_map.len() {
        for x in 0..noise_map[y].len() {
            let noise_value = noise_map[y][x] as f32;
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::Rgba {
                        red: noise_value,
                        green: noise_value,
                        blue: noise_value,
                        alpha: 1.0,
                    },
                    custom_size: Some(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        x as f32 * TILE_SIZE as f32,
                        y as f32 * TILE_SIZE as f32,
                        0.0,
                    ),
                    ..default()
                },
                ..default()
            });
        }
    }
}
