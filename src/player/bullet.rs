use bevy::prelude::*;
use std::time::Duration;

use crate::{
    arena_index::{ArenaHex, OutOfBoundsEvent},
    enemy::Enemy,
    game_assets::GameAssets,
    health::DamageEvent,
};

const BULLET_SPEED: f32 = 30.0;
const BULLET_LIFETIME_MILLIS: u64 = 500;
const BULLET_HIT_RADIUS: f32 = 1.0;

#[derive(Component)]
#[require(ArenaHex)]
pub struct PlayerBullet {
    damage: u16,
    timer: Timer,
}

impl Default for PlayerBullet {
    fn default() -> Self {
        PlayerBullet {
            damage: 1,
            timer: Timer::new(
                Duration::from_millis(BULLET_LIFETIME_MILLIS),
                TimerMode::Once,
            ),
        }
    }
}

pub struct CreateBulletCommand {
    pub transform: Transform,
}

impl Command for CreateBulletCommand {
    fn apply(self, world: &mut World) {
        let game_assets = world
            .get_resource::<GameAssets>()
            .expect("GameAssets isn't present during CreateBulletCommand");

        world
            .spawn((
                PlayerBullet::default(),
                Mesh3d(game_assets.player_bullet_mesh.clone()),
                MeshMaterial3d(game_assets.player_bullet_material.clone()),
                self.transform,
            ))
            .observe(
                |trigger: Trigger<OutOfBoundsEvent>, mut commands: Commands| {
                    // Despawn bullet when it goes out of arena bounds
                    commands.entity(trigger.target()).despawn();
                },
            );
    }
}

pub fn cleanup_bullets(mut commands: Commands, q_bullets: Query<Entity, With<PlayerBullet>>) {
    for entity in q_bullets {
        commands.entity(entity).despawn();
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
            commands.entity(entity).despawn();
            continue;
        }

        transform.translation =
            transform.translation + transform.forward() * BULLET_SPEED * time.delta_secs();
    }
}

pub fn check_bullet_collision(
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
                commands.entity(bullet_entity).despawn();
                break;
            }
        }
    }
}
