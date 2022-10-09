use crate::prelude::*;
use bevy::sprite::Anchor;
use bevy::utils::HashMap;

pub struct GraphicsPlugin;

const BUNNY_SIZE: f32 = 10.0;
const BUNNY_SIDE_HEIGHT_RATIO: f32 = 28.0 / 33.0;
const BUNNY_FRONTBACK_HEIGHT_RATIO: f32 = 19.0 / 29.0;

pub struct SpriteSheets {
    pub trees: Handle<TextureAtlas>,
    pub bunny: HashMap<AnimalState, HashMap<AnimalDirection, Handle<TextureAtlas>>>, // bunny[state][direction]
}

#[derive(Component)]
pub struct FrameAnimation {
    timer: Timer,
    current_frame: usize,
}

pub struct DrawPathEvent(pub Path);

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DrawPathEvent>()
            .add_startup_system_to_stage(StartupStage::PreStartup, Self::load_spritesheets)
            .add_startup_system_to_stage(StartupStage::Startup, Self::render_map)
            .add_startup_system_to_stage(StartupStage::Startup, Self::render_trees)
            .add_startup_system_to_stage(StartupStage::Startup, spawn_animal_sprites)
            .add_system(Self::frame_animation)
            .add_system(Self::draw_paths)
            .add_system(Self::update_sprite_positions);
    }
}

fn spawn_animal_sprites(
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
    // later, instead of reading from query we should be reading from some config file
    // query is just a dirty workaround here, cause I don't want to figure out how to create such
    // config file yet
    query
        .iter()
        .for_each(|(entity, animal, pos, state, direction, animal_type)| {
            let animal_atlases = match animal_type {
                AnimalType::Bunny => &atlases.bunny,
            };
            let sprite_size = match animal_type {
                AnimalType::Bunny => match direction {
                    AnimalDirection::Down | AnimalDirection::Up => {
                        Vec2::new(BUNNY_SIZE * BUNNY_FRONTBACK_HEIGHT_RATIO, BUNNY_SIZE)
                    }
                    AnimalDirection::Left | AnimalDirection::Right => {
                        Vec2::new(BUNNY_SIZE, BUNNY_SIZE * BUNNY_SIDE_HEIGHT_RATIO)
                    }
                },
            };

            let direction_map = animal_atlases.get(state).unwrap();
            let target_atlas = direction_map.get(direction).unwrap();
            commands
                .entity(entity)
                .insert_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        custom_size: Some(sprite_size),
                        index: 0,
                        ..default()
                    },
                    texture_atlas: target_atlas.clone(),
                    transform: Transform {
                        translation: Vec3::new(pos.0.x, pos.0.y, 0.0),
                        scale: Vec3::new(1.0, 1.0, 1.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(FrameAnimation {
                    timer: Timer::from_seconds(0.1, true),
                    current_frame: 0,
                });
        })
}
impl GraphicsPlugin {
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
            Vec2::new(165.0, 86.0),
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
        let bunny_right_eat_texture_atlas_handle =
            texture_atlases.add(bunny_right_eat_texture_atlas);
        let bunny_left_eat_texture_atals_handle = texture_atlases.add(bunny_left_eat_texture_atals);

        let bunny_down_idle_texture_atlas_handle =
            texture_atlases.add(bunny_down_idle_texture_atlas);
        let bunny_up_idle_texture_atlas_handle = texture_atlases.add(bunny_up_idle_texture_atlas);
        let bunny_left_idle_texture_atlas_handle =
            texture_atlases.add(bunny_left_idle_texture_atlas);
        let bunny_right_idle_texture_atlas_handle =
            texture_atlases.add(bunny_right_idle_texture_atlas);

        let mut bunny_atlas_map = HashMap::new();

        let mut hop_atlases = HashMap::new();
        hop_atlases.insert(AnimalDirection::Down, bunny_down_textrue_atlas_handle);
        hop_atlases.insert(AnimalDirection::Up, bunny_up_texture_atlas_handle);
        hop_atlases.insert(AnimalDirection::Left, bunny_left_texture_atlas_hanlde);
        hop_atlases.insert(AnimalDirection::Right, bunny_right_texture_atlas_handle);

        bunny_atlas_map.insert(AnimalState::Moving, hop_atlases);

        let mut eat_atlases = HashMap::new();
        eat_atlases.insert(AnimalDirection::Down, bunny_down_eat_texture_atlas_handle);
        eat_atlases.insert(AnimalDirection::Up, bunny_up_eat_texture_atlas_handle);
        eat_atlases.insert(AnimalDirection::Left, bunny_left_eat_texture_atals_handle);
        eat_atlases.insert(AnimalDirection::Right, bunny_right_eat_texture_atlas_handle);

        bunny_atlas_map.insert(AnimalState::Eating, eat_atlases);

        let mut idle_atlases = HashMap::new();
        idle_atlases.insert(AnimalDirection::Down, bunny_down_idle_texture_atlas_handle);
        idle_atlases.insert(AnimalDirection::Up, bunny_up_idle_texture_atlas_handle);
        idle_atlases.insert(AnimalDirection::Left, bunny_left_idle_texture_atlas_handle);
        idle_atlases.insert(
            AnimalDirection::Right,
            bunny_right_idle_texture_atlas_handle,
        );

        bunny_atlas_map.insert(AnimalState::Idle, idle_atlases);

        commands.insert_resource(SpriteSheets {
            trees: tree_texture_atlas_handle,
            bunny: bunny_atlas_map,
        });
        println!("Spritesheets are loaded!");
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
                    translation: Vec3::new(
                        pos.0.x * TILE_SIZE as f32,
                        pos.0.y * TILE_SIZE as f32,
                        0.0,
                    ),
                    ..default()
                },
                ..default()
            });
        });
    }

    fn frame_animation(
        mut sprites_query: Query<(
            &mut TextureAtlasSprite,
            &mut FrameAnimation,
            &mut Handle<TextureAtlas>,
            &AnimalType,
            &AnimalState,
            &AnimalDirection,
        )>,
        time: Res<Time>,
        atlases: Res<SpriteSheets>,
        assets: Res<Assets<TextureAtlas>>,
    ) {
        for (
            mut sprite,
            mut animation,
            mut texture_atlas_handle,
            animal_type,
            animal_state,
            animal_direction,
        ) in sprites_query.iter_mut()
        {
            let animal_atlases = match animal_type {
                AnimalType::Bunny => &atlases.bunny,
            };

            let direction_map = animal_atlases.get(animal_state).unwrap();
            let target_atlas = direction_map.get(animal_direction).unwrap();
            animation.timer.tick(time.delta());

            if animation.timer.just_finished() {
                if *texture_atlas_handle != *target_atlas {
                    *texture_atlas_handle = target_atlas.clone();
                    animation.current_frame = 0;
                    sprite.index = animation.current_frame;
                } else {
                    let texture_atlas = assets.get(&texture_atlas_handle).unwrap();
                    animation.current_frame = (animation.current_frame + 1) % texture_atlas.len();
                    sprite.index = animation.current_frame;
                }
            }
        }
    }

    fn draw_paths(mut ev_drawpath: EventReader<DrawPathEvent>, mut commands: Commands) {
        for ev in ev_drawpath.iter() {
            for node in ev.0 .0.iter() {
                commands.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::BLACK,
                        custom_size: Some(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(node.x, node.y, 0.0),
                        ..default()
                    },
                    ..default()
                });
            }
        }
    }

    fn update_sprite_positions(mut query: Query<(&Pos, &mut Transform)>) {
        for (pos, mut transform) in query.iter_mut() {
            transform.translation = Vec3::new(pos.0.x, pos.0.y, 0.0);
        }
    }
}
