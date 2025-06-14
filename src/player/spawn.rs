use bevy::prelude::*;

use crate::{
    AppState,
    force::ForceEmitter,
    health::{DiedEvent, Health},
};

use super::{Player, PlayerCamera, PlayerGun};

pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Player,
            PlayerGun::default(),
            Health::new(3),
            ForceEmitter {
                radius: 5.0,
                strength: 20.0,
            },
            children![
                // Camera
                (
                    PlayerCamera,
                    Camera {
                        hdr: true,
                        ..default()
                    },
                    Transform::from_xyz(0.0, 30.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
                ),
                // Player body
                (
                    Mesh3d(meshes.add(Cylinder::new(0.5, 0.2))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::hsl(100.0, 0.7, 0.3),
                        perceptual_roughness: 1.0,
                        ..default()
                    })),
                    Transform::from_xyz(0.0, 1.0, 0.0),
                ),
                // Light
                (
                    PointLight {
                        range: 30.0,
                        intensity: 8_000_000.0,
                        ..default()
                    },
                    Transform::from_xyz(0.0, 5.0, 0.0),
                ),
            ],
        ))
        .observe(player_death_observer);
}

pub fn cleanup_player(mut commands: Commands, q_player: Query<Entity, With<Player>>) -> Result {
    let entity = q_player.single()?;
    commands.entity(entity).try_despawn();
    Ok(())
}

pub fn player_death_observer(
    _trigger: Trigger<DiedEvent>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    next_app_state.set(AppState::GameOver);
}
