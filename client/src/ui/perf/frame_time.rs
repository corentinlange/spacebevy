use crate::ui::styles::UiStyle;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

#[derive(Component)]
struct FrameTimeText;

pub struct FrameTimePlugin;

impl Plugin for FrameTimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_frame_time_ui)
            .add_systems(Update, update_frame_time);
    }
}

fn setup_frame_time_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text::new("Frame Time: ..."),
        UiStyle::text_style(&asset_server, 18.0, UiStyle::TEXT_COLOR),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(30.0),
            left: Val::Px(10.0),
            ..default()
        },
        FrameTimeText,
    ));
}

fn update_frame_time(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FrameTimeText>>,
    time: Res<Time>,
    mut timer: Local<Timer>,
) {
    if timer.duration().is_zero() {
        *timer = Timer::from_seconds(1.0, TimerMode::Repeating);
    }
    timer.tick(time.delta());
    if !timer.finished() {
        return;
    }

    if let Some(fps) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|v| v.smoothed())
    {
        let ms = 1000.0 / fps.max(1.0);
        for mut text in &mut query {
            text.0 = format!("Frame Time: {:.2} ms", ms);
        }
    }
}
