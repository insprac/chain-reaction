use std::f32::consts::PI;

use bevy::prelude::*;
use hexx::{EdgeDirection, Hex};

use crate::{
    AppState, GameState, arena::Arena, arena_index::ArenaIndex, game_assets::GameAssets,
    player::SpawnPlayerBulletCommand,
};

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TriggerTowerEvent>().add_systems(
            Update,
            trigger_towers
                .in_set(TowerSet)
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameState::Running)),
        );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TowerSet;

/// Towers take up a hex and trigger an effect when hit by a player bullet or explosion.
#[derive(Component)]
pub struct Tower {
    pub kind: TowerKind,
    /// The rotation offset of the `EdgeDirection`, equivalent to `EdgeDirection >> rotation`.
    pub rotation: u8,
}

#[derive(Debug, Clone)]
pub enum TowerKind {
    Bullet2,
    Bullet3,
    Bullet4,
    Bullet6,
    Explosion1,
    Explosion2,
    Explosion3,
}

impl TowerKind {
    pub fn actions(&self) -> Vec<TowerAction> {
        match *self {
            TowerKind::Bullet2 => vec![
                TowerAction::Shoot(EdgeDirection::FLAT_TOP),
                TowerAction::Shoot(EdgeDirection::FLAT_BOTTOM),
            ],
            TowerKind::Bullet3 => vec![
                TowerAction::Shoot(EdgeDirection::FLAT_TOP),
                TowerAction::Shoot(EdgeDirection::FLAT_BOTTOM_LEFT),
                TowerAction::Shoot(EdgeDirection::FLAT_BOTTOM_RIGHT),
            ],
            TowerKind::Bullet4 => vec![
                TowerAction::Shoot(EdgeDirection::FLAT_TOP_LEFT),
                TowerAction::Shoot(EdgeDirection::FLAT_TOP_RIGHT),
                TowerAction::Shoot(EdgeDirection::FLAT_BOTTOM_LEFT),
                TowerAction::Shoot(EdgeDirection::FLAT_BOTTOM_RIGHT),
            ],
            TowerKind::Bullet6 => vec![
                TowerAction::Shoot(EdgeDirection::FLAT_TOP),
                TowerAction::Shoot(EdgeDirection::FLAT_BOTTOM),
                TowerAction::Shoot(EdgeDirection::FLAT_TOP_LEFT),
                TowerAction::Shoot(EdgeDirection::FLAT_TOP_RIGHT),
                TowerAction::Shoot(EdgeDirection::FLAT_BOTTOM_LEFT),
                TowerAction::Shoot(EdgeDirection::FLAT_BOTTOM_RIGHT),
            ],
            TowerKind::Explosion1 => vec![TowerAction::Explode(1)],
            TowerKind::Explosion2 => vec![TowerAction::Explode(2)],
            TowerKind::Explosion3 => vec![TowerAction::Explode(3)],
        }
    }
}

/// Emit to trigger a tower's effect such as shoot, explode, etc.
#[derive(Event)]
pub struct TriggerTowerEvent {
    /// The tower's ID.
    pub target: Entity,
    /// A list of all previous triggers.
    pub trigger_history: Vec<Entity>,
}

pub enum TowerAction {
    Shoot(EdgeDirection),
    Explode(u32),
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
                game_assets.tower_materials.get(&self.tower.kind),
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

fn trigger_towers(
    mut commands: Commands,
    mut evr_trigger_tower: EventReader<TriggerTowerEvent>,
    q_tower: Query<(&Tower, &Transform)>,
) {
    for event in evr_trigger_tower.read() {
        let Ok((tower, tower_transform)) = q_tower.get(event.target) else {
            warn!(tower_id=?event.target, "Tower triggered targeting an entity that is not a tower");
            continue;
        };

        let mut trigger_history = event.trigger_history.clone();
        trigger_history.push(event.target);

        for action in tower.kind.actions() {
            match action {
                TowerAction::Shoot(direction) => {
                    let direction = direction >> tower.rotation;
                    let transform = Transform::from_translation(tower_transform.translation)
                        .with_rotation(Quat::from_axis_angle(
                            Vec3::Y,
                            direction.angle_flat() + PI / 2.0,
                        ));

                    commands.queue(SpawnPlayerBulletCommand {
                        transform,
                        trigger_history: trigger_history.clone(),
                    });
                }
                TowerAction::Explode(radius) => {
                    unimplemented!()
                }
            }
        }
    }
}
