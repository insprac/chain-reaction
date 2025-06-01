use bevy::{core_pipeline::bloom::Bloom, prelude::*};

const MOVE_SPEED: f32 = 10.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player.in_set(PlayerSet))
            .add_systems(Update, update_player_movement.in_set(PlayerSet));
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerSet;

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct Player;

#[derive(Component)]
#[require(Camera3d)]
pub struct PlayerCamera;

fn spawn_player(
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

fn update_player_movement(
    key_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut q_player_transform: Query<&mut Transform, With<Player>>,
    q_camera_transform: Query<&Transform, (With<PlayerCamera>, Without<Player>)>,
) -> Result {
    let input = Vec2::new(
        -(key_input.pressed(KeyCode::KeyA) as i32 - key_input.pressed(KeyCode::KeyD) as i32) as f32,
        -(key_input.pressed(KeyCode::KeyW) as i32 - key_input.pressed(KeyCode::KeyS) as i32) as f32,
    )
    .normalize_or_zero();

    if input.length() > 0.0 {
        // Get the player camera and rotate input to be relative to the camera
        let camera_transform = q_camera_transform.single()?;
        let camera_yaw = -camera_transform.rotation.to_euler(EulerRot::YXZ).0;
        let rotated_input = Vec2::new(
            input.x * camera_yaw.cos() - input.y * camera_yaw.sin(),
            input.x * camera_yaw.sin() + input.y * camera_yaw.cos(),
        );

        let mut player_transform = q_player_transform.single_mut()?;
        player_transform.translation +=
            Vec3::new(rotated_input.x, 0.0, rotated_input.y) * MOVE_SPEED * time.delta_secs();
    }

    Ok(())
}
