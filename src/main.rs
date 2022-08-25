mod map;

mod prelude {
    pub use crate::map::*;
    pub use bevy::prelude::*;
}
use bevy::window::PresentMode;
use prelude::*;

fn main() {
    let mut map = Map::new();
    for _i in 0..LAKE_COUNT {
        map.generate_lake();
    }
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
        .add_system(render_map)
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
            translation: Vec3::new(MAP_WIDTH as f32 * 2.0, MAP_HEIGHT as f32 * 2.0, 900.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..default()
        },
        ..default()
    });
}
