mod components;
mod map;
mod noise_map_gen;

mod prelude {
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::noise_map_gen::*;
    pub use bevy::prelude::*;
    pub use rand::{thread_rng, Rng};
}

use std::hash::Hash;

use bevy::{sprite::Anchor, utils::HashMap, window::PresentMode};
use prelude::*;

const NOISE_MAP_SCALE: f64 = 10.0;
const NOISE_MAP_OCTAVES: usize = 4;
const NOISE_MAP_PERSISTENCE: f64 = 0.5;
const NOISE_MAP_LACUNARITY: f64 = 2.0;

pub struct SpriteSheets {
    trees: Handle<TextureAtlas>,
    bunny: HashMap<AnimalState, HashMap<AnimalDirection, Handle<TextureAtlas>>>, // bunny[state][direction]
}

fn main() {
    let mut map = Map::new();
    for _i in 0..LAKE_COUNT {
        map.generate_lake();
    }
    map.spawn_trees();

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(map)
        .insert_resource(WindowDescriptor {
            title: "Ecosystem sim".to_string(),
            width: 1280.0,
            height: 640.0,
            present_mode: PresentMode::AutoVsync,
            ..default()
        })
        .add_startup_system_to_stage(StartupStage::PreStartup, camera_init)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_spritesheets)
        .add_startup_system_to_stage(StartupStage::PreStartup, spawn_entities)
        .add_startup_system_to_stage(StartupStage::PreStartup, spawn_initial_animals)
        .add_startup_system(render_map)
        .add_startup_system(render_trees)
        .add_system(mouse_button_input)
        .add_system(render_animals)
        // .add_startup_system(render_noise_map)
        .run();
}

pub fn render_map(map: Res<Map>, mut commands: Commands) {
    for (idx, tile) in map.tiles.iter().enumerate() {
        let tile_pos = idx_to_vec2(idx as i32);
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: tile.get_color(),
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

fn tile_checker(windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();

    if let Some(pos) = window.cursor_position() {
        println!("x: {}, y: {}", pos.x, pos.y);
    }
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
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..default()
        },
        ..default()
    });
}
fn load_spritesheets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tree_sprite_sheet_handle: Handle<Image> = asset_server.load("tree_sprites.png");
    let bunny_sprite_sheet_handle: Handle<Image> = asset_server.load("bunnysheet.png");

    let tree_texture_atlas =
        TextureAtlas::from_grid(tree_sprite_sheet_handle, Vec2::new(30.0, 53.0), 5, 1);
    let tree_texture_atlas_handle = texture_atlases.add(tree_texture_atlas);

    let bunny_down_texture_atlas = TextureAtlas::from_grid_with_padding(
        bunny_sprite_sheet_handle.clone(),
        Vec2::new(19.0, 29.0),
        5,
        1,
        Vec2::ZERO,
        Vec2::ZERO,
    );
    let bunny_up_texture_atlas = TextureAtlas::from_grid_with_padding(
        bunny_sprite_sheet_handle.clone(),
        Vec2::new(19.0, 35.0),
        5,
        1,
        Vec2::ZERO,
        Vec2::new(0.0, 29.0),
    );
    let bunny_right_texture_atlas = TextureAtlas::from_grid_with_padding(
        bunny_sprite_sheet_handle.clone(),
        Vec2::new(33.0, 28.0),
        5,
        1,
        Vec2::ZERO,
        Vec2::new(0.0, 64.0),
    );
    let bunny_left_texture_atlas = TextureAtlas::from_grid_with_padding(
        bunny_sprite_sheet_handle.clone(),
        Vec2::new(33.0, 28.0),
        5,
        1,
        Vec2::ZERO,
        Vec2::new(0.0, 92.0),
    );

    let bunny_down_eat_texture_atlas = TextureAtlas::from_grid_with_padding(
        bunny_sprite_sheet_handle.clone(),
        Vec2::new(19.0, 29.0),
        3,
        1,
        Vec2::ZERO,
        Vec2::new(95.0, 0.0),
    );
    let bunny_up_eat_texture_atlas = TextureAtlas::from_grid_with_padding(
        bunny_sprite_sheet_handle.clone(),
        Vec2::new(19.0, 35.0),
        3,
        1,
        Vec2::ZERO,
        Vec2::new(95.0, 29.0),
    );
    let bunny_right_eat_texture_atlas = TextureAtlas::from_grid_with_padding(
        bunny_sprite_sheet_handle.clone(),
        Vec2::new(28.0, 22.0),
        3,
        1,
        Vec2::ZERO,
        Vec2::new(165.0, 64.0),
    );
    let bunny_left_eat_texture_atals = TextureAtlas::from_grid_with_padding(
        bunny_sprite_sheet_handle.clone(),
        Vec2::new(28.0, 22.0),
        3,
        1,
        Vec2::ZERO,
        Vec2::new(165.0, 92.0),
    );

    let bunny_down_idle_texture_atlas = TextureAtlas::from_grid_with_padding(
        bunny_sprite_sheet_handle.clone(),
        Vec2::new(19.0, 26.0),
        1,
        1,
        Vec2::ZERO,
        Vec2::new(0.0, 128.0),
    );
    let bunny_up_idle_texture_atlas = TextureAtlas::from_grid_with_padding(
        bunny_sprite_sheet_handle.clone(),
        Vec2::new(20.0, 28.0),
        1,
        1,
        Vec2::ZERO,
        Vec2::new(19.0, 128.0),
    );
    let bunny_left_idle_texture_atlas = TextureAtlas::from_grid_with_padding(
        bunny_sprite_sheet_handle.clone(),
        Vec2::new(25.0, 27.0),
        1,
        1,
        Vec2::ZERO,
        Vec2::new(38.0, 128.0),
    );
    let bunny_right_idle_texture_atlas = TextureAtlas::from_grid_with_padding(
        bunny_sprite_sheet_handle.clone(),
        Vec2::new(25.0, 27.0),
        1,
        1,
        Vec2::ZERO,
        Vec2::new(63.0, 128.0),
    );

    let bunny_down_textrue_atlas_handle = texture_atlases.add(bunny_down_texture_atlas);
    let bunny_up_texture_atlas_handle = texture_atlases.add(bunny_up_texture_atlas);
    let bunny_right_texture_atlas_handle = texture_atlases.add(bunny_right_texture_atlas);
    let bunny_left_texture_atlas_hanlde = texture_atlases.add(bunny_left_texture_atlas);

    let bunny_down_eat_texture_atlas_handle = texture_atlases.add(bunny_down_eat_texture_atlas);
    let bunny_up_eat_texture_atlas_handle = texture_atlases.add(bunny_up_eat_texture_atlas);
    let bunny_right_eat_texture_atlas_handle = texture_atlases.add(bunny_right_eat_texture_atlas);
    let bunny_left_eat_texture_atals_handle = texture_atlases.add(bunny_left_eat_texture_atals);

    let bunny_down_idle_texture_atlas_handle = texture_atlases.add(bunny_down_idle_texture_atlas);
    let bunny_up_idle_texture_atlas_handle = texture_atlases.add(bunny_up_idle_texture_atlas);
    let bunny_left_idle_texture_atlas_handle = texture_atlases.add(bunny_left_idle_texture_atlas);
    let bunny_right_idle_texture_atlas_handle = texture_atlases.add(bunny_right_idle_texture_atlas);

    let mut bunny_atlas_map = HashMap::new();

    let mut down_hop = HashMap::new();
    down_hop.insert(AnimalDirection::Down, bunny_down_textrue_atlas_handle);
    let mut up_hop = HashMap::new();
    up_hop.insert(AnimalDirection::Up, bunny_up_texture_atlas_handle);
    let mut left_hop = HashMap::new();
    left_hop.insert(AnimalDirection::Left, bunny_left_texture_atlas_hanlde);
    let mut right_hop = HashMap::new();
    right_hop.insert(AnimalDirection::Right, bunny_right_texture_atlas_handle);

    bunny_atlas_map.insert(AnimalState::Moving, down_hop);
    bunny_atlas_map.insert(AnimalState::Moving, up_hop);
    bunny_atlas_map.insert(AnimalState::Moving, left_hop);
    bunny_atlas_map.insert(AnimalState::Moving, right_hop);

    let mut down_eat = HashMap::new();
    down_eat.insert(AnimalDirection::Down, bunny_down_eat_texture_atlas_handle);
    let mut up_eat = HashMap::new();
    up_eat.insert(AnimalDirection::Up, bunny_up_eat_texture_atlas_handle);
    let mut left_eat = HashMap::new();
    left_eat.insert(AnimalDirection::Left, bunny_left_eat_texture_atals_handle);
    let mut right_eat = HashMap::new();
    right_eat.insert(AnimalDirection::Right, bunny_right_eat_texture_atlas_handle);

    bunny_atlas_map.insert(AnimalState::Eating, down_eat);
    bunny_atlas_map.insert(AnimalState::Eating, up_eat);
    bunny_atlas_map.insert(AnimalState::Eating, left_eat);
    bunny_atlas_map.insert(AnimalState::Eating, right_eat);

    let mut down_idle = HashMap::new();
    down_idle.insert(AnimalDirection::Down, bunny_down_idle_texture_atlas_handle);
    let mut up_idle = HashMap::new();
    up_idle.insert(AnimalDirection::Up, bunny_up_idle_texture_atlas_handle);
    let mut left_idle = HashMap::new();
    left_idle.insert(AnimalDirection::Left, bunny_left_idle_texture_atlas_handle);
    let mut right_idle = HashMap::new();
    right_idle.insert(
        AnimalDirection::Right,
        bunny_right_idle_texture_atlas_handle,
    );

    bunny_atlas_map.insert(AnimalState::Idle, down_idle);
    bunny_atlas_map.insert(AnimalState::Idle, up_idle);
    bunny_atlas_map.insert(AnimalState::Idle, left_idle);
    bunny_atlas_map.insert(AnimalState::Idle, right_idle);

    commands.insert_resource(SpriteSheets {
        trees: tree_texture_atlas_handle,
        bunny: bunny_atlas_map,
    });
    println!("Spritesheets are loaded!");
}

fn spawn_entities(mut commands: Commands, map: Res<Map>) {
    let mut rng = thread_rng();
    for tree in map.tree_positions.iter() {
        commands.spawn_bundle((Tree, Pos(*tree), RelativeTextureIndex(rng.gen_range(0..5))));
    }
}
fn render_trees(
    mut commands: Commands,
    sprite_sheets: Res<SpriteSheets>,
    tree_query: Query<(&Tree, &Pos, &RelativeTextureIndex)>,
) {
    let img_size_ratio: f32 = 30.0 / 53.0;
    // let tree_dimensions: Vec2 = Vec2::new(4.0, 4.0);
    let tree_dimensions: Vec2 = Vec2::new(15.0 * img_size_ratio, 15.0);
    tree_query.iter().for_each(|(_, pos, index)| {
        commands.spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(tree_dimensions),
                index: index.0,
                anchor: Anchor::BottomCenter,
                ..default()
            },
            texture_atlas: sprite_sheets.trees.clone(),
            transform: Transform {
                scale: Vec3::new(1.0, 1.0, 1.0),
                translation: Vec3::new(pos.0.x * TILE_SIZE as f32, pos.0.y * TILE_SIZE as f32, 0.0),
                ..default()
            },
            ..default()
        });
    });
}

fn render_animals(
    mut commands: Commands,
    atlases: Res<SpriteSheets>,
    query: Query<(
        Entity,
        &Animal,
        &Pos,
        &AnimalState,
        &AnimalDirection,
        &AnimalType,
    )>,
) {
    query
        .iter()
        .for_each(|(_, animal, pos, state, direction, animal_type)| {
            let animal_atlases = match animal_type {
                AnimalType::Bunny => &atlases.bunny,
            };
            let direction_map = animal_atlases.get(state).unwrap();
            let target_atlas = direction_map.get(direction).unwrap();
            commands.spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    index: animal.frame_index,
                    ..default()
                },
                texture_atlas: target_atlas.clone(),
                transform: Transform {
                    translation: Vec3::new(pos.0.x, pos.0.y, 0.0),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    ..default()
                },
                ..default()
            });
        })
}


fn spawn_initial_animals(mut commands: Commands) {
    commands.spawn_bundle((
        Animal { frame_index: 0 },
        Pos(Vec2::new(MAP_WIDTH as f32 / 2.0, MAP_HEIGHT as f32 / 2.0)),
        AnimalType::Bunny,
        AnimalState::Idle,
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
