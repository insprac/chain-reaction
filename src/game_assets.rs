use bevy::{
    asset::RenderAssetUsages,
    pbr::ExtendedMaterial,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use hexx::{ColumnMeshBuilder, HexLayout, PlaneMeshBuilder};

use crate::{
    arena::Arena,
    materials::{BulletMaterial, TowerMaterial},
};

pub struct GameAssetPlugin;

impl Plugin for GameAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_assets);
    }
}

#[derive(Resource)]
pub struct GameAssets {
    pub enemy_mesh: Handle<Mesh>,
    pub enemy_material: Handle<StandardMaterial>,
    pub player_bullet_mesh: Handle<Mesh>,
    pub player_bullet_material: Handle<BulletMaterial>,
    pub hex_plane_mesh: Handle<Mesh>,
    pub hex_plane_material: Handle<StandardMaterial>,
    pub tower_placeholder_mesh: Handle<Mesh>,
    pub tower_placeholder_material: Handle<StandardMaterial>,
    pub tower_mesh: Handle<Mesh>,
    pub tower_bullet2_material: Handle<ExtendedMaterial<StandardMaterial, TowerMaterial>>,
    pub tower_bullet3_material: Handle<ExtendedMaterial<StandardMaterial, TowerMaterial>>,
    pub tower_bullet4_material: Handle<ExtendedMaterial<StandardMaterial, TowerMaterial>>,
    pub tower_bullet6_material: Handle<ExtendedMaterial<StandardMaterial, TowerMaterial>>,
}

fn load_assets(
    mut commands: Commands,
    arena: Res<Arena>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut bullet_materials: ResMut<Assets<BulletMaterial>>,
    mut tower_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, TowerMaterial>>>,
) {
    let enemy_mesh = meshes.add(Cuboid::new(0.5, 0.3, 0.5));
    let enemy_material = materials.add(StandardMaterial {
        base_color: Color::hsl(350.0, 1.0, 0.5),
        perceptual_roughness: 1.0,
        unlit: true,
        ..default()
    });

    let player_bullet_mesh = meshes.add(Plane3d::new(Vec3::Y, Vec2::new(0.1, 1.0)));
    let player_bullet_material = bullet_materials.add(BulletMaterial {
        color: LinearRgba::new(0.2, 0.8, 0.2, 1.0),
    });

    let hex_plane_mesh = meshes.add(build_hex_plane(&arena.layout));
    let hex_plane_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.5, 0.5, 0.8, 0.1),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    let tower_placeholder_mesh = meshes.add(build_tower_placeholder_mesh(&arena.layout));
    let tower_placeholder_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.5, 0.5, 0.8),
        unlit: true,
        ..default()
    });

    let tower_mesh = meshes.add(build_tower_mesh(&arena.layout));
    let base_tower_material = StandardMaterial {
        base_color: Color::srgb(0.0, 0.1, 0.0),
        perceptual_roughness: 1.0,
        ..default()
    };
    let tower_bullet2_image: Handle<Image> = asset_server.load("textures/bullet2.png");
    let tower_bullet3_image: Handle<Image> = asset_server.load("textures/bullet3.png");
    let tower_bullet4_image: Handle<Image> = asset_server.load("textures/bullet4.png");
    let tower_bullet6_image: Handle<Image> = asset_server.load("textures/bullet6.png");
    let tower_bullet2_material = tower_materials.add(ExtendedMaterial {
        base: base_tower_material.clone(),
        extension: TowerMaterial {
            texture: tower_bullet2_image,
        },
    });
    let tower_bullet3_material = tower_materials.add(ExtendedMaterial {
        base: base_tower_material.clone(),
        extension: TowerMaterial {
            texture: tower_bullet3_image,
        },
    });
    let tower_bullet4_material = tower_materials.add(ExtendedMaterial {
        base: base_tower_material.clone(),
        extension: TowerMaterial {
            texture: tower_bullet4_image,
        },
    });
    let tower_bullet6_material = tower_materials.add(ExtendedMaterial {
        base: base_tower_material,
        extension: TowerMaterial {
            texture: tower_bullet6_image,
        },
    });

    commands.insert_resource(GameAssets {
        enemy_mesh,
        enemy_material,
        player_bullet_mesh,
        player_bullet_material,
        hex_plane_mesh,
        hex_plane_material,
        tower_placeholder_mesh,
        tower_placeholder_material,
        tower_mesh,
        tower_bullet2_material,
        tower_bullet3_material,
        tower_bullet4_material,
        tower_bullet6_material,
    });
}

fn build_hex_plane(layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(layout)
        .with_scale(Vec3::splat(0.9))
        .build();

    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
        .with_inserted_indices(Indices::U16(mesh_info.indices))
}

pub fn build_tower_placeholder_mesh(hex_layout: &HexLayout) -> Mesh {
    let height = 2.0;
    let mesh_info = ColumnMeshBuilder::new(hex_layout, height)
        .without_bottom_face()
        .with_offset(Vec3::new(0.0, -height, 0.0))
        .with_scale(Vec3::new(0.9, 1.0, 0.9))
        .build();

    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
        .with_inserted_indices(Indices::U16(mesh_info.indices))
}

pub fn build_tower_mesh(hex_layout: &HexLayout) -> Mesh {
    let height = 2.0;
    let mesh_info = ColumnMeshBuilder::new(hex_layout, height)
        .without_bottom_face()
        .with_scale(Vec3::new(0.8, 1.0, 0.8))
        .build();

    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
        .with_inserted_indices(Indices::U16(mesh_info.indices))
}
