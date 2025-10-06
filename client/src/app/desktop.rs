use bevy::prelude::*;

use crate::net::NetworkPlugin;
use super::common::base_app;

pub fn run() {
    let mut app = base_app();
    app.add_plugins(DefaultPlugins)
        .add_plugins(NetworkPlugin)
        .run();
}
