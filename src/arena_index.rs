use std::collections::HashMap;

use bevy::prelude::*;
use hexx::Hex;

use crate::{
    AppState, GameState,
    arena::{ARENA_RADIUS, Arena},
};

pub struct ArenaIndexPlugin;

impl Plugin for ArenaIndexPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ArenaIndex>()
            .add_systems(
                Update,
                update_arena_hex_and_index
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameState::Running)),
            )
            .add_observer(remove_entity_from_index);
    }
}

#[derive(Resource)]
pub struct ArenaIndex {
    /// A map of hexes to all contained `ArenaHex` entities.
    pub index: HashMap<Hex, Vec<Entity>>,
    /// A map of hexes to their corresponding `Column` entity.
    /// This field is populated when the columns are spawned.
    pub column_index: HashMap<Hex, Entity>,
}

impl Default for ArenaIndex {
    fn default() -> Self {
        let index_iter =
            hexx::shapes::hexagon(Hex::ZERO, ARENA_RADIUS).map(|hex| (hex, Vec::with_capacity(10)));

        Self {
            column_index: HashMap::with_capacity(index_iter.len()),
            index: HashMap::from_iter(index_iter),
        }
    }
}

/// Entities with this component emit events when `hex` changes and they are indexed in
/// `ArenaHexIndex`.
#[derive(Component, Default)]
pub struct ArenaHex {
    pub hex: Hex,
}

/// Triggered on an entity when it moves to an out of bounds hex (outside the arena).
#[derive(Event, Debug)]
pub struct OutOfBoundsEvent {
    pub last_valid_hex: Hex,
    pub out_of_bounds_hex: Hex,
}

fn update_arena_hex_and_index(
    mut commands: Commands,
    arena: Res<Arena>,
    mut arena_index: ResMut<ArenaIndex>,
    q_transform: Query<(Entity, &Transform), (With<ArenaHex>, Changed<Transform>)>,
    mut q_arena_hex: Query<&mut ArenaHex>,
) {
    for (entity, transform) in q_transform {
        let Ok(arena_hex) = q_arena_hex.get(entity) else {
            continue;
        };

        let new_hex = arena.layout.world_pos_to_hex(transform.translation.xz());
        if new_hex != arena_hex.hex {
            // Get a mutable reference to the ArenaHex, we avoid doing this earlier so `Changed`
            // events aren't fired unnecessarily
            let Ok(mut arena_hex) = q_arena_hex.get_mut(entity) else {
                // This should never occur
                continue;
            };

            // Get the index for the new hex and insert the entity
            let Some(new_index) = arena_index.index.get_mut(&new_hex) else {
                // The hex is out of bounds, don't change hex or index it
                commands.entity(entity).trigger(OutOfBoundsEvent {
                    last_valid_hex: arena_hex.hex,
                    out_of_bounds_hex: new_hex,
                });
                continue;
            };
            new_index.push(entity);

            // Try remove the entity from the previous index
            if let Some(old_index) = arena_index.index.get_mut(&arena_hex.hex) {
                if let Some(i) = old_index.iter().position(|e| *e == entity) {
                    old_index.remove(i);
                }
            }

            // Update the hex
            arena_hex.hex = new_hex;
        }
    }
}

fn remove_entity_from_index(
    trigger: Trigger<OnRemove, ArenaHex>,
    mut arena_index: ResMut<ArenaIndex>,
    q_arena_hex: Query<&ArenaHex>,
) -> Result {
    let arena_hex = q_arena_hex.get(trigger.target())?;

    // Try get the hex's index, this should always succeed
    let Some(index) = arena_index.index.get_mut(&arena_hex.hex) else {
        error!(hex=?arena_hex.hex, "Tried removing ArenaHex from ArenaIndex, but hex wasn't found");
        return Ok(());
    };

    // Try get the entity's position/index, if it's not found then there is an indexing issue
    if let Some(i) = index.iter().position(|e| *e == trigger.target()) {
        index.remove(i);
    } else {
        error!(
            hex=?arena_hex.hex,
            entity=?trigger.target(),
            "Tried removing ArenaHex from ArenaIndex, but entity wasn't found in hex's index",
        );
    }

    Ok(())
}
