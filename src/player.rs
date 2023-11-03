use bevy::prelude::*;

use crate::{
    get_input_dir,
    physics::get_gravity_acceleration,
    physics::{AffectedByGravity, Circle, Collision, GravitySource, Mass, Velocity},
    time::TimeScale,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, input_system)
            .add_systems(Update, rotate);
    }
}

#[derive(Component)]
pub struct Player;

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        Velocity::default(),
        Mass(0.1),
        AffectedByGravity,
        Collision,
        Circle { radius: 0.5 },
        SpriteBundle {
            sprite: Sprite {
                custom_size: Vec2::splat(1.0).into(),
                ..default()
            },
            texture: asset_server.load("player.png"),
            visibility: Visibility::Hidden,
            ..default()
        },
    ));
}

const PLAYER_SPEED: f32 = 50.0;
fn input_system(
    mut player: Query<&mut Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    time_scale: Res<TimeScale>,
) {
    let mut player = player.single_mut();
    let dir = player.rotation * get_input_dir(&input).extend(0.0) ;
    player.translation += dir * PLAYER_SPEED * time_scale.delta_f32(time);
}

fn rotate(
    mut player: Query<&mut Transform, With<Player>>,
    gravity_sources: Query<(&Transform, &Mass), (With<GravitySource>, Without<Player>)>,
) {
    let mut player = player.single_mut();
    let gravity_sources = gravity_sources.iter();
    let strongest_affector_relative_point =
        strongest_affector(gravity_sources, player.translation.truncate());

    if let Some(dir) = strongest_affector_relative_point {
        let angle = dir.y.atan2(dir.x);
        player.rotation = Quat::from_rotation_z(angle + 90.0f32.to_radians());
    }
}

fn strongest_affector<'a>(
    iter: impl IntoIterator<Item = (&'a Transform, &'a Mass)>,
    point: Vec2,
) -> Option<Vec2> {
    iter.into_iter()
        .flat_map(|(transform, mass)| {
            let dir = transform.translation.truncate() - point;
            get_gravity_acceleration(dir, mass.0)
                .ok()
                .map(Vec2::length_squared)
                .map(|accel| (transform, accel))
        })
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(transform, _)| transform.translation.truncate() - point)
}
