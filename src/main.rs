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
use bevy::{sprite::Anchor, window::PresentMode};
use prelude::*;

const NOISE_MAP_SCALE: f64 = 10.0;
const NOISE_MAP_OCTAVES: usize = 4;
const NOISE_MAP_PERSISTENCE: f64 = 0.5;
const NOISE_MAP_LACUNARITY: f64 = 2.0;

pub struct SpriteSheets {
    trees: Handle<TextureAtlas>,
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
        .add_startup_system(render_map)
        .add_startup_system(render_trees)
        .add_system(mouse_button_input)
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
fn mouse_button_input(buttons: Res<Input<MouseButton>>, mut map: ResMut<Map>, mut commands: Commands, query: Query<&TextureAtlasSprite>, tree_query: Query<(&Tree, &Pos)>) {
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
        
        } )
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
    let sprite_sheet_handle: Handle<Image> = asset_server.load("tree_sprites.png");

    let texture_atlas = TextureAtlas::from_grid(sprite_sheet_handle, Vec2::new(30.0, 53.0), 5, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(SpriteSheets {
        trees: texture_atlas_handle,
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
