use super::common::base_app;
use crate::ui::perf::PerfPlugin;
use bevy::prelude::*;

pub fn run() {
    let mut app = base_app();
    app.add_plugins(DefaultPlugins)
        .add_plugins(PerfPlugin)
        .run();
}
