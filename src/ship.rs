use std::f32::consts::{PI, TAU};

use crate::{
    get_input_dir,
    physics::{AffectedByGravity, Circle, Collision, Mass, Velocity},
    planet::Planet, time::TimeScale, camera::CameraTarget,
};
use bevy::prelude::*;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ship)
            .add_systems(Update, (input_system, set_sky_color_by_planet_distance));
    }
}

#[derive(Component)]
pub struct Ship {
    pub max_fuel: f32,
    pub fuel: f32,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            fuel: 100.0,
            max_fuel: 100.0,
        }
    }
}

#[derive(Bundle)]
struct ShipBundle {
    ship: Ship,
    velocity: Velocity,
    mass: Mass,
    affected_by_gravity: AffectedByGravity,
    collision: Collision,
    sprite: SpriteBundle,
    t: CameraTarget,
}

impl Default for ShipBundle {
    fn default() -> Self {
        Self {
            mass: Mass(1.0),
            ship: default(),
            velocity: default(),
            affected_by_gravity: default(),
            collision: default(),
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(10.0)),
                    ..default()
                },
                ..default()
            },
            t: default(),
        }
    }
}

fn setup_ship(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(ShipBundle {
        sprite: SpriteBundle {
            texture: asset_server.load("ship.png"),
            ..ShipBundle::default().sprite
        },
        ..default()
    });
}

const MAX_ROTATION_SPEED: f32 = 3.0;
const MAX_VELOCITY_CHANGE: f32 = 100.0;
fn input_system(
    mut ships: Query<(&mut Velocity, &mut Transform, &mut Ship)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    time_scale: Res<TimeScale>,
) {
    // Only move the ship if the alt key is not held down
    if input.pressed(KeyCode::AltLeft) || input.pressed(KeyCode::AltRight) {
        return;
    }

    let dir = get_input_dir(&input);
    for (mut velocity, mut transform, mut ship) in ships.iter_mut() {
        if input.pressed(KeyCode::Space) {
            let fuel_used = time_scale.delta_f32(&time).min(ship.fuel);
            ship.fuel -= fuel_used;
            velocity.0 += transform.right().truncate() * fuel_used * MAX_VELOCITY_CHANGE;
        }

        if dir != Vec2::ZERO {
            let max_rotation = MAX_ROTATION_SPEED * time_scale.delta_f32(&time);
            let target_angle = dir.y.atan2(dir.x).rem_euclid(TAU);
            let current_angle = transform.rotation.to_axis_angle().1.rem_euclid(TAU);
            let next_angle = move_angle_towards(current_angle, target_angle, max_rotation);

            transform.rotation = Quat::from_rotation_z(next_angle.rem_euclid(TAU));
        }
    }
}

fn move_angle_towards(angle: f32, target: f32, step: f32) -> f32 {
    assert!(step >= 0.0);

    let mut diff = (target - angle).rem_euclid(TAU);
    if diff > PI {
        diff -= TAU;
    }
    if diff.abs() < step {
        target
    } else {
        angle + step * diff.signum()
    }
}

fn set_sky_color_by_planet_distance(
    ship: Query<&Transform, With<Ship>>,
    planets: Query<(&Transform, &Circle), With<Planet>>,
    mut sky: ResMut<ClearColor>,
) {
    let ship_transform = ship.single();
    let ship_position = ship_transform.translation.truncate();

    let (min_distance, radius) = if let Some((planet_transform, planet_circle)) =
        planets.iter().min_by(|(tr_1, _), (tr_2, _)| {
            let dist_1 = tr_1
                .translation
                .truncate()
                .distance_squared(ship_position);
            let dist_2 = tr_2
                .translation
                .truncate()
                .distance_squared(ship_position);
            dist_1.partial_cmp(&dist_2).unwrap()
        }) {
        let distance_squared =
            ship_position.distance_squared(planet_transform.translation.truncate());
        let radius = planet_circle.radius;
        ((distance_squared - radius * radius).max(0.).sqrt(), radius)
    } else {
        (1.0, 1.0)
    };

    *sky = ClearColor(get_sky_color(min_distance, radius));
}

fn get_sky_color(distance: f32, radius: f32) -> Color {
    let color_1: Color = Color::hex("88b4db").unwrap();
    let color_2: Color = Color::hex("150e19").unwrap();

    let t = (distance / radius + 0.05).min(1.0).max(0.0);

    Vec4::lerp(color_1.into(), color_2.into(), t).into()
}
