use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow}; // ou avian2d::prelude::* pour un jeu 2D

use crate::camera::CameraPlugin;
// use crate::net::NetworkPlugin;
use crate::scene::ScenePlugin;
use crate::ui::perf::fps::FpsPlugin;

pub fn base_app() -> App {
    let mut app = App::new();

    app.add_plugins(PhysicsPlugins::default())
        .add_plugins(ScenePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(FpsPlugin)
        .add_systems(Startup, hide_cursor)
        .add_systems(Startup, setup);

    app
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(1000.0, 0.1),
        Mesh3d(meshes.add(Cylinder::new(100.0, 0.1))),
        MeshMaterial3d(materials.add(Color::WHITE)),
    ));

    let count: u32 = 10;
    let side: f32 = 50.0;

    // grille carrée la plus compacte pour 1000 éléments
    let per_side = (count as f32).sqrt().ceil() as u32; // 32
    let spacing = side / per_side as f32; // ~0.078125
    let start = -side * 0.5 + spacing * 0.5; // centre la grille dans [-1.25, 1.25]

    // handles réutilisables
    let mesh = meshes.add(Cuboid::from_length(1.0));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb_u8(124, 144, 255),
        ..Default::default()
    });

    let mut batch = Vec::with_capacity(count as usize);
    for i in 0..count {
        let x_i = i % per_side;
        let z_i = i / per_side;
        let x = start + x_i as f32 * spacing;
        let z = start + z_i as f32 * spacing;

        batch.push((
            RigidBody::Dynamic,
            Collider::cuboid(1.0, 1.0, 1.0), // attention: en rapier, ce sont des demi-extensions
            AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            Transform::from_xyz(x, 4.0, z),
        ));
    }

    commands.spawn_batch(batch);
}
fn hide_cursor(mut windows: Query<&mut CursorOptions, With<PrimaryWindow>>) {
    let Ok(mut cursor_options) = windows.single_mut() else {
        return;
    };

    cursor_options.visible = false;
    cursor_options.grab_mode = CursorGrabMode::Locked;
}
