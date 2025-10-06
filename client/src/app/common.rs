use crate::camera::CameraPlugin;
use crate::scene::ScenePlugin;
use bevy::prelude::*;

pub fn base_app() -> App {
    let mut app = App::new();
    app.add_plugins(ScenePlugin).add_plugins(CameraPlugin);
    app
}
