use crate::ui::styles::UiStyle;
use bevy::prelude::*;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System};

#[derive(Resource)]
struct MemoryData {
    sys: System,
    pid: sysinfo::Pid,
    timer: Timer,
}

#[derive(Component)]
struct MemoryText;

pub struct MemoryPlugin;

impl Plugin for MemoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MemoryData {
            sys: System::new_with_specifics(
                RefreshKind::default().with_processes(ProcessRefreshKind::default()),
            ),
            pid: sysinfo::get_current_pid().unwrap(),
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        })
        .add_systems(Startup, setup_memory_ui)
        .add_systems(Update, update_memory_text);
    }
}

fn setup_memory_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text::new("RAM: ..."),
        UiStyle::text_style(&asset_server, 18.0, UiStyle::TEXT_COLOR),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(70.0),
            left: Val::Px(10.0),
            ..default()
        },
        MemoryText,
    ));
}

fn update_memory_text(
    time: Res<Time>,
    mut data: ResMut<MemoryData>,
    mut query: Query<&mut Text, With<MemoryText>>,
) {
    data.timer.tick(time.delta());
    if !data.timer.is_finished() {
        return;
    }

    let pid = data.pid;
    let sys = &mut data.sys;
    let ram = data
        .sys
        .process(data.pid)
        .map(|p| p.memory() as f32 / 1_048_576.0)
        .unwrap_or(0.0);

    for mut text in &mut query {
        text.0 = format!("RAM: {:.1} MB", ram);
    }
}
