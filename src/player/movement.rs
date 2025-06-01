use bevy::prelude::*;

use super::{Player, PlayerCamera};

const MOVE_SPEED: f32 = 10.0;

pub fn player_movement(
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
