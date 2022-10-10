mod components;
mod graphics;
mod animal_behavour;
mod map;
mod noise_map_gen;
mod pathfinder;

mod prelude {
    pub use crate::components::*;
    pub use crate::graphics::*;
    pub use crate::animal_behavour::*;
    pub use crate::map::*;
    pub use crate::noise_map_gen::*;
    pub use crate::pathfinder::*;
    pub use bevy::prelude::*;
    pub use rand::{thread_rng, Rng};
    pub use strum::IntoEnumIterator;
    pub use strum_macros::EnumIter;
}

use std::hash::Hash;

use bevy::{
    sprite::Anchor,
    utils::HashMap,
    window::{self, PresentMode},
};
use prelude::*;

const NOISE_MAP_SCALE: f64 = 10.0;
const NOISE_MAP_OCTAVES: usize = 4;
const NOISE_MAP_PERSISTENCE: f64 = 0.5;
const NOISE_MAP_LACUNARITY: f64 = 2.0;

fn main() {
    let test = Vec2::new(10.0, 15.0);
    let test1 = Vec2::new(10.0, 15.0);

    println!("{}", test == test1);
    let mut map = Map::new();
    for _i in 0..LAKE_COUNT {
        map.generate_lake();
    }
    map.spawn_trees();
    let pathfinder = Pathfinder::new(map.clone());

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GraphicsPlugin)
        .add_plugin(AnimalBehaviourPlugin)
        .insert_resource(map)
        .insert_resource(pathfinder)
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
    pathfinder: Res<Pathfinder>,
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    camera_query: Query<&Camera>,
    animal_query: Query<(Entity, &Animal, &Pos)>,
    mut ev_drawpath: EventWriter<DrawPathEvent>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let window = windows.get_primary_mut().unwrap();
        if let Some(mouse_pos) = window.cursor_position() {
            let camera = camera_query.get_single().unwrap();

            let viewport_size = camera.logical_viewport_size().unwrap();
            let map_offset = (viewport_size / 2.0)
                - (Vec2::new(
                    MAP_WIDTH as f32 * (TILE_SIZE as f32 / 2.0),
                    MAP_HEIGHT as f32 * (TILE_SIZE as f32 / 2.0),
                ) / Vec2::new(0.5, 0.5));
            let map_pos = ((mouse_pos - map_offset) * Vec2::new(0.5, 0.5));

            let map_idx = vec2_to_idx(map_pos);

            let (entity, _, pos) = animal_query.get_single().unwrap();

            let path = pathfinder.a_star(pos.0, map_pos);
            ev_drawpath.send(DrawPathEvent(Path(path.clone())));

            commands.entity(entity).insert(Path(path.clone()));
        }
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
        Pos(Vec2::new(
            (MAP_WIDTH as f32 / 2.0) * TILE_SIZE as f32,
            (MAP_HEIGHT as f32 / 2.0) * TILE_SIZE as f32,
        )),
        AnimalType::Bunny,
        AnimalState::Moving,
        AnimalDirection::Down,
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
