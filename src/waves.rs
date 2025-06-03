use bevy::prelude::*;
use hexx::Hex;

use crate::{
    GameState,
    arena::Arena,
    enemy::{EnemySet, SpawnEnemyCommand},
};

pub struct WavePlugin;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_enemies.in_set(EnemySet));
    }
}

fn spawn_enemies(mut commands: Commands, arena: Res<Arena>) {
    let mut hexes: Vec<Hex> = Hex::ZERO.ring(Arena::RADIUS).collect();
    for _ in 0..20 {
        if hexes.len() == 0 {
            break;
        }
        let index = rand::random_range(0..hexes.len() - 1);
        let hex = hexes.swap_remove(index);
        commands.queue(SpawnEnemyCommand::new(arena.layout.hex_to_world_pos(hex)));
    }
}
