use crate::get_input_dir;
use bevy::{
    input::keyboard::KeyCode, input::mouse::MouseWheel, prelude::*, render::camera::Camera,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, (camera_zoom_system, camera_input_system))
            .add_systems(PostUpdate, camera_track_target);
    }
}

#[derive(Component, Default, Clone, Debug)]
pub struct CameraTarget {
    align_rotation: bool,
}

#[derive(Component)]
pub struct CameraAnchor;

const MAX_CAMERA_ZOOM_PERCENT_CHANGE: f32 = 10.0;
const MAX_CAMERA_MOVE_SPEED: f32 = 200.0;

fn setup_camera(mut commands: Commands) {
    commands.spawn((CameraAnchor, TransformBundle::default())).with_children(|parent| {
        parent.spawn(Camera2dBundle::default());
    });
}

fn camera_zoom_system(
    mut query: Query<&mut OrthographicProjection, With<Camera>>,
    mut scroll_events: EventReader<MouseWheel>,
) {
    let mut zoom = 0.0;
    for event in scroll_events.iter() {
        zoom += event.y.signum();
    }

    let zoom_factor = zoom * MAX_CAMERA_ZOOM_PERCENT_CHANGE / 100.0;
    for mut projection in query.iter_mut() {
        projection.scale *= 1.0 - zoom_factor;
    }
}

fn camera_input_system(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &OrthographicProjection), With<Camera>>,
) {
    // Only move the camera if the alt key is held down
    if !input.pressed(KeyCode::AltLeft) && !input.pressed(KeyCode::AltRight) {
        return;
    }

    let (mut transform, proj) = query.single_mut();
    let dir = get_input_dir(&input);
    let position_change = dir * proj.scale * time.delta_seconds() * MAX_CAMERA_MOVE_SPEED;
    transform.translation += position_change.extend(0.0);
}

fn camera_track_target(
    mut query: Query<&mut Transform, With<CameraAnchor>>,
    target_query: Query<(&Transform, &CameraTarget), Without<CameraAnchor>>,
) {
    let (target_tr, target) = target_query.single();
    let mut camera = query.single_mut();

    camera.translation = target_tr
        .translation
        .truncate()
        .extend(camera.translation.z);

    camera.rotation = if target.align_rotation {
        target_tr.rotation
    } else {
        Quat::from_rotation_z(0.0)
    };
}
