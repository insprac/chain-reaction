use bevy::prelude::*;

use crate::player::{Player, PlayerSet};

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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh_handle = meshes.add(Cuboid::new(TILE_SIZE, TILE_HEIGHT, TILE_SIZE));
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::hsl(0.0, 0.0, 0.0),
        perceptual_roughness: 1.0,
        ..default()
    });

    let half_grid_size = GRID_SIZE as f32 / 2.0;
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            commands.spawn((
                Tile,
                Mesh3d(mesh_handle.clone()),
                MeshMaterial3d(material_handle.clone()),
                Transform::from_xyz(
                    x as f32 * TILE_SIZE - half_grid_size,
                    TILE_HEIGHT * -0.5,
                    y as f32 * TILE_SIZE - half_grid_size,
                ),
            ));
        }
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
        tile_transform.translation.y = -TILE_HALF_HEIGHT - (1.0 - distance.max(1.0).log10());
    }
    Ok(())
}
