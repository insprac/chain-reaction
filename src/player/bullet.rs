use bevy::prelude::*;
use std::time::Duration;

use crate::{
    arena_index::{ArenaHex, ArenaIndex, OutOfBoundsEvent},
    enemy::Enemy,
    game_assets::GameAssets,
    health::DamageEvent, tower::TriggerTowerEvent,
};

const BULLET_SPEED: f32 = 30.0;
const BULLET_LIFETIME_MILLIS: u64 = 500;
const BULLET_HIT_RADIUS: f32 = 1.0;

#[derive(Component)]
#[require(ArenaHex)]
pub struct PlayerBullet {
    damage: u16,
    timer: Timer,
    /// A list of all towers that have been triggered in this event chain, used to prevent infinite
    /// loops.
    trigger_history: Vec<Entity>,
}

impl Default for PlayerBullet {
    fn default() -> Self {
        PlayerBullet {
            damage: 1,
            timer: Timer::new(
                Duration::from_millis(BULLET_LIFETIME_MILLIS),
                TimerMode::Once,
            ),
            trigger_history: Vec::new(),
        }
    }
}

pub struct SpawnPlayerBulletCommand {
    pub transform: Transform,
    pub trigger_history: Vec<Entity>,
}

impl Command for SpawnPlayerBulletCommand {
    fn apply(self, world: &mut World) {
        let game_assets = world
            .get_resource::<GameAssets>()
            .expect("GameAssets isn't present during SpawnPlayerBulletCommand");

        world
            .spawn((
                PlayerBullet {
                    trigger_history: self.trigger_history,
                    ..default()
                },
                Mesh3d(game_assets.player_bullet_mesh.clone()),
                MeshMaterial3d(game_assets.player_bullet_material.clone()),
                self.transform,
            ))
            .observe(out_of_bounds_observer);
    }
}

pub fn cleanup_bullets(mut commands: Commands, q_bullets: Query<Entity, With<PlayerBullet>>) {
    for entity in q_bullets {
        commands.entity(entity).try_despawn();
    }
}

pub fn update_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut q_bullets: Query<(Entity, &mut PlayerBullet, &mut Transform)>,
) {
    for (entity, mut bullet, mut transform) in q_bullets.iter_mut() {
        // Tick timer, if it's finished despawn and skip
        bullet.timer.tick(time.delta());
        if bullet.timer.finished() {
            commands.entity(entity).try_despawn();
            continue;
        }

        transform.translation =
            transform.translation + transform.forward() * BULLET_SPEED * time.delta_secs();
    }
}

pub fn check_enemy_collision(
    mut commands: Commands,
    mut evw_damage: EventWriter<DamageEvent>,
    q_bullet: Query<(Entity, &PlayerBullet, &Transform), Without<Enemy>>,
    q_enemy: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (bullet_entity, bullet, bullet_trans) in q_bullet {
        for (enemy_entity, enemy_trans) in q_enemy {
            if bullet_trans
                .translation
                .xz()
                .distance(enemy_trans.translation.xz())
                < BULLET_HIT_RADIUS
            {
                evw_damage.write(DamageEvent {
                    target: enemy_entity,
                    damage: bullet.damage,
                });
                commands.entity(bullet_entity).try_despawn();
                break;
            }
        }
    }
}

pub fn check_tower_collision(
    mut commands: Commands,
    arena_index: Res<ArenaIndex>,
    mut evw_trigger_tower: EventWriter<TriggerTowerEvent>,
    mut q_bullet: Query<(Entity, &mut PlayerBullet, &ArenaHex), Changed<ArenaHex>>,
) {
    for (bullet_id, mut bullet, arena_hex) in q_bullet.iter_mut() {
        let Some(tower_id) = arena_index.tower_index.get(&arena_hex.hex) else {
            // There is no tower in this hex
            continue;
        };

        if bullet.trigger_history.contains(tower_id) {
            // Already been triggered by this tower
            continue;
        }

        evw_trigger_tower.write(TriggerTowerEvent {
            target: *tower_id,
            trigger_history: bullet.trigger_history.clone(),
        });

        bullet.trigger_history.push(*tower_id);

        commands.entity(bullet_id).try_despawn();
    }
}

fn out_of_bounds_observer(trigger: Trigger<OutOfBoundsEvent>, mut commands: Commands) {
    // Despawn bullet when it goes out of arena bounds
    commands.entity(trigger.target()).try_despawn();
}
