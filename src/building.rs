use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use hexx::Hex;

use crate::{
    AppState, GameState,
    arena::Arena,
    arena_index::ArenaIndex,
    game_assets::GameAssets,
    pointer_tracking::{PointerChangedHexEvent, PointerPosition},
    tower::{PlaceTowerCommand, Tower, TowerAction, TowerKind},
};

const PLACEHOLDER_HEIGHT: f32 = 2.0;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BuildingSettings>()
            .add_event::<RedrawPlacementEvent>()
            .add_event::<BuildingsUpdatedEvent>()
            .add_systems(OnEnter(GameState::Building), setup_building)
            .add_systems(OnExit(GameState::Building), cleanup_building)
            .add_systems(OnExit(AppState::InGame), cleanup_building)
            .add_systems(
                Update,
                select_building
                    .in_set(BuildingSet)
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(
                Update,
                (
                    cancel_building.run_if(input_just_pressed(KeyCode::Escape)),
                    select_building,
                    place_building.run_if(input_just_pressed(MouseButton::Left)),
                    redraw_placement.run_if(
                        on_event::<RedrawPlacementEvent>.or(on_event::<PointerChangedHexEvent>),
                    ),
                )
                    .in_set(BuildingSet)
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameState::Building)),
            );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct BuildingSet;

/// Keeps track of the collected towers, and the tower being built.
#[derive(Resource)]
pub struct BuildingSettings {
    /// All the player's collected towers that can be built.
    pub towers: Vec<TowerKind>,
    /// Index of the tower currently being built, when None no tower is currently being built.
    pub selected_tower: Option<usize>,
}

impl BuildingSettings {
    pub fn get_selected(&self) -> Option<TowerKind> {
        self.selected_tower
            .map(|i| self.towers.get(i).map(Clone::clone))
            .flatten()
    }
}

impl Default for BuildingSettings {
    fn default() -> Self {
        BuildingSettings {
            towers: vec![],
            selected_tower: None,
        }
    }
}

/// A ghost image of where the tower would be built.
#[derive(Component)]
pub struct BuildingPlaceholder;

/// A highlighted hex hinting at area of effect for the building.
#[derive(Component)]
pub struct HighlightedHex;

/// Fired when the placeholder graphics need to be redrawn (the focused hex changed).
#[derive(Event)]
pub struct RedrawPlacementEvent;

#[derive(Event)]
pub struct BuildingsUpdatedEvent;

pub struct AddTowerCommand {
    kind: TowerKind,
}

impl Command for AddTowerCommand {
    fn apply(self, world: &mut World) -> () {
        let mut settings = world.get_resource_mut::<BuildingSettings>().unwrap();
        settings.towers.push(self.kind);
        world.send_event(BuildingsUpdatedEvent);
    }
}

fn setup_building(
    mut commands: Commands,
    arena: Res<Arena>,
    game_assets: Res<GameAssets>,
    pointer_pos: Res<PointerPosition>,
    settings: Res<BuildingSettings>,
    mut evw_redraw_placement: EventWriter<RedrawPlacementEvent>,
) {
    let world_pos = arena.layout.hex_to_world_pos(pointer_pos.hex);

    let material = if let Some(kind) = settings.get_selected() {
        game_assets.tower_placeholder_materials.get(&kind)
    } else {
        game_assets.tower_placeholder_empty_material.clone()
    };

    commands.spawn((
        BuildingPlaceholder,
        Mesh3d(game_assets.tower_placeholder_mesh.clone()),
        MeshMaterial3d(material),
        Transform::from_xyz(world_pos.x, PLACEHOLDER_HEIGHT, world_pos.y),
    ));

    evw_redraw_placement.write(RedrawPlacementEvent);
}

fn cleanup_building(
    mut commands: Commands,
    q_graphic: Query<Entity, With<BuildingPlaceholder>>,
    q_highlight: Query<Entity, With<HighlightedHex>>,
) {
    for entity in q_graphic {
        commands.entity(entity).try_despawn();
    }

    for entity in q_highlight {
        commands.entity(entity).try_despawn();
    }
}

fn cancel_building(mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::Running);
}

fn place_building(
    mut commands: Commands,
    mut settings: ResMut<BuildingSettings>,
    mut next_state: ResMut<NextState<GameState>>,
    arena_index: Res<ArenaIndex>,
    pointer_pos: Res<PointerPosition>,
    mut evw_buildings_updated: EventWriter<BuildingsUpdatedEvent>,
) {
    if arena_index.is_occupied(&pointer_pos.hex) {
        return;
    }

    let Some(selected_tower) = settings.selected_tower else {
        return;
    };

    let Some(tower_kind) = settings.towers.get(selected_tower) else {
        return;
    };

    commands.queue(PlaceTowerCommand {
        hex: pointer_pos.hex,
        tower: Tower {
            kind: tower_kind.clone(),
            rotation: 0,
        },
    });

    // Remove the tower from the player's collection.
    settings.towers.remove(selected_tower);
    settings.selected_tower = None;

    evw_buildings_updated.write(BuildingsUpdatedEvent);

    // Return to playing the game
    next_state.set(GameState::Running);
}

fn select_building(
    mut commands: Commands,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut settings: ResMut<BuildingSettings>,
    key_input: Res<ButtonInput<KeyCode>>,
    game_assets: Res<GameAssets>,
    q_placeholder: Query<Entity, With<BuildingPlaceholder>>,
    mut evw_redraw_placement: EventWriter<RedrawPlacementEvent>,
) {
    let key_codes = [
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
    ];

    for (index, key_code) in key_codes.iter().enumerate() {
        if !key_input.just_pressed(*key_code) {
            continue;
        }

        evw_redraw_placement.write(RedrawPlacementEvent);

        let Some(kind) = settings.towers.get(index) else {
            continue;
        };

        if let Ok(placeholder_id) = q_placeholder.single() {
            let material = game_assets.tower_placeholder_materials.get(kind);
            commands
                .entity(placeholder_id)
                .insert(MeshMaterial3d(material));
        }

        settings.selected_tower = Some(index);

        if *state.get() != GameState::Building {
            next_state.set(GameState::Building);
        }

        break;
    }
}

fn redraw_placement(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    arena: Res<Arena>,
    pointer_pos: Res<PointerPosition>,
    settings: Res<BuildingSettings>,
    mut graphic_transform: Single<&mut Transform, With<BuildingPlaceholder>>,
    q_highlight: Query<Entity, With<HighlightedHex>>,
) {
    // Remove existing highlights
    for entity in q_highlight {
        commands.entity(entity).try_despawn();
    }

    let world_pos = arena.layout.hex_to_world_pos(pointer_pos.hex);
    graphic_transform.translation = Vec3::new(world_pos.x, PLACEHOLDER_HEIGHT, world_pos.y);

    let Some(kind) = settings.get_selected() else {
        return;
    };

    for action in kind.actions() {
        match action {
            TowerAction::Shoot(direction) => {
                let mut hex = pointer_pos.hex + direction;
                while hex.unsigned_distance_to(Hex::ZERO) <= Arena::RADIUS {
                    commands.spawn(highlighted_hex_bundle(hex, &arena, &game_assets));
                    hex += direction;
                }
            }
            TowerAction::Explode(range) => {
                for hex in pointer_pos.hex.range(range) {
                    if hex.unsigned_distance_to(Hex::ZERO) <= Arena::RADIUS {
                        commands.spawn(highlighted_hex_bundle(hex, &arena, &game_assets));
                    }
                }
            }
        }
    }
}

fn highlighted_hex_bundle(hex: Hex, arena: &Arena, game_assets: &GameAssets) -> impl Bundle {
    let position = arena.layout.hex_to_world_pos(hex);
    (
        HighlightedHex,
        Mesh3d(game_assets.hex_plane_mesh.clone()),
        MeshMaterial3d(game_assets.hex_plane_material.clone()),
        Transform::from_xyz(position.x, 0.1, position.y),
    )
}
