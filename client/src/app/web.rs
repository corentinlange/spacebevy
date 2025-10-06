use bevy::prelude::*;

use super::common::base_app;

#[cfg(target_arch = "wasm32")]
pub fn run() {
    console_error_panic_hook::set_once();

    let mut app = base_app();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "SpaceBevy (Web)".into(),
            canvas: Some("#bevy".into()),
            fit_canvas_to_parent: true,
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    }));

    app.run();
}
