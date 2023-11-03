use bevy::prelude::*;

use super::{velocity_system, Velocity};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>().add_systems(
            FixedUpdate,
            (
                collision_detection.after(velocity_system),
                collision_resolution.after(collision_detection),
            ).in_set(super::PhysicsSet::PhysicsSet),
        );
    }
}

#[derive(Component)]
pub struct Circle {
    pub radius: f32,
}

/// An entity that collides with colliders.
#[derive(Component, Default)]
pub struct Collision;

/// An entity that can be collided with.
#[derive(Component, Default)]
pub struct Collider;

#[derive(Event)]
pub struct CollisionEvent {
    pub collision_entity: Entity,
    pub collider_entity: Entity,
    pub normal: Vec2,
    pub point: Vec2,
}

pub fn collision_detection(
    collisions: Query<(Entity, &Transform, Option<&Circle>), With<Collision>>,
    colliders: Query<(Entity, &Transform, &Circle), With<Collider>>,
    mut events: EventWriter<CollisionEvent>,
) {
    for (collision_entity, collision_transform, collision_circle) in collisions.iter() {
        for (collider_entity, collider_transform, collider_circle) in colliders.iter() {
            // Don't collide with yourself!
            if collision_entity == collider_entity {
                continue;
            }

            let collision_position = collision_transform.translation.truncate();
            let collider_position = collider_transform.translation.truncate();
            let distance_squared = collision_position.distance_squared(collider_position);
            let min_distance = collision_circle.map_or(0., |c| c.radius) + collider_circle.radius;
            let min_distance_squared = min_distance * min_distance;

            if distance_squared <= min_distance_squared {
                let collider_to_collision = collision_position - collider_position;
                let normal = collider_to_collision.normalize_or_zero();

                let point = collider_position + normal * collider_circle.radius;

                events.send(CollisionEvent {
                    collision_entity,
                    collider_entity,
                    normal,
                    point,
                });
            }
        }
    }
}

pub fn collision_resolution(
    mut collision_events: EventReader<CollisionEvent>,
    mut collisions: Query<
        (&mut Transform, Option<&mut Velocity>, Option<&Circle>),
        With<Collision>,
    >,
) {
    for event in collision_events.iter() {
        let (mut collision_transform, collision_velocity, collision_circle) =
            collisions.get_mut(event.collision_entity).unwrap();

        let radius = collision_circle.map_or(0., |c| c.radius);

        // Move the collision entity out!
        collision_transform.translation = (event.point + event.normal * radius).extend(0.0);

        // And update it's velocity.
        if let Some(mut velocity) = collision_velocity {
            let amount_to_remove = velocity.0.dot(-event.normal).max(0.);
            velocity.0 += amount_to_remove * event.normal;
        }
    }
}
