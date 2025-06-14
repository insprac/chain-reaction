use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;

use crate::player::bullet::SpawnPlayerBulletCommand;

#[derive(Component)]
pub struct PlayerGun {
    /// The angle of the gun in radians (in the range [-π, +π]).
    pub angle: f32,
    pub cooldown: Timer,
}

impl Default for PlayerGun {
    fn default() -> Self {
        PlayerGun {
            angle: 0.0,
            cooldown: Timer::new(Duration::from_millis(200), TimerMode::Once),
        }
    }
}

pub fn update_gun_direction(mut q_gun: Query<&mut PlayerGun>, q_window: Query<&Window>) -> Result {
    let window = q_window.single()?;
    if let Some(cursor_pos) = window.cursor_position() {
        let mut gun = q_gun.single_mut()?;
        let center = window.size() / 2.0;
        let direction = (cursor_pos - center).normalize_or_zero();
        gun.angle = direction.y.atan2(direction.x);
    }
    Ok(())
}

pub fn update_gun_cooldown(time: Res<Time>, mut q_gun: Query<&mut PlayerGun>) -> Result {
    let mut gun = q_gun.single_mut()?;
    gun.cooldown.tick(time.delta());
    Ok(())
}

pub fn fire_gun(mut commands: Commands, mut q_gun: Query<(&mut PlayerGun, &Transform)>) -> Result {
    let (mut gun, gun_transform) = q_gun.single_mut()?;

    if !gun.cooldown.finished() {
        return Ok(());
    }

    gun.cooldown.reset();

    let mut transform = Transform::from_translation(gun_transform.translation)
        .with_rotation(Quat::from_axis_angle(Vec3::Y, -PI / 2.0 + -gun.angle));
    transform.translation = transform.translation + transform.forward().as_vec3() * 1.5;

    commands.queue(SpawnPlayerBulletCommand {
        transform,
        trigger_history: Vec::new(),
    });

    Ok(())
}
