use bevy::prelude::*;
use hexx::{EdgeDirection, Hex};

use crate::{arena::Arena, arena_index::ArenaIndex, game_assets::GameAssets};

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TowerSet;

/// Towers take up a hex and trigger an effect when hit by a player bullet or explosion.
#[derive(Component)]
pub struct Tower {
    pub kind: TowerKind,
    pub direction: EdgeDirection,
}

#[derive(Debug, Clone)]
pub enum TowerKind {
    Bullet2,
    Bullet3,
    Bullet4,
    Bullet6,
    Explosion1,
    Explosion2,
}

pub struct PlaceTowerCommand {
    pub tower: Tower,
    pub hex: Hex,
}

impl Command for PlaceTowerCommand {
    fn apply(self, world: &mut World) {
        // Ensure the hex isn't already occupied
        let is_occupied = {
            let arena_index = world.get_resource::<ArenaIndex>().unwrap();
            arena_index.is_occupied(&self.hex)
        };
        if is_occupied {
            warn!(hex=?self.hex, "Hex is occupied, tower can't be placed");
            return;
        }

        // Get the world position of the hex
        let world_pos = {
            let arena = world.get_resource::<Arena>().unwrap();
            arena.layout.hex_to_world_pos(self.hex)
        };

        let (mesh_handle, material_handle) = {
            let game_assets = world.get_resource::<GameAssets>().unwrap();
            (
                game_assets.tower_mesh.clone(),
                game_assets.tower_material.clone(),
            )
        };

        // Spawn the tower
        let id = world
            .spawn((
                self.tower,
                Transform::from_xyz(world_pos.x, 0.0, world_pos.y),
                Mesh3d(mesh_handle),
                MeshMaterial3d(material_handle),
            ))
            .id();

        // Update the index to ensure no other towers are built here
        let mut arena_index = world.get_resource_mut::<ArenaIndex>().unwrap();
        arena_index.tower_index.insert(self.hex, id);
    }
}
