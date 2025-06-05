use bevy::{
    asset::RenderAssetUsages,
    input::common_conditions::input_just_pressed,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use hexx::{ColumnMeshBuilder, EdgeDirection, Hex, HexLayout, PlaneMeshBuilder};

use crate::{
    AppState, GameState,
    arena::Arena,
    game_assets::{self, GameAssets},
    pointer_tracking::{PointerMovedEvent, PointerPosition},
};

const GRAPHIC_HEIGHT: f32 = 2.0;

pub const TOWERS: [Tower; 7] = [
    Tower::Bullet2,
    Tower::Bullet3,
    Tower::Bullet4,
    Tower::Bullet6,
    Tower::Explosion1,
    Tower::Explosion2,
    Tower::Explosion3,
];

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TowerPlacementSettings>()
            .add_event::<RedrawPlacementEvent>()
            .add_systems(OnEnter(GameState::PlaceTower), setup_placement)
            .add_systems(OnExit(GameState::PlaceTower), cleanup_placement)
            .add_systems(OnExit(AppState::InGame), cleanup_placement)
            .add_systems(
                Update,
                (
                    update_placement.run_if(on_event::<PointerMovedEvent>),
                    redraw_placement.run_if(on_event::<RedrawPlacementEvent>),
                )
                    .in_set(TowerSet)
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameState::PlaceTower)),
            )
            .add_systems(
                Update,
                start_placing_tower_1
                    .in_set(TowerSet)
                    .run_if(input_just_pressed(KeyCode::Digit1))
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameState::Running).or(in_state(GameState::PlaceTower))),
            );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TowerSet;

/// Keeps track of the tower being placed, and where the player is currently targeting.
#[derive(Resource)]
pub struct TowerPlacementSettings {
    pub tower: Tower,
    pub hex: Hex,
}

impl Default for TowerPlacementSettings {
    fn default() -> Self {
        Self {
            tower: Tower::Bullet2,
            hex: Hex::ZERO,
        }
    }
}

#[derive(Component)]
pub struct TowerPlacementGraphic;

#[derive(Component)]
pub struct HighlightedHex;

/// Fired when the placement graphics need to be redrawn (the focused hex changed).
#[derive(Event)]
pub struct RedrawPlacementEvent {
    pub hex: Hex,
}

/// Towers take up a hex and trigger an effect when hit by a player bullet or explosion.
#[derive(Component, Clone)]
pub enum Tower {
    Bullet2,
    Bullet3,
    Bullet4,
    Bullet6,
    Explosion1,
    Explosion2,
    Explosion3,
}

fn setup_placement(
    mut commands: Commands,
    arena: Res<Arena>,
    placement_settings: Res<TowerPlacementSettings>,
    game_assets: Res<GameAssets>,
    mut evw_redraw_placement: EventWriter<RedrawPlacementEvent>,
) {
    // The world position of the hex
    let world_pos = arena.layout.hex_to_world_pos(placement_settings.hex);

    commands.spawn((
        TowerPlacementGraphic,
        Mesh3d(game_assets.tower_placement_mesh.clone()),
        MeshMaterial3d(game_assets.tower_placement_material.clone()),
        Transform::from_xyz(world_pos.x, GRAPHIC_HEIGHT, world_pos.y),
    ));

    evw_redraw_placement.write(RedrawPlacementEvent {
        hex: placement_settings.hex,
    });
}

fn cleanup_placement(
    mut commands: Commands,
    q_graphic: Query<Entity, With<TowerPlacementGraphic>>,
    q_highlight: Query<Entity, With<HighlightedHex>>,
) {
    for entity in q_graphic {
        commands.entity(entity).despawn();
    }

    for entity in q_highlight {
        commands.entity(entity).despawn();
    }
}

fn start_placing_tower_1(
    mut commands: Commands,
    mut settings: ResMut<TowerPlacementSettings>,
    arena: Res<Arena>,
    pointer_pos: Res<PointerPosition>,
) {
    commands.set_state(GameState::PlaceTower);

    let hex = arena.layout.world_pos_to_hex(pointer_pos.pos.xz());

    settings.tower = TOWERS[0].clone();
    settings.hex = hex;
}

fn update_placement(
    mut settings: ResMut<TowerPlacementSettings>,
    arena: Res<Arena>,
    pointer_pos: Res<PointerPosition>,
    mut evw_redraw_placement: EventWriter<RedrawPlacementEvent>,
) {
    let hex = arena.layout.world_pos_to_hex(pointer_pos.pos.xz());

    if settings.hex != hex {
        settings.hex = hex;
        evw_redraw_placement.write(RedrawPlacementEvent { hex });
    }
}

fn redraw_placement(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    arena: Res<Arena>,
    mut evr_redraw_placement: EventReader<RedrawPlacementEvent>,
    mut graphic_transform: Single<&mut Transform, With<TowerPlacementGraphic>>,
    q_highlight: Query<Entity, With<HighlightedHex>>,
) {
    // We only care about the last event fired, though it should never fire more than once per
    // frame
    let Some(event) = evr_redraw_placement.read().last() else {
        return;
    };

    // Remove existing highlights
    for entity in q_highlight {
        commands.entity(entity).despawn();
    }

    let world_pos = arena.layout.hex_to_world_pos(event.hex);
    graphic_transform.translation = Vec3::new(world_pos.x, GRAPHIC_HEIGHT, world_pos.y);

    // Spawn new highlights in all directions
    for direction in EdgeDirection::ALL_DIRECTIONS {
        let mut hex = event.hex + direction;
        while hex.unsigned_distance_to(Hex::ZERO) <= Arena::RADIUS {
            let position = arena.layout.hex_to_world_pos(hex);
            commands.spawn((
                HighlightedHex,
                Mesh3d(game_assets.hex_plane_mesh.clone()),
                MeshMaterial3d(game_assets.hex_plane_material.clone()),
                Transform::from_xyz(position.x, 0.1, position.y),
            ));
            hex += direction;
        }
    }
}
