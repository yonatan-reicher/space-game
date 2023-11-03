use bevy::prelude::*;

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TimeScale::default())
            .add_systems(Startup, setup_time_scale_ui)
            .add_systems(Update, (time_scale_ui, change_fixed_time_step));
    }
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TimeScale(pub f64);

impl Default for TimeScale {
    fn default() -> Self {
        Self(1.)
    }
}

impl TimeScale {
    pub fn delta<T>(&self, time_step: impl AsRef<T>) -> f64
    where
        T: TimeStep,
    {
        self.0 * time_step.as_ref().time_step()
    }

    pub fn delta_f32<T>(&self, time_step: impl AsRef<T>) -> f32
    where
        T: TimeStep,
    {
        self.delta(time_step) as _
    }
}

pub trait TimeStep {
    fn time_step(&self) -> f64;
}

impl TimeStep for Time {
    fn time_step(&self) -> f64 {
        self.delta_seconds_f64()
    }
}

impl TimeStep for FixedTime {
    fn time_step(&self) -> f64 {
        self.period.as_secs_f64()
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimeScaleText;

fn setup_time_scale_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            make_ui_text(1.),
            TextStyle {
                font: asset_server.load("pixeboy.ttf"),
                font_size: 20.,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::Left)
        .with_no_wrap()
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.),
            right: Val::Px(5.),
            ..default()
        }),
        TimeScaleText,
    ));
}

fn make_ui_text(time_scale: f32) -> String {
    let time_scale_fraction = float_as_fraction(time_scale);
    format!("Time Scale: {time_scale_fraction}")
}

/// Returns the number as a text fraction, from a set of predefined fractions.
fn float_as_fraction(number: f32) -> String {
    const FRACTIONS: [(f32, &str); 10] = [
        (1. / 16., "1/16"),
        (1. / 8., "1/8"),
        (1. / 4., "1/4"),
        (1. / 2., "1/2"),
        (1., "1"),
        (2., "2"),
        (4., "4"),
        (8., "8"),
        (16., "16"),
        (32., "32"),
    ];
    FRACTIONS
        .iter()
        .find(|(f, _)| (number - f).abs() < 0.01)
        .map(|(_, s)| s.to_string())
        .unwrap_or(number.to_string())
}

fn time_scale_ui(
    time_scale: Res<TimeScale>,
    mut query: Query<&mut Text, With<TimeScaleText>>,
) {
    if !time_scale.is_changed() {
        return;
    }

    let mut text = query.single_mut();
    text.sections[0].value = make_ui_text(time_scale.0 as _);
}

/// When the time scale changes, the fixed time step needs to be updated to run
/// enough times to not lose any precision.
fn change_fixed_time_step(
    time_scale: Res<TimeScale>,
    mut fixed_time: ResMut<FixedTime>,
) {
    if !time_scale.is_changed() {
        return;
    }

    let secs = 1. / 60. / time_scale.0.max(1.);
    fixed_time.period = std::time::Duration::from_secs_f64(secs);
}
