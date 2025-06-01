use bevy::prelude::*;

mod spawn;
mod movement;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn::spawn_player.in_set(PlayerSet))
            .add_systems(Update, movement::player_movement.in_set(PlayerSet));
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
