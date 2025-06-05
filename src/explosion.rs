use std::time::Duration;

use bevy::prelude::*;

use crate::{AppState, GameState, force::ForceEmitter};

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_explosion
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameState::Running)),
        );
    }
}

#[derive(Component, Debug)]
#[require(ForceEmitter)]
pub struct Explosion {
    /// The time until this entity is removed.
    pub timer: Timer,
    /// Force strength is multiplied by this value over time (less than 1.0 reduces over time, more
    /// than 1.0 increases over time).
    pub strength_multiplier: f32,
}

pub struct CreateExplosionCommand {
    pub duration: Duration,
    pub position: Vec2,
    pub radius: f32,
    pub strength: f32,
    pub strength_multiplier: f32,
}

impl Command for CreateExplosionCommand {
    fn apply(self, world: &mut World) -> () {
        world.spawn((
            Explosion {
                timer: Timer::new(self.duration, TimerMode::Once),
                strength_multiplier: self.strength_multiplier,
            },
            ForceEmitter {
                strength: self.strength,
                radius: self.radius,
            },
            Transform::from_xyz(self.position.x, 0.0, self.position.y),
        ));
    }
}

fn update_explosion(
    mut commands: Commands,
    time: Res<Time>,
    mut q_explosions: Query<(Entity, &mut Explosion, &mut ForceEmitter)>,
) {
    for (entity, mut explosion, mut emitter) in q_explosions.iter_mut() {
        explosion.timer.tick(time.delta());

        if explosion.timer.finished() {
            commands.entity(entity).despawn();
            continue;
        }

        if explosion.strength_multiplier != 1.0 {
            let adjust = emitter.strength * explosion.strength_multiplier - emitter.strength;
            emitter.strength += adjust * time.delta_secs();
        }
    }
}
