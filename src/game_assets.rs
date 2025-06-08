use bevy::{
    asset::RenderAssetUsages,
    pbr::ExtendedMaterial,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use hexx::{ColumnMeshBuilder, HexLayout, PlaneMeshBuilder};

use crate::{
    arena::Arena,
    materials::{BulletMaterial, TowerMaterial, TowerPlaceholderMaterial},
    tower::TowerKind,
};

pub struct GameAssetPlugin;

impl Plugin for GameAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_assets);
    }
}

#[derive(Resource)]
pub struct GameAssets {
    pub audiowide_font: Handle<Font>,

    pub enemy_mesh: Handle<Mesh>,
    pub enemy_material: Handle<StandardMaterial>,

    pub player_bullet_mesh: Handle<Mesh>,
    pub player_bullet_material: Handle<BulletMaterial>,

    pub hex_plane_mesh: Handle<Mesh>,
    pub hex_plane_material: Handle<StandardMaterial>,

    pub tower_mesh: Handle<Mesh>,
    pub tower_materials: TowerAssets<ExtendedMaterial<StandardMaterial, TowerMaterial>>,

    pub tower_placeholder_mesh: Handle<Mesh>,
    pub tower_placeholder_materials: TowerAssets<TowerPlaceholderMaterial>,
    pub tower_placeholder_empty_material: Handle<TowerPlaceholderMaterial>,

    pub tower_icons: TowerAssets<Image>,
}

pub struct TowerAssets<T: Asset> {
    pub bullet2: Handle<T>,
    pub bullet3: Handle<T>,
    pub bullet4: Handle<T>,
    pub bullet6: Handle<T>,
    pub explosion1: Handle<T>,
    pub explosion2: Handle<T>,
    pub explosion3: Handle<T>,
}

impl<T: Asset> TowerAssets<T> {
    pub fn get(&self, kind: &TowerKind) -> Handle<T> {
        match *kind {
            TowerKind::Bullet2 => self.bullet2.clone(),
            TowerKind::Bullet3 => self.bullet3.clone(),
            TowerKind::Bullet4 => self.bullet4.clone(),
            TowerKind::Bullet6 => self.bullet6.clone(),
            TowerKind::Explosion1 => self.explosion1.clone(),
            TowerKind::Explosion2 => self.explosion2.clone(),
            TowerKind::Explosion3 => self.explosion3.clone(),
        }
    }
}

fn load_assets(
    mut commands: Commands,
    arena: Res<Arena>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut bullet_materials: ResMut<Assets<BulletMaterial>>,
    mut tower_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, TowerMaterial>>>,
    mut tower_placeholder_materials: ResMut<Assets<TowerPlaceholderMaterial>>,
) {
    let audiowide_font = asset_server.load("fonts/Audiowide-Regular.ttf");
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

    // Tower images
    let tower_empty_image: Handle<Image> = asset_server.load("textures/empty.png");
    let tower_bullet2_image: Handle<Image> = asset_server.load("textures/bullet2.png");
    let tower_bullet3_image: Handle<Image> = asset_server.load("textures/bullet3.png");
    let tower_bullet4_image: Handle<Image> = asset_server.load("textures/bullet4.png");
    let tower_bullet6_image: Handle<Image> = asset_server.load("textures/bullet6.png");
    let tower_explosion1_image: Handle<Image> = asset_server.load("textures/explosion1.png");
    let tower_explosion2_image: Handle<Image> = asset_server.load("textures/explosion2.png");
    let tower_explosion3_image: Handle<Image> = asset_server.load("textures/explosion3.png");

    let tower_mesh = meshes.add(build_tower_mesh(&arena.layout));
    let base_tower_material = StandardMaterial {
        base_color: Color::srgb(0.0, 0.1, 0.0),
        perceptual_roughness: 1.0,
        ..default()
    };

    let tower_materials = TowerAssets {
        bullet2: tower_materials.add(ExtendedMaterial {
            base: base_tower_material.clone(),
            extension: TowerMaterial {
                texture: tower_bullet2_image.clone(),
            },
        }),
        bullet3: tower_materials.add(ExtendedMaterial {
            base: base_tower_material.clone(),
            extension: TowerMaterial {
                texture: tower_bullet3_image.clone(),
            },
        }),
        bullet4: tower_materials.add(ExtendedMaterial {
            base: base_tower_material.clone(),
            extension: TowerMaterial {
                texture: tower_bullet4_image.clone(),
            },
        }),
        bullet6: tower_materials.add(ExtendedMaterial {
            base: base_tower_material.clone(),
            extension: TowerMaterial {
                texture: tower_bullet6_image.clone(),
            },
        }),
        explosion1: tower_materials.add(ExtendedMaterial {
            base: base_tower_material.clone(),
            extension: TowerMaterial {
                texture: tower_explosion1_image.clone(),
            },
        }),
        explosion2: tower_materials.add(ExtendedMaterial {
            base: base_tower_material.clone(),
            extension: TowerMaterial {
                texture: tower_explosion2_image.clone(),
            },
        }),
        explosion3: tower_materials.add(ExtendedMaterial {
            base: base_tower_material,
            extension: TowerMaterial {
                texture: tower_explosion3_image.clone(),
            },
        }),
    };

    let tower_placeholder_mesh = meshes.add(build_tower_placeholder_mesh(&arena.layout));
    let tower_placeholder_empty_material =
        tower_placeholder_materials.add(TowerPlaceholderMaterial {
            texture: tower_empty_image,
        });

    let tower_placeholder_materials = TowerAssets {
        bullet2: tower_placeholder_materials.add(TowerPlaceholderMaterial {
            texture: tower_bullet2_image,
        }),
        bullet3: tower_placeholder_materials.add(TowerPlaceholderMaterial {
            texture: tower_bullet3_image,
        }),
        bullet4: tower_placeholder_materials.add(TowerPlaceholderMaterial {
            texture: tower_bullet4_image,
        }),
        bullet6: tower_placeholder_materials.add(TowerPlaceholderMaterial {
            texture: tower_bullet6_image,
        }),
        explosion1: tower_placeholder_materials.add(TowerPlaceholderMaterial {
            texture: tower_explosion1_image,
        }),
        explosion2: tower_placeholder_materials.add(TowerPlaceholderMaterial {
            texture: tower_explosion2_image,
        }),
        explosion3: tower_placeholder_materials.add(TowerPlaceholderMaterial {
            texture: tower_explosion3_image,
        }),
    };

    // Tower icons
    let tower_bullet2_icon: Handle<Image> = asset_server.load("icons/bullet2.png");
    let tower_bullet3_icon: Handle<Image> = asset_server.load("icons/bullet3.png");
    let tower_bullet4_icon: Handle<Image> = asset_server.load("icons/bullet4.png");
    let tower_bullet6_icon: Handle<Image> = asset_server.load("icons/bullet6.png");
    let tower_explosion1_icon: Handle<Image> = asset_server.load("icons/explosion1.png");
    let tower_explosion2_icon: Handle<Image> = asset_server.load("icons/explosion2.png");
    let tower_explosion3_icon: Handle<Image> = asset_server.load("icons/explosion3.png");

    let tower_icons = TowerAssets {
        bullet2: tower_bullet2_icon,
        bullet3: tower_bullet3_icon,
        bullet4: tower_bullet4_icon,
        bullet6: tower_bullet6_icon,
        explosion1: tower_explosion1_icon,
        explosion2: tower_explosion2_icon,
        explosion3: tower_explosion3_icon,
    };

    commands.insert_resource(GameAssets {
        audiowide_font,
        enemy_mesh,
        enemy_material,
        player_bullet_mesh,
        player_bullet_material,
        hex_plane_mesh,
        hex_plane_material,
        tower_mesh,
        tower_materials,
        tower_placeholder_mesh,
        tower_placeholder_empty_material,
        tower_placeholder_materials,
        tower_icons,
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
