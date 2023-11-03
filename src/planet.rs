use bevy::prelude::*;

use crate::physics::{GravitySource, Mass, Collider, Circle};

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, _app: &mut App) {
    }
}

#[derive(Component, Default)]
pub struct Planet;

#[derive(Bundle)]
pub struct PlanetBundle {
    planet: Planet,
    gravity_source: GravitySource,
    mass: Mass,
    sprite: SpriteBundle,
    collider: Collider,
    circle: Circle,
}

impl PlanetBundle {
    pub fn new(asset_server: &AssetServer, radius: f32, position: Vec2) -> Self {
        let texture = asset_server.load("planet.png");
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(radius * 2.0)),
                    ..default()
                },
                texture,
                transform: Transform::from_xyz(position.x, position.y, 0.0),
                ..default()
            },
            mass: Mass(radius * radius * radius),
            planet: Planet,
            gravity_source: GravitySource,
            collider: default(),
            circle: Circle { radius },
        }
    }
}
