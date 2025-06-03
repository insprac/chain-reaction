use bevy::prelude::*;

use crate::{
    game_assets::GameAssets,
    health::{DamageEvent, DiedEvent, Health},
    player::Player,
};

const MOVE_SPEED: f32 = 2.0;
const COLLISION_DISTANCE: f32 = 0.8;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, follow_and_self_destruct.in_set(EnemySet));
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
            world
                .spawn((
                    Enemy,
                    Health::new(1),
                    Transform::from_xyz(pos.x, 0.0, pos.z),
                    Mesh3d(mesh_handle.clone()),
                    MeshMaterial3d(material_handle.clone()),
                ))
                .observe(despawn_on_death);
        }
    }
}

fn follow_and_self_destruct(
    mut commands: Commands,
    mut evw_damage: EventWriter<DamageEvent>,
    time: Res<Time>,
    q_player: Query<(Entity, &Transform), (With<Player>, Without<Enemy>)>,
    mut q_enemy: Query<(Entity, &mut Transform), With<Enemy>>,
) -> Result {
    let (player_entity, player_transform) = q_player.single()?;
    let player_pos = player_transform.translation;

    for (enemy_entity, mut enemy_transform) in q_enemy.iter_mut() {
        let y = enemy_transform.translation.y;
        let direction = (player_pos.with_y(y) - enemy_transform.translation).normalize_or_zero();
        enemy_transform.translation += direction * MOVE_SPEED * time.delta_secs();
        enemy_transform.look_at(player_pos.with_y(y), Vec3::Y);
        if player_pos.xz().distance(enemy_transform.translation.xz()) < COLLISION_DISTANCE {
            evw_damage.write(DamageEvent {
                target: player_entity,
                damage: 1,
            });
            commands.entity(enemy_entity).despawn();
        }
    }

    Ok(())
}

fn despawn_on_death(trigger: Trigger<DiedEvent>, mut commands: Commands) {
    commands.entity(trigger.entity).despawn();
}
