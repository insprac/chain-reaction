use std::time::Duration;

use bevy::prelude::*;

use crate::{
    AppState, EnemyTeam, GameState, Team,
    arena_index::ArenaHex,
    explosion::{CreateExplosionCommand, ExplosionDamageArea},
    game_assets::GameAssets,
    health::{DamageEvent, DiedEvent, Health},
    player::Player,
};

const MOVE_SPEED: f32 = 5.0;
const COLLISION_DISTANCE: f32 = 0.8;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::InGame), cleanup_enemies)
            .add_systems(
                Update,
                follow_and_self_destruct
                    .in_set(EnemySet)
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameState::Running)),
            );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnemySet;

#[derive(Component)]
#[require(EnemyTeam, ArenaHex, Transform, Visibility)]
pub struct Enemy;

pub struct SpawnEnemyCommand {
    pub position: Vec2,
}

impl SpawnEnemyCommand {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }
}

impl Command for SpawnEnemyCommand {
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

        world
            .spawn((
                Enemy,
                Health::new(1),
                Transform::from_xyz(self.position.x, 1.0, self.position.y),
                Mesh3d(mesh_handle.clone()),
                MeshMaterial3d(material_handle.clone()),
            ))
            .observe(despawn_on_death);
    }
}

fn cleanup_enemies(mut commands: Commands, q_enemy: Query<Entity, With<Enemy>>) {
    for entity in q_enemy {
        commands.entity(entity).try_despawn();
    }
}

fn follow_and_self_destruct(
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
            evw_damage.write(DamageEvent {
                target: enemy_entity,
                damage: 100,
            });
        }
    }

    Ok(())
}

fn despawn_on_death(
    trigger: Trigger<DiedEvent>,
    mut commands: Commands,
    q_transform: Query<&Transform>,
) {
    if let Ok(transform) = q_transform.get(trigger.entity) {
        commands.queue(CreateExplosionCommand {
            team: Team::Enemy,
            color: LinearRgba::new(1.0, 0.0, 0.0, 1.0),
            duration: Duration::from_millis(600),
            position: transform.translation.xz(),
            damage: 1,
            damage_area: ExplosionDamageArea::Radius(1.0),
            damage_delay: Duration::from_millis(200),
            radius: 5.0,
            strength: 50.0,
            strength_modifier: -100.0,
            trigger_history: Vec::new(),
        });
    }
    commands.entity(trigger.entity).try_despawn();
}
