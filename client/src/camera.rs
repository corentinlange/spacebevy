use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (camera_controls, camera_mouse_look));
    }
}

// --- Contrôles ZQSD ---
fn camera_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<PlayerCamera>>,
) {
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

fn camera_mouse_look(
    mut mouse_motion: MessageReader<MouseMotion>,
    mut query: Query<&mut Transform, With<PlayerCamera>>,
) {
    let Ok(mut transform) = query.single_mut() else {
        return;
    };

    let sensitivity = 0.002;

    let mut delta = Vec2::ZERO;
    for ev in mouse_motion.read() {
        delta += ev.delta;
    }

    if delta == Vec2::ZERO {
        return;
    }

    let mut yaw = transform.rotation.to_euler(EulerRot::YXZ).0;
    let mut pitch = transform.rotation.to_euler(EulerRot::YXZ).1;

    yaw -= delta.x * sensitivity;
    pitch -= delta.y * sensitivity;

    pitch = pitch.clamp(-1.54, 1.54);

    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
}
