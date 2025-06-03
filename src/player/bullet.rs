use std::time::Duration;

use bevy::prelude::*;

const BULLET_SPEED: f32 = 30.0;
const BULLET_LIFETIME_SECS: u64 = 3;

#[derive(Component)]
pub struct PlayerBullet {
    timer: Timer,
}

impl Default for PlayerBullet {
    fn default() -> Self {
        PlayerBullet {
            timer: Timer::new(Duration::from_secs(BULLET_LIFETIME_SECS), TimerMode::Once),
        }
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
