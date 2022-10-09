use std::collections::VecDeque;


use crate::prelude::*;

// At some point each animal will have its own speed but for now this is good enough
const BUNNY_SPEED: f32 = 5.0;

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
    pub pos: Pos,
    pub destination: Vec2,
}

impl AnimalBehaviourPlugin {
    pub fn move_along_path(
        mut query: Query<(Entity, &Animal, &mut Path, &Pos)>,
        mut ev_apply_velocity: EventWriter<ApplyVelocityEvent>,
    ) {
        for (entity, _, mut path, pos) in query.iter_mut() {
            let next_step_in_path = path.0[0];
            // println!("next step: {}, {}", next_step_in_path.x, next_step_in_path.y);
            // println!("current pos: {}, {}", pos.0.x, pos.0.y);
            if pos.0.x.ceil() == next_step_in_path.x.ceil()
                && pos.0.y.ceil() == next_step_in_path.y.ceil()
            {
                ev_apply_velocity.send(ApplyVelocityEvent {
                    entity,
                    pos: pos.clone(),
                    destination: path.0.pop_front().unwrap(),
                });
            }
        }
    }
    
    pub fn apply_initial_velocity(query: Query<(Entity, &Animal, &Path, &Pos), Without<Velocity>>, mut ev_apply_velocity: EventWriter<ApplyVelocityEvent>) {
        for (entity, _, path, pos) in query.iter() {
            println!("Applying initial velocity");
            ev_apply_velocity.send(ApplyVelocityEvent { entity, pos: pos.clone(), destination: path.0[0] });
        }
    }

    pub fn apply_velocity(
        mut ev_apply_velocity: EventReader<ApplyVelocityEvent>,
        mut commands: Commands,
    ) {
        for ev in ev_apply_velocity.iter() {
            let velocity = (ev.destination - ev.pos.0).normalize();
            commands.entity(ev.entity).insert(Velocity(velocity));
        }
    }

    pub fn move_animals(mut query: Query<(&Animal, &Velocity, &mut Pos)>, time: Res<Time>) {
        for (_, velocity, mut pos) in query.iter_mut() {
            // println!("Velocity: {} {}", velocity.0.x, velocity.0.y);
            pos.0 += velocity.0 * BUNNY_SPEED * time.delta_seconds();
            println!("Pos: {} {}", pos.0.x, pos.0.y);
        }
    }
}
