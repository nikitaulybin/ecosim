use crate::prelude::*;

// At some point each animal will have its own speed but for now this is good enough
const BUNNY_SPEED: f32 = 5.0;
const VELOCITY_REAPPLY_TILE_PROXIMITY: f32 = 2.0;
pub struct AnimalBehaviourPlugin;

impl Plugin for AnimalBehaviourPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ApplyVelocityEvent>()
            .add_system(Self::apply_initial_velocity)
            .add_system(Self::move_along_path)
            .add_system(Self::apply_velocity)
            .add_system(Self::move_animals);
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

pub struct ApplyVelocityEvent {
    pub entity: Entity,
    pub pos: Vec2,
    pub destination: Vec2,
}

impl AnimalBehaviourPlugin {
    pub fn move_along_path(
        mut query: Query<(Entity, &Animal, &mut Path, &mut Pos)>,
        mut ev_apply_velocity: EventWriter<ApplyVelocityEvent>,
    ) {
        for (entity, _, mut path, mut pos) in query.iter_mut() {
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
                pos.0 = path.0.pop_front().unwrap(); 
                ev_apply_velocity.send(ApplyVelocityEvent {
                    entity,
                    pos: pos.0,
                    destination: path.0.pop_front().unwrap(),
                });
            }
        }
    }

    pub fn apply_initial_velocity(
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

    pub fn apply_velocity(
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

    pub fn move_animals(mut query: Query<(&Animal, &Velocity, &mut Pos)>, time: Res<Time>) {
        for (_, velocity, mut pos) in query.iter_mut() {
            println!("Velocity: {} {}", velocity.0.x, velocity.0.y);
            pos.0 += velocity.0 * BUNNY_SPEED * time.delta_seconds();
            // println!("Pos: {} {}", pos.0.x, pos.0.y);
        }
    }
}
