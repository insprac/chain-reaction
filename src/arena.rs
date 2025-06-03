use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use hexx::{ColumnMeshBuilder, Hex, HexLayout, HexOrientation};

const COLUMN_HEIGHT: f32 = 20.0;
const ARENA_RADIUS: u32 = 20;
const WALL_DEPTH: u32 = 3;

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
        .add_systems(Startup, spawn_arena);
    }
}

#[derive(Resource)]
pub struct Arena {
    pub layout: HexLayout,
}

impl Arena {
    const RADIUS: u32 = ARENA_RADIUS;
}

#[derive(Component)]
pub struct ArenaColumn {
    pub hex: Hex,
    pub kind: ColumnKind,
}

pub enum ColumnKind {
    Floor,
    Wall,
}

fn spawn_arena(
    mut commands: Commands,
    arena: Res<Arena>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh_handle = meshes.add(hex_column(&arena.layout));
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.0, 0.0),
        perceptual_roughness: 1.0,
        ..default()
    });

    for hex in hexx::shapes::hexagon(Hex::ZERO, ARENA_RADIUS + WALL_DEPTH) {
        let pos = arena.layout.hex_to_world_pos(hex);
        let dist = hex.unsigned_distance_to(Hex::ZERO);
        if dist <= ARENA_RADIUS {
            // Within the arena
            commands.spawn((
                Mesh3d(mesh_handle.clone()),
                MeshMaterial3d(material_handle.clone()),
                Transform::from_xyz(pos.x, rand::random_range(-0.3..0.0), pos.y),
                ArenaColumn {
                    hex,
                    kind: ColumnKind::Floor,
                },
            ));
        } else {
            // Outside the arena (wall)
            commands.spawn((
                Mesh3d(mesh_handle.clone()),
                MeshMaterial3d(material_handle.clone()),
                Transform::from_xyz(pos.x, 4.0 + rand::random_range(0.0..3.0), pos.y),
                ArenaColumn {
                    hex,
                    kind: ColumnKind::Wall,
                },
            ));
        }
    }
}

fn hex_column(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = ColumnMeshBuilder::new(hex_layout, COLUMN_HEIGHT)
        .without_bottom_face()
        .with_offset(Vec3::new(0.0, -COLUMN_HEIGHT, 0.0))
        .center_aligned()
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
