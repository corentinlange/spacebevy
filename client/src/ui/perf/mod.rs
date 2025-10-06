pub mod cpu;
pub mod fps;
pub mod frame_time;
pub mod memory;

use self::{cpu::CpuPlugin, fps::FpsPlugin, frame_time::FrameTimePlugin, memory::MemoryPlugin};
use bevy::prelude::*;

pub struct PerfPlugin;

impl Plugin for PerfPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((FpsPlugin, FrameTimePlugin, CpuPlugin, MemoryPlugin));
    }
}

fn setup_ui_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
