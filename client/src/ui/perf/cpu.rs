use crate::ui::styles::UiStyle;
use bevy::prelude::*;
use sysinfo::{ProcessRefreshKind, RefreshKind, System};

#[derive(Resource)]
struct CpuData {
    sys: System,
    pid: sysinfo::Pid,
    timer: Timer,
}

#[derive(Component)]
struct CpuText;

pub struct CpuPlugin;

impl Plugin for CpuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CpuData {
            sys: System::new_with_specifics(
                RefreshKind::default().with_processes(ProcessRefreshKind::default()),
            ),
            pid: sysinfo::get_current_pid().unwrap(),
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        })
        .add_systems(Startup, setup_cpu_ui)
        .add_systems(Update, update_cpu_text);
    }
}

fn setup_cpu_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text::new("CPU: ..."),
        UiStyle::text_style(&asset_server, 18.0, UiStyle::TEXT_COLOR),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            left: Val::Px(10.0),
            ..default()
        },
        CpuText,
    ));
}

fn update_cpu_text(
    time: Res<Time>,
    mut data: ResMut<CpuData>,
    mut query: Query<&mut Text, With<CpuText>>,
) {
    data.timer.tick(time.delta());
    if !data.timer.is_finished() {
        return;
    }

    let pid = data.pid;
    let sys = &mut data.sys;
    let cpu = data
        .sys
        .process(data.pid)
        .map(|p| p.cpu_usage())
        .unwrap_or(0.0);

    for mut text in &mut query {
        text.0 = format!("CPU: {:.1}%", cpu);
    }
}
