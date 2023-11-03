mod loader;

use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<LevelAsset>()
            .add_asset_loader(loader::LevelAssetLoader)
            .add_systems(Update, (spawn_entities, listen_for_level_loading));
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LevelAssetObject {
    Planet { position: Vec2, radius: f32 },
}

#[derive(Debug, TypeUuid, TypePath)]
#[uuid = "209cad4e-c5fd-48ed-b16e-567adf098ed2"]
pub struct LevelAsset {
    pub objects: Vec<LevelAssetObject>,
}

#[derive(Component, Debug, Default)]
pub struct LevelObject;

#[derive(Component, Debug, Default)]
pub struct Level {
    pub level_asset: Handle<LevelAsset>,
    pub objects: Vec<Entity>,
}

#[derive(Component)]
pub struct LevelAssetLoaded;

#[derive(Component)]
pub struct LevelDoneLoading;

fn spawn_entities(
    mut levels: Query<(Entity, &mut Level), Without<LevelAssetLoaded>>,
    asset_server: Res<AssetServer>,
    level_assets: Res<Assets<LevelAsset>>,
    mut commands: Commands,
) {
    for (entity, mut level) in levels.iter_mut() {
        if let Some(level_asset) = level_assets.get(&level.level_asset) {
            // The asset has finished loading!
            commands.entity(entity).insert(LevelAssetLoaded);
            // Now load the objects.
            level.objects = spawn_level_objects(level_asset, &asset_server, &mut commands);
        }
    }
}

fn spawn_level_objects(
    level_asset: &LevelAsset,
    asset_server: &AssetServer,
    commands: &mut Commands,
) -> Vec<Entity> {
    let mut ret = vec![];
    for object in &level_asset.objects {
        ret.push(spawn_object(object, asset_server, commands));
    }
    ret
}

fn spawn_object(
    object: &LevelAssetObject,
    asset_server: &AssetServer,
    commands: &mut Commands,
) -> Entity {
    use crate::planet::PlanetBundle;

    match object {
        LevelAssetObject::Planet { position, radius } => commands
            .spawn((LevelObject, PlanetBundle::new(asset_server, *radius, *position)))
            .id(),
    }
}

fn listen_for_level_loading(
    levels: Query<(Entity, &Level), (With<LevelAssetLoaded>, Without<LevelDoneLoading>)>,
    level_objects: Query<&LevelObject>,
    level_assets: Res<Assets<LevelAsset>>,
    mut commands: Commands,
) {
    for (entity, level) in levels.iter() {
        let objects = level_assets.get(&level.level_asset).unwrap().objects.len();
        if level.objects.len() == objects {
            // Check if all objects have been loaded.
            if level.objects.iter().all(|&entity| {
                level_objects.get(entity).is_ok()
            }) {
                commands.entity(entity).insert(LevelDoneLoading);
            }
        }
    }
}
