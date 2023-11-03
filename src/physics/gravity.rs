use super::{Mass, Velocity};
use crate::time::TimeScale;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct GravitySource;

#[derive(Component, Default)]
pub struct AffectedByGravity;

pub fn gravity_system(
    time: Res<FixedTime>,
    time_scale: Res<TimeScale>,
    affectors: Query<(&Transform, &Mass), With<GravitySource>>,
    mut affected: Query<(&mut Velocity, &Transform), With<AffectedByGravity>>,
) {
    for (mut velocity, affected_transform) in affected.iter_mut() {
        let affected_position = affected_transform.translation.truncate();
        let acceleration = get_total_gravity_acceleration(&affectors, affected_position);
        velocity.0 += acceleration * time_scale.delta_f32(&time);
    }
}

fn get_total_gravity_acceleration(
    affectors: &Query<(&Transform, &Mass), With<GravitySource>>,
    point: Vec2,
) -> Vec2 {
    let mut acceleration = Vec2::ZERO;
    for (affector_transform, affector_mass) in affectors.iter() {
        let affector_position = affector_transform.translation.truncate();
        let relative_position = affector_position - point;
        if let Ok(change_in_vel) = get_gravity_acceleration(relative_position, affector_mass.0) {
            acceleration += change_in_vel;
        }
    }
    acceleration
}

pub fn get_gravity_acceleration(relative_position: Vec2, mass: f32) -> Result<Vec2, ()> {
    let distance = relative_position.length();
    if distance == 0.0 {
        return Err(());
    }

    Ok(0.5 * relative_position * mass / (distance * distance * distance))
}
