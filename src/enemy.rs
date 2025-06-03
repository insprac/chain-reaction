use bevy::prelude::*;

use crate::{game_assets::GameAssets, player::Player};

const MOVE_SPEED: f32 = 2.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, follow_player.in_set(EnemySet));
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnemySet;

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct Enemy;

pub struct SpawnEnemiesCommand {
    pub positions: Vec<Vec3>,
}

impl Command for SpawnEnemiesCommand {
    fn apply(self, world: &mut World) -> () {
        let (mesh_handle, material_handle) = {
            let Some(game_assets) = world.get_resource::<GameAssets>() else {
                panic!("GameAssets not available during SpawnEnemiesCommand");
            };
            (
                game_assets.enemy_mesh.clone(),
                game_assets.enemy_material.clone(),
            )
        };

        for pos in self.positions {
            world.spawn((
                Enemy,
                Transform::from_xyz(pos.x, 0.0, pos.z),
                Mesh3d(mesh_handle.clone()),
                MeshMaterial3d(material_handle.clone()),
            ));
        }
    }
}

fn follow_player(
    time: Res<Time>,
    q_player_transform: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut q_enemy_transform: Query<&mut Transform, With<Enemy>>,
) -> Result {
    let player_translation = q_player_transform.single()?.translation;

    for mut enemy_transform in q_enemy_transform.iter_mut() {
        let y = enemy_transform.translation.y;
        let direction =
            (player_translation.with_y(y) - enemy_transform.translation).normalize_or_zero();
        enemy_transform.translation += direction * MOVE_SPEED * time.delta_secs();
        enemy_transform.look_at(player_translation.with_y(y), Vec3::Y);
    }

    Ok(())
}
