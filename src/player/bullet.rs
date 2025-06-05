use bevy::prelude::*;
use std::time::Duration;

use crate::{enemy::Enemy, health::DamageEvent};

const BULLET_SPEED: f32 = 30.0;
const BULLET_LIFETIME_MILLIS: u64 = 500;
const BULLET_HIT_RADIUS: f32 = 1.0;

#[derive(Component)]
pub struct PlayerBullet {
    damage: u16,
    timer: Timer,
}

impl Default for PlayerBullet {
    fn default() -> Self {
        PlayerBullet {
            damage: 1,
            timer: Timer::new(Duration::from_millis(BULLET_LIFETIME_MILLIS), TimerMode::Once),
        }
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
