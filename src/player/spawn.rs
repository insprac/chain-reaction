use bevy::prelude::*;

use super::{Player, PlayerCamera, PlayerGun};

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Player,
        PlayerGun::default(),
        children![
            // Camera
            (
                PlayerCamera,
                Camera {
                    hdr: true,
                    ..default()
                },
                Transform::from_xyz(0.0, 20.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ),
            // Player body
            (
                Mesh3d(meshes.add(Cylinder::new(0.5, 0.2))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::hsl(100.0, 0.7, 0.5),
                    perceptual_roughness: 1.0,
                    ..default()
                })),
                Transform::from_xyz(0.0, 0.5, 0.0),
            ),
            // Light
            (
                PointLight {
                    range: 10.0,
                    ..default()
                },
                Transform::from_xyz(0.0, 2.0, 0.0),
            ),
        ],
    ));
}
