use bevy::prelude::*;
use hexx::Hex;

use crate::{
    arena::Arena,
    arena_index::{ArenaHex, ArenaIndex},
};

use super::{Player, PlayerCamera};

const MOVE_SPEED: f32 = 10.0;

pub fn player_movement(
    key_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    arena: Res<Arena>,
    arena_index: Res<ArenaIndex>,
    player: Single<(&mut Transform, &ArenaHex), With<Player>>,
    camera_transform: Single<&Transform, (With<PlayerCamera>, Without<Player>)>,
) {
    let input = Vec2::new(
        -(key_input.pressed(KeyCode::KeyA) as i32 - key_input.pressed(KeyCode::KeyD) as i32) as f32,
        -(key_input.pressed(KeyCode::KeyW) as i32 - key_input.pressed(KeyCode::KeyS) as i32) as f32,
    )
    .normalize_or_zero();

    if input.length() == 0.0 {
        return;
    }

    // Get the player camera and rotate input to be relative to the camera
    let camera_yaw = -camera_transform.rotation.to_euler(EulerRot::YXZ).0;
    let rotated_input = Vec2::new(
        input.x * camera_yaw.cos() - input.y * camera_yaw.sin(),
        input.x * camera_yaw.sin() + input.y * camera_yaw.cos(),
    );

    let (mut player_transform, arena_hex) = player.into_inner();
    // Move the player based on input
    player_transform.translation +=
        Vec3::new(rotated_input.x, 0.0, rotated_input.y) * MOVE_SPEED * time.delta_secs();

    // Keep player within the arena bounds
    let new_hex = arena
        .layout
        .world_pos_to_hex(player_transform.translation.xz());

    if new_hex == arena_hex.hex {
        return;
    }

    if new_hex.unsigned_distance_to(Hex::ZERO) <= Arena::RADIUS {
        // The player is within the arena so movement is valid
        return;
    }

    // The player has tried to leave the arena
    let old_hex_pos = arena.layout.hex_to_world_pos(arena_hex.hex);
    let direction = Vec3::new(old_hex_pos.x, player_transform.translation.y, old_hex_pos.y)
        - player_transform.translation;
    let length = direction.length() - 1.0;
    player_transform.translation += direction.normalize() * length;
}
