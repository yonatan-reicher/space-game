use bevy::prelude::*;

use crate::ship::Ship;

pub struct FuelBarPlugin;

impl Plugin for FuelBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_fuelbar)
            .add_systems(Update, fuelbar_system);
    }
}

#[derive(Component)]
struct FuelBar;

const MIDDLE_WIDTH_VW: f32 = 30.0;
const HEIGHT_VW: f32 = 4.0;
const WIDTH_VW: f32 = 2.0 * HEIGHT_VW + MIDDLE_WIDTH_VW;
const FILL_PADDING_VW: f32 = HEIGHT_VW * 0.1;
const FILL_WIDTH_VW: f32 = WIDTH_VW - 2.0 * FILL_PADDING_VW;
const FILL_HEIGHT_VW: f32 = HEIGHT_VW - 2.0 * FILL_PADDING_VW;

fn setup_fuelbar(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    let atlas = atlases.add(TextureAtlas::from_grid(
        asset_server.load("fuelbar.png"),
        Vec2::new(32., 32.),
        4,
        1,
        None,
        None,
    ));

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Vw(WIDTH_VW),
                height: Val::Vw(HEIGHT_VW),
                top: Val::Px(5.),
                left: Val::Px(5.),
                position_type: PositionType::Relative,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(AtlasImageBundle {
                texture_atlas: atlas.clone(),
                texture_atlas_image: UiTextureAtlasImage {
                    index: 3,
                    ..default()
                },
                style: Style {
                    width: Val::Vw(FILL_WIDTH_VW),
                    height: Val::Vw(FILL_HEIGHT_VW),
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.),
                    left: Val::Px(0.),
                    margin: UiRect::all(Val::Vw(FILL_PADDING_VW)),
                    ..default()
                },
                background_color: Color::rgb(0.4, 0.5, 0.5).into(),
                ..default()
            });
            parent.spawn((
                FuelBar,
                AtlasImageBundle {
                    texture_atlas: atlas.clone(),
                    texture_atlas_image: UiTextureAtlasImage {
                        index: 3,
                        ..default()
                    },
                    style: Style {
                        width: Val::Vw(FILL_WIDTH_VW),
                        height: Val::Vw(FILL_HEIGHT_VW),
                        position_type: PositionType::Absolute,
                        top: Val::Px(0.),
                        left: Val::Px(0.),
                        margin: UiRect::all(Val::Vw(FILL_PADDING_VW)),
                        ..default()
                    },
                    ..default()
                },
            ));
            parent.spawn(AtlasImageBundle {
                texture_atlas: atlas.clone(),
                texture_atlas_image: UiTextureAtlasImage {
                    index: 0,
                    ..default()
                },
                ..default()
            });
            parent.spawn(AtlasImageBundle {
                texture_atlas: atlas.clone(),
                texture_atlas_image: UiTextureAtlasImage {
                    index: 2,
                    ..default()
                },
                style: Style {
                    width: Val::Vw(MIDDLE_WIDTH_VW),
                    ..default()
                },
                ..default()
            });
            parent.spawn(AtlasImageBundle {
                texture_atlas: atlas,
                texture_atlas_image: UiTextureAtlasImage {
                    index: 1,
                    ..default()
                },
                ..default()
            });
        });
}

fn fuelbar_system(
    ship_query: Query<&Ship>,
    mut fuelbar_query: Query<&mut Style, With<FuelBar>>,
) {
    let ship = ship_query.single();
    let mut fuelbar_style = fuelbar_query.single_mut();

    fuelbar_style.width = Val::Vw(FILL_WIDTH_VW * ship.fuel / ship.max_fuel);
}
