use bevy::prelude::*;

use crate::{
    GameState,
    enemy::{EnemySet, SpawnEnemiesCommand},
};

pub struct WavePlugin;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_enemies.in_set(EnemySet));
    }
}

fn spawn_enemies(mut commands: Commands) {
    commands.queue(SpawnEnemiesCommand {
        positions: vec![
            Vec3::new(10.0, 0.0, 5.0),
            Vec3::new(-10.0, 0.0, 10.0),
            Vec3::new(-5.0, 0.0, -10.0),
        ],
    });
}
