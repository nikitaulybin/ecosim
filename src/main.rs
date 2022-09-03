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

const NOISE_MAP_SCALE: f64 = 4.9;

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
        .add_startup_system(camera_init)
        .add_startup_system(load_spritesheets)
        // .add_system(render_map)
        // .add_system(render_trees.after(render_map))
        .add_startup_system(render_noise_map)
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

fn render_trees(mut commands: Commands, sprite_sheets: Res<SpriteSheets>) {
    let mut rng = thread_rng();

    let img_size_ratio: f32 = 30.0 / 53.0;
    let tree_dimensions: Vec2 = Vec2::new(15.0 * img_size_ratio, 15.0);
    commands.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            custom_size: Some(tree_dimensions),
            index: rng.gen_range(0..5),
            ..default()
        },
        texture_atlas: sprite_sheets.trees.clone(),
        transform: Transform {
            scale: Vec3::new(1.0, 1.0, 1.0),
            translation: Vec3::new(100.0, 100.0, 0.0),
            ..default()
        },
        ..default()
    });
}

fn render_noise_map(mut commands: Commands) {
    let noise_map = generate_noise_map(MAP_WIDTH, MAP_HEIGHT, NOISE_MAP_SCALE);

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
                    translation: Vec3::new(x as f32 * TILE_SIZE as f32, y as f32 * TILE_SIZE as f32, 0.0),
                    ..default()
                },
                ..default()
            });
        }
    }
}
