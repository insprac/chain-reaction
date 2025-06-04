use bevy::{input::common_conditions::input_pressed, prelude::*};

mod bullet;
mod gun;
mod movement;
mod spawn;

pub use gun::PlayerGun;

use crate::{AppState, GameState};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Setup
            .add_systems(
                OnEnter(AppState::InGame),
                spawn::setup_player.in_set(PlayerSet),
            )
            // Cleanup
            .add_systems(
                OnExit(AppState::InGame),
                (spawn::cleanup_player, bullet::cleanup_bullets).in_set(PlayerSet),
            )
            // Update
            .add_systems(
                Update,
                (
                    movement::player_movement,
                    gun::update_gun_direction.run_if(on_event::<CursorMoved>),
                    gun::update_gun_cooldown,
                    gun::fire_gun.run_if(input_pressed(MouseButton::Left)),
                    bullet::update_bullets,
                    bullet::check_bullet_collision.after(bullet::update_bullets),
                )
                    .in_set(PlayerSet)
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameState::Running)),
            );
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
