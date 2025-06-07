use std::time::Duration;

use bevy::prelude::*;

use crate::{
    arena::Arena, arena_index::ArenaIndex, force::ForceEmitter, health::{DamageEvent, Health}, materials::ExplodingRingMaterial, tower::TriggerTowerEvent, AppState, EnemyTeam, GameState, PlayerTeam, Team
};

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_explosion,
                update_material_times,
                apply_explosion_damage,
            )
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameState::Running)),
        );
    }
}

#[derive(Component, Debug)]
#[require(ForceEmitter)]
pub struct Explosion {
    /// The team the explosion is on, the explosion will not damage this team.
    pub team: Team,
    /// The time until this entity is removed.
    pub timer: Timer,
    /// The time until the damage is dealt.
    pub damage_timer: Timer,
    /// The amount of damage to do.
    pub damage: u16,
    /// The area in which the explosion does damage.
    pub damage_area: ExplosionDamageArea,
    /// Force strength changes by this modifier each second.
    pub strength_modifier: f32,
    /// A list of all towers that have been triggered in this event chain.
    pub trigger_history: Vec<Entity>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExplosionDamageArea {
    Radius(f32),
    Hex(u32),
}

pub struct CreateExplosionCommand {
    pub team: Team,
    pub color: LinearRgba,
    pub duration: Duration,
    pub damage_delay: Duration,
    pub damage: u16,
    pub damage_area: ExplosionDamageArea,
    pub position: Vec2,
    pub radius: f32,
    pub strength: f32,
    pub strength_modifier: f32,
    pub trigger_history: Vec<Entity>,
}

impl Command for CreateExplosionCommand {
    fn apply(self, world: &mut World) -> () {
        let mesh_handle = {
            let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
            meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(self.radius)))
        };
        let material_handle = {
            let mut materials = world
                .get_resource_mut::<Assets<ExplodingRingMaterial>>()
                .unwrap();
            materials.add(ExplodingRingMaterial {
                color: self.color,
                time: 0.0,
                duration: self.duration.as_secs_f32(),
            })
        };

        world.spawn((
            Explosion {
                team: self.team,
                timer: Timer::new(self.duration, TimerMode::Once),
                damage_timer: Timer::new(self.damage_delay, TimerMode::Once),
                damage: self.damage,
                damage_area: self.damage_area,
                strength_modifier: self.strength_modifier,
                trigger_history: self.trigger_history,
            },
            ForceEmitter {
                strength: self.strength,
                radius: self.radius,
            },
            Transform::from_xyz(self.position.x, 0.5, self.position.y),
            Mesh3d(mesh_handle),
            MeshMaterial3d(material_handle),
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
            commands.entity(entity).try_despawn();
            continue;
        }

        emitter.strength += explosion.strength_modifier * time.delta_secs();
        if emitter.strength < 0.0 {
            emitter.strength = 0.0;
        }
    }
}

fn apply_explosion_damage(
    time: Res<Time>,
    arena: Res<Arena>,
    arena_index: Res<ArenaIndex>,
    mut evw_damage: EventWriter<DamageEvent>,
    mut evw_trigger_tower: EventWriter<TriggerTowerEvent>,
    mut q_explosions: Query<(Entity, &mut Explosion, &Transform)>,
    q_enemies: Query<(Entity, &Transform), (With<EnemyTeam>, With<Health>)>,
    q_players: Query<(Entity, &Transform), (With<PlayerTeam>, With<Health>)>,
) {
    for (id, mut explosion, transform) in q_explosions.iter_mut() {
        explosion.damage_timer.tick(time.delta());
        if !explosion.damage_timer.just_finished() {
            continue;
        }

        let center_hex = arena.layout.world_pos_to_hex(transform.translation.xz());
        let hex_range = match explosion.damage_area {
            ExplosionDamageArea::Hex(range) => range,
            ExplosionDamageArea::Radius(radius) => radius.ceil() as u32,
        };

        // Tower triggers
        for hex in center_hex.range(hex_range) {
            if let Some(id) = arena_index.tower_index.get(&hex) {
                evw_trigger_tower.write(TriggerTowerEvent {
                    target: *id,
                    trigger_history: explosion.trigger_history.clone(),
                });
            }
        }

        // Get all entities in range from the arena index
        let ids = arena_index.get_many_index(center_hex.range(hex_range));
        let targets = match explosion.team {
            Team::Player => q_enemies
                .iter_many(ids)
                .collect::<Vec<(Entity, &Transform)>>(),
            Team::Enemy => q_players
                .iter_many(ids)
                .collect::<Vec<(Entity, &Transform)>>(),
        };

        // Unit hits
        for (target_id, target_transform) in targets {
            if target_id == id {
                continue;
            }

            if let ExplosionDamageArea::Radius(radius) = explosion.damage_area {
                if transform
                    .translation
                    .xz()
                    .distance(target_transform.translation.xz())
                    > radius
                {
                    continue;
                }
            }

            evw_damage.write(DamageEvent {
                target: target_id,
                damage: explosion.damage,
            });
        }
    }
}

fn update_material_times(mut materials: ResMut<Assets<ExplodingRingMaterial>>, time: Res<Time>) {
    for (_id, material) in materials.iter_mut() {
        material.time += time.delta_secs();
    }
}
