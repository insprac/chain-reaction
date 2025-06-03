use bevy::prelude::*;

use crate::{GameState, PauseState};

pub struct ForcePlugin;

impl Plugin for ForcePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::InGame), cleanup_emitters)
            .add_systems(
                Update,
                (apply_force, reduce_force)
                    .in_set(ForceSet)
                    .run_if(in_state(GameState::InGame))
                    .run_if(in_state(PauseState::Running)),
            );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForceSet;

/// Emits force in a radius around it, `ForceReceivers` receive this force and apply it to `Force`.
/// Emitter strength decays out towards `radius`, see `ForceEmitter::strength_at` for details.
#[derive(Component)]
pub struct ForceEmitter {
    pub radius: f32,
    pub strength: f32,
}

impl Default for ForceEmitter {
    fn default() -> Self {
        Self {
            strength: 10.0,
            radius: 5.0,
        }
    }
}

impl ForceEmitter {
    pub fn strength_at(&self, distance: f32) -> f32 {
        if distance < 0.0 {
            return self.strength;
        }
        if distance > self.radius {
            return 0.0;
        }
        (1.0 - (distance / self.radius)) * self.strength
    }
}

/// Receives force from a `ForceEmitter`.
#[derive(Component)]
#[require(Force)]
pub struct ForceReceiver {
    /// From 0 to 1.0+, the amount of "spring back" force.
    pub restitution_coefficient: f32,
}

impl Default for ForceReceiver {
    fn default() -> Self {
        Self {
            restitution_coefficient: 0.5,
        }
    }
}

/// The accumulated force received.
#[derive(Component, Default)]
pub struct Force {
    pub force: f32,
}

// Marker for a `Force` entity when force is being applied, if force is 0
#[derive(Component, Default)]
pub struct IsForceApplied;

fn cleanup_emitters(mut commands: Commands, q_emitters: Query<Entity, With<ForceEmitter>>) {
    for entity in q_emitters {
        commands.entity(entity).despawn();
    }
}

fn apply_force(
    mut commands: Commands,
    time: Res<Time>,
    q_receiver: Query<(Entity, &Transform, Option<&IsForceApplied>), With<ForceReceiver>>,
    q_emitters: Query<(&ForceEmitter, &Transform)>,
    mut q_force: Query<&mut Force>,
) {
    for (receiver_entity, receiver_transform, is_force_applied) in q_receiver {
        // This is checked and updated so we only add `IsForceApplied` once
        let mut is_force_applied = is_force_applied.is_some();
        let receiver_xz = receiver_transform.translation.xz();
        for (emitter, emitter_transform) in q_emitters {
            let distance = receiver_xz.distance(emitter_transform.translation.xz());
            if distance <= emitter.radius {
                if let Ok(mut force) = q_force.get_mut(receiver_entity) {
                    force.force += emitter.strength_at(distance) * time.delta_secs();
                }
                if !is_force_applied {
                    is_force_applied = true;
                    commands.entity(receiver_entity).insert(IsForceApplied);
                }
            }
        }
    }
}

fn reduce_force(
    mut commands: Commands,
    time: Res<Time>,
    mut q_force: Query<(Entity, &ForceReceiver, &mut Force), With<IsForceApplied>>,
) {
    for (entity, receiver, mut force) in q_force.iter_mut() {
        if force.force < 0.0001 && receiver.restitution_coefficient > 0.0 {
            // Force is imperceivable, set it to zero and remove applied force flag.
            force.force = 0.0;
            commands.entity(entity).remove::<IsForceApplied>();
        } else {
            let restitution = force.force * receiver.restitution_coefficient * time.delta_secs();
            force.force -= restitution;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emitter_strength_at() {
        let emitter = ForceEmitter {
            strength: 10.0,
            radius: 5.0,
        };

        // 0 distance = full strength
        assert_eq!(emitter.strength_at(0.0), 10.0);
        // Max distance = 0 strength
        assert_eq!(emitter.strength_at(5.0), 0.0);
        // Half distance = half strength
        assert_eq!(emitter.strength_at(2.5), 5.0);
        // 1/5 distance = 4/5 strength
        assert_eq!(emitter.strength_at(1.0), 8.0);
    }
}
