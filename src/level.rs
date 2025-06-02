use bevy::prelude::*;

use crate::{
    hex_grid::{HexCoord, HexLayout},
    player::{Player, PlayerSet},
};

const GRID_SIZE: usize = 100;

const TILE_SIZE: f32 = 1.0;
const TILE_HEIGHT: f32 = 10.0;
const TILE_HALF_HEIGHT: f32 = TILE_HEIGHT / 2.0;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_level)
            .add_systems(Update, update_tile_height.after(PlayerSet));
    }
}

#[derive(Component)]
#[require(Visibility)]
pub struct Level;

#[derive(Component)]
struct Tile;

fn spawn_level(
    mut commands: Commands,
    layout: Res<HexLayout>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh_handle: Handle<Mesh> = asset_server.load(
        GltfAssetLabel::Primitive {
            mesh: 0,
            primitive: 0,
        }
        .from_asset("hexagon.glb"),
    );

    let material_handle = materials.add(StandardMaterial {
        base_color: Color::hsl(0.0, 0.0, 0.0),
        perceptual_roughness: 1.0,
        ..default()
    });

    let hexes_to_spawn = HexCoord::origin().filled_disk(20);
    for hex_coord in hexes_to_spawn.iter() {
        let dist = hex_coord.distance(&HexCoord::origin());
        let height = if dist >= 18 {
            5.0 + rand::random_range(-1.0..3.0)
        } else {
            rand::random_range(-0.3..0.0)
        };
        let world_pos = layout.hex_to_world_3d(*hex_coord).with_y(height);

        commands.spawn((
            Mesh3d(mesh_handle.clone()),
            MeshMaterial3d(material_handle.clone()),
            Transform::from_translation(world_pos).with_scale(Vec3::ONE.with_y(10.0)),
            *hex_coord,
            Name::new(format!("Hex ({},{})", hex_coord.q, hex_coord.r)),
        ));
    }
}

fn update_tile_height(
    q_player_transform: Query<&Transform, With<Player>>,
    mut q_tile_transform: Query<&mut Transform, (With<Tile>, Without<Player>)>,
) -> Result {
    let player_transform = q_player_transform.single()?;
    let player_xz = player_transform.translation.xz();
    for mut tile_transform in q_tile_transform.iter_mut() {
        let distance = tile_transform.translation.xz().distance(player_xz);
        tile_transform.translation.y = -TILE_HALF_HEIGHT + distance.max(0.3).log10() - 2.0;
    }
    Ok(())
}
