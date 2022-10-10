use bevy::utils::HashMap;

use crate::prelude::*;

// At some point each animal will have its own speed but for now this is good enough
const BUNNY_SPEED: f32 = 15.0;
const VELOCITY_REAPPLY_TILE_PROXIMITY: f32 = 6.0;
pub struct AnimalBehaviourPlugin;

impl Plugin for AnimalBehaviourPlugin {
    fn build(&self, app: &mut App) {
        let mut animal_direction_map: HashMap<AnimalDirection, Vec2> = HashMap::new();
        animal_direction_map.insert(AnimalDirection::Up, Vec2::new(0.0, 1.0));
        animal_direction_map.insert(AnimalDirection::Down, Vec2::new(0.0, -1.0));
        animal_direction_map.insert(AnimalDirection::Left, Vec2::new(-1.0, 0.0));
        animal_direction_map.insert(AnimalDirection::Right, Vec2::new(1.0, 0.0));

        app.add_event::<ApplyVelocityEvent>()
            .insert_resource(AnimalDirectionVectorMap(animal_direction_map))
            .add_system(Self::apply_initial_velocity)
            .add_system(Self::move_along_path)
            .add_system(Self::apply_velocity)
            .add_system(Self::move_animals)
            .add_system(Self::evaluate_animal_direction)
            .add_system(Self::evaluate_animal_state);
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

pub struct ApplyVelocityEvent {
    pub entity: Entity,
    pub pos: Vec2,
    pub destination: Vec2,
}

pub struct AnimalDirectionVectorMap(pub HashMap<AnimalDirection, Vec2>);

impl AnimalBehaviourPlugin {
    fn move_along_path(
        mut query: Query<(Entity, &Animal, &mut Path, &Pos)>,
        mut ev_apply_velocity: EventWriter<ApplyVelocityEvent>,
        mut commands: Commands,
    ) {
        for (entity, _, mut path, pos) in query.iter_mut() {
            if path.0.len() == 0 {
                commands.entity(entity).remove::<Path>();
                commands.entity(entity).remove::<Velocity>();
                continue;
            }

            let next_step_in_path = path.0[0];
            let distance_to_next = (pos.0 - next_step_in_path).length();
            let velocity_reapply_range_min = Vec2::new(
                next_step_in_path.x - VELOCITY_REAPPLY_TILE_PROXIMITY,
                next_step_in_path.y - VELOCITY_REAPPLY_TILE_PROXIMITY,
            );
            let velocity_reapply_range_max = Vec2::new(
                next_step_in_path.x + VELOCITY_REAPPLY_TILE_PROXIMITY,
                next_step_in_path.y + VELOCITY_REAPPLY_TILE_PROXIMITY,
            );
            println!("disatnce to next{}", distance_to_next);
            if pos.0.x >= velocity_reapply_range_min.x
                && pos.0.y >= velocity_reapply_range_min.y
                && pos.0.x <= velocity_reapply_range_max.x
                && pos.0.y <= velocity_reapply_range_max.y
            {
                println!("Reapplying velocity");
                // pos.0 = path.0.pop_front().unwrap();
                ev_apply_velocity.send(ApplyVelocityEvent {
                    entity,
                    pos: pos.0,
                    destination: path.0.pop_front().unwrap(),
                });
            }
        }
    }

    fn apply_initial_velocity(
        query: Query<(Entity, &Animal, &Path, &Pos), Without<Velocity>>,
        mut ev_apply_velocity: EventWriter<ApplyVelocityEvent>,
    ) {
        for (entity, _, path, pos) in query.iter() {
            println!("Applying initial velocity");
            ev_apply_velocity.send(ApplyVelocityEvent {
                entity,
                pos: pos.0,
                destination: path.0[0],
            });
        }
    }

    fn apply_velocity(
        mut ev_apply_velocity: EventReader<ApplyVelocityEvent>,
        mut commands: Commands,
    ) {
        for ev in ev_apply_velocity.iter() {
            // println!("Dest: {} {}", ev.destination.x, ev.destination.y);
            // println!("Pos: {} {}", ev.pos.ceil().x, ev.pos.ceil().y);
            let velocity = (ev.destination - ev.pos.ceil()).normalize();
            commands.entity(ev.entity).insert(Velocity(velocity));
        }
    }

    fn evaluate_animal_state(
        mut moving_animal_query: Query<(&Animal, &mut AnimalState), With<Velocity>>,
        mut idle_animal_query: Query<(&Animal, &mut AnimalState), Without<Velocity>>,
    ) {
        for (_, mut state) in moving_animal_query.iter_mut() {
            if *state == AnimalState::Moving {
                continue;
            }
            *state = AnimalState::Moving;
        }

        for (_, mut state) in idle_animal_query.iter_mut() {
            if *state == AnimalState::Idle || *state == AnimalState::Eating {
                continue;
            }
            *state = AnimalState::Idle;
        }
    }

    fn evaluate_animal_direction(
        mut query: Query<(&Animal, &Velocity, &mut AnimalDirection)>,
        animal_direction_map: Res<AnimalDirectionVectorMap>,
    ) {
        for (_, velocity, mut direction) in query.iter_mut() {
            let mut target_direction = direction.clone();
            let biggest_dot = velocity
                .0
                .dot(*animal_direction_map.0.get(&target_direction).unwrap());
            for dir in AnimalDirection::iter() {
                if dir == target_direction {
                    continue;
                }
                let current_dot = velocity.0.dot(*animal_direction_map.0.get(&dir).unwrap());
                if current_dot > biggest_dot {
                    target_direction = dir;
                }
            }

            if target_direction != *direction {
                *direction = target_direction;
            }
        }
    }

    fn move_animals(mut query: Query<(&Animal, &Velocity, &mut Pos)>, time: Res<Time>) {
        for (_, velocity, mut pos) in query.iter_mut() {
            println!("Velocity: {} {}", velocity.0.x, velocity.0.y);
            pos.0 += velocity.0 * BUNNY_SPEED * time.delta_seconds();

            // println!("Pos: {} {}", pos.0.x, pos.0.y);
        }
    }
}
