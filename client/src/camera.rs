use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_controls);
    }
}

// --- Contrôles ZQSD ---
fn camera_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<PlayerCamera>>,
) {
    // On essaye de récupérer la caméra, mais sans paniquer si elle n'existe pas.
    let Ok(mut transform) = query.single_mut() else {
        return; // aucune caméra PlayerCamera -> on sort
    };
    let mut direction = Vec3::ZERO;
    let speed = 5.0 * time.delta_secs();

    if keyboard.pressed(KeyCode::KeyZ) {
        direction.z -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.z += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyQ) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keyboard.pressed(KeyCode::Space) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::ShiftLeft) {
        direction.y -= 1.0;
    }

    if direction.length_squared() > 0.0 {
        transform.translation += direction.normalize() * speed;
    }
}
