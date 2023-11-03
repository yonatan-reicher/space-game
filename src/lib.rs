mod camera;
mod player;
mod fuelbar;
mod level;
mod physics;
mod physics_prediction;
mod planet;
mod ship;
mod time;

use bevy::{app::AppExit, prelude::*};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(
                // Don't blur pixels when scaling
                ImagePlugin::default_nearest(),
            ),
            camera::CameraPlugin,
            ship::ShipPlugin,
            physics::PhysicsPlugin,
            planet::PlanetPlugin,
            physics_prediction::PhysicsPredictionPlugin,
            fuelbar::FuelBarPlugin,
            level::LevelPlugin,
            time::TimePlugin,
            player::PlayerPlugin,
        ))
        .insert_resource(ClearColor(Color::hex("1d2b53").unwrap()))
        .add_systems(Startup, setup)
        .add_systems(Update, (stop_on_escape_system, set_time_scale))
        .run();
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(level::Level {
        level_asset: asset_server.load("Level1.txt"),
        ..default()
    });
}

pub fn get_input_dir(input: &Input<KeyCode>) -> Vec2 {
    let mut dir = Vec2::ZERO;
    if input.pressed(KeyCode::Up) {
        dir.y += 1.0;
    }
    if input.pressed(KeyCode::Down) {
        dir.y -= 1.0;
    }
    if input.pressed(KeyCode::Left) {
        dir.x -= 1.0;
    }
    if input.pressed(KeyCode::Right) {
        dir.x += 1.0;
    }
    dir.normalize_or_zero()
}

fn stop_on_escape_system(input: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

fn set_time_scale(input: Res<Input<KeyCode>>, mut time_scale: ResMut<time::TimeScale>) {
    if let Some(number) = get_min_number_pressed(&input) {
        time_scale.0 = 2.0f64.powi(number as i32 - 5);
    }
}

/// Zero is returned as 10
fn get_min_number_pressed(input: &Input<KeyCode>) -> Option<u16> {
    const NUMBER_CODES: [(KeyCode, KeyCode); 10] = [
        (KeyCode::Key1, KeyCode::Numpad1),
        (KeyCode::Key2, KeyCode::Numpad2),
        (KeyCode::Key3, KeyCode::Numpad3),
        (KeyCode::Key4, KeyCode::Numpad4),
        (KeyCode::Key5, KeyCode::Numpad5),
        (KeyCode::Key6, KeyCode::Numpad6),
        (KeyCode::Key7, KeyCode::Numpad7),
        (KeyCode::Key8, KeyCode::Numpad8),
        (KeyCode::Key9, KeyCode::Numpad9),
        (KeyCode::Key0, KeyCode::Numpad0),
    ];

    for i in 0..10u16 {
        let (key, numpad) = NUMBER_CODES[i as usize];
        if input.just_pressed(key) || input.just_pressed(numpad) {
            return Some(i + 1);
        }
    }

    None
}
