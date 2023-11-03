use bevy::{
    asset::{AssetLoader, LoadedAsset, Error},
    prelude::*,
};

use crate::level::LevelAssetObject;

use super::LevelAsset;

pub struct LevelAssetLoader;

impl AssetLoader for LevelAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let string = std::str::from_utf8(bytes).map_err(Error::new)?;
            let level = parse_level(string)?;
            load_context.set_default_asset(LoadedAsset::new(level));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["txt"]
    }
}

fn parse_level(source: &str) -> Result<LevelAsset, Error> {
    let objects = source
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(LevelAsset { objects })
}

fn parse_line(line: &str) -> Result<LevelAssetObject, Error> {
    let line = line.trim();
    if !line.starts_with("Planet ") {
        return Err(Error::msg("Invalid line"));
    }

    let parts = line.split(' ').collect::<Vec<_>>();
    if parts.len() != 4 {
        return Err(Error::msg("Invalid line"));
    }

    let x = parts[1].parse::<f32>()?;
    let y = parts[2].parse::<f32>()?;
    let radius = parts[3].parse::<f32>()?;
    Ok(LevelAssetObject::Planet {
        radius,
        position: Vec2::new(x, y),
    })
}
