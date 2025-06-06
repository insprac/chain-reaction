use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use hexx::{ColumnMeshBuilder, Hex, HexLayout, HexOrientation};

use crate::{
    AppState, GameState,
    arena_index::ArenaIndex,
    force::{Force, ForceReceiver},
};

pub const ARENA_COLUMN_HEIGHT: f32 = 100.0;
pub const ARENA_RADIUS: u32 = 20;
pub const ARENA_WALL_DEPTH: u32 = 3;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Arena {
            layout: HexLayout {
                orientation: HexOrientation::Flat,
                scale: Vec2::new(1.0, 1.0),
                ..default()
            },
        })
        .add_systems(OnEnter(AppState::InGame), setup_arena)
        .add_systems(OnExit(AppState::InGame), cleanup_arena)
        .add_systems(
            Update,
            move_column_with_force
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameState::Running)),
        );
    }
}

#[derive(Resource)]
pub struct Arena {
    pub layout: HexLayout,
}

impl Arena {
    pub const RADIUS: u32 = ARENA_RADIUS;
}

#[derive(Component)]
pub struct ArenaColumn {
    /// The column's hex coordinates.
    pub hex: Hex,
    /// The height offset of the column, this gives the floor a varied look.
    pub offset: f32,
    /// The type of column, determines how it should be interacted with.
    pub kind: ColumnKind,
}

pub enum ColumnKind {
    Floor,
    Wall,
}

fn setup_arena(
    mut commands: Commands,
    arena: Res<Arena>,
    mut arena_index: ResMut<ArenaIndex>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh_handle = meshes.add(hex_column_mesh(&arena.layout, ARENA_COLUMN_HEIGHT));
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.0, 0.0),
        perceptual_roughness: 1.0,
        ..default()
    });

    for hex in hexx::shapes::hexagon(Hex::ZERO, ARENA_RADIUS + ARENA_WALL_DEPTH) {
        let pos = arena.layout.hex_to_world_pos(hex);
        let dist = hex.unsigned_distance_to(Hex::ZERO);
        if dist <= ARENA_RADIUS {
            // Within the arena
            let offset = rand::random_range(-0.3..0.0);
            let id = commands
                .spawn((
                    Mesh3d(mesh_handle.clone()),
                    MeshMaterial3d(material_handle.clone()),
                    Transform::from_xyz(pos.x, offset, pos.y),
                    ArenaColumn {
                        hex,
                        offset,
                        kind: ColumnKind::Floor,
                    },
                    ForceReceiver {
                        restitution_coefficient: 5.0,
                    },
                ))
                .id();
            // Insert the column into the arena index
            arena_index.column_index.insert(hex, id);
        } else {
            // Outside the arena (wall)
            let offset = 4.0 + rand::random_range(0.0..3.0);
            commands.spawn((
                Mesh3d(mesh_handle.clone()),
                MeshMaterial3d(material_handle.clone()),
                Transform::from_xyz(pos.x, offset, pos.y),
                ArenaColumn {
                    hex,
                    offset,
                    kind: ColumnKind::Wall,
                },
            ));
        }
    }
}

fn cleanup_arena(mut commands: Commands, q_arena_columns: Query<Entity, With<ArenaColumn>>) {
    for entity in q_arena_columns {
        commands.entity(entity).try_despawn();
    }
}

fn move_column_with_force(
    mut q_arena_columns: Query<(&ArenaColumn, &Force, &mut Transform), Changed<Force>>,
) {
    for (column, force, mut transform) in q_arena_columns.iter_mut() {
        transform.translation.y = column.offset - force.force;
    }
}

pub fn hex_column_mesh(hex_layout: &HexLayout, height: f32) -> Mesh {
    let mesh_info = ColumnMeshBuilder::new(hex_layout, height)
        .without_bottom_face()
        .with_offset(Vec3::new(0.0, -height, 0.0))
        .build();

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}
