use bevy::prelude::*;

use super::common::base_app;
use crate::net::NetworkPlugin;

pub fn run() {
    let mut app = base_app();
    app.add_plugins(DefaultPlugins)
        .add_plugins(NetworkPlugin)
        .run();
}
