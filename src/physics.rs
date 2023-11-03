mod collision;
mod gravity;

use bevy::prelude::*;

pub use collision::*;
pub use gravity::*;

use crate::time::TimeScale;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CollisionPlugin).add_systems(
            FixedUpdate,
            (velocity_system, gravity_system.before(velocity_system))
                .in_set(PhysicsSet::PhysicsSet),
        );
    }
}

#[derive(SystemSet, Clone, PartialEq, Eq, Hash, Debug)]
pub enum PhysicsSet {
    PhysicsSet,
}

#[derive(Component)]
pub struct Mass(pub f32);

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

pub fn velocity_system(
    time: Res<FixedTime>,
    time_scale: Res<TimeScale>,
    mut query: Query<(&Velocity, &mut Transform)>) {
    query
        .par_iter_mut()
        .for_each_mut(|(velocity, mut transform)| {
            transform.translation += (velocity.0 * time_scale.delta_f32(&time)).extend(0.0);
        });
}
