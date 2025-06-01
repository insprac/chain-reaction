use bevy::{core_pipeline::bloom::Bloom, prelude::*};

use super::{Player, PlayerCamera};

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let color = Color::hsl(100.0, 0.9, 0.5);

    commands.spawn((
        Player,
        children![
            // Camera
            (
                PlayerCamera,
                Camera {
                    hdr: true,
                    ..default()
                },
                Bloom::ANAMORPHIC,
                Transform::from_xyz(10.0, 20.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ),
            // Player visuals
            (
                Mesh3d(meshes.add(Cylinder::new(0.5, 0.2))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: color,
                    perceptual_roughness: 1.0,
                    ..default()
                })),
                Transform::from_xyz(0.0, 0.5, 0.0),
            ),
            // Light
            (
                PointLight {
                    color: Color::WHITE,
                    range: 10.0,
                    ..default()
                },
                Transform::from_xyz(0.0, 2.0, 0.0),
            )
        ],
    ));
}
