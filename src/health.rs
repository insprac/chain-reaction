use bevy::prelude::*;

use crate::{GameState, PauseState};

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>()
            .add_event::<HealEvent>()
            .add_event::<DiedEvent>()
            .add_systems(
                Update,
                (
                    apply_heal_event.run_if(on_event::<HealEvent>),
                    apply_damage_event
                        .run_if(on_event::<DamageEvent>)
                        .after(apply_heal_event),
                )
                    .in_set(HealthSet)
                    .run_if(in_state(GameState::InGame))
                    .run_if(in_state(PauseState::Running)),
            );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct HealthSet;

/// A unit's health, current health should never be modified directly, use `DamageEvent` and
/// `HealEvent` instead.
///
/// When current health reaches 0 a `DiedEvent` is emitted and triggered on the entity,
/// then any future heals or damage are ignored.
#[derive(Component)]
pub struct Health {
    pub current: u16,
    pub max: u16,
}

impl Default for Health {
    fn default() -> Self {
        Self::new(1)
    }
}

impl Health {
    pub fn new(max: u16) -> Self {
        Self { max, current: max }
    }
}

/// Emit to damage a unit's health.
#[derive(Event)]
pub struct DamageEvent {
    pub target: Entity,
    pub damage: u16,
}

/// Emit to heal a unit's health.
#[derive(Event)]
pub struct HealEvent {
    pub target: Entity,
    pub heal: u16,
}

/// Emitted when health reaches 0.
#[derive(Event, Clone)]
pub struct DiedEvent {
    pub entity: Entity,
}

fn apply_damage_event(
    mut commands: Commands,
    mut evr_damage: EventReader<DamageEvent>,
    mut evw_died: EventWriter<DiedEvent>,
    mut q_health: Query<&mut Health>,
) {
    for event in evr_damage.read() {
        let Ok(mut health) = q_health.get_mut(event.target) else {
            continue;
        };

        if health.current == 0 {
            continue;
        }

        if health.current > event.damage {
            health.current -= event.damage;
        } else {
            health.current = 0;
            let died_event = DiedEvent {
                entity: event.target,
            };
            evw_died.write(died_event.clone());
            commands.entity(event.target).trigger(died_event);
        }
    }
}

fn apply_heal_event(mut evr_heal: EventReader<HealEvent>, mut q_health: Query<&mut Health>) {
    for event in evr_heal.read() {
        let Ok(mut health) = q_health.get_mut(event.target) else {
            continue;
        };

        if health.current == 0 {
            continue;
        }

        health.current = (health.current + event.heal).min(health.max);
    }
}
