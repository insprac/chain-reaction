use std::time::Duration;

use bevy::prelude::*;

use crate::{AppState, GameState, force::ForceEmitter, materials::ExplodingRingMaterial};

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_explosion, update_material_times)
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
    /// Force strength changes by this modifier each second.
    pub strength_modifier: f32,
}

pub struct CreateExplosionCommand {
    pub duration: Duration,
    pub position: Vec2,
    pub radius: f32,
    pub strength: f32,
    pub strength_modifier: f32,
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
                color: LinearRgba::new(1.0, 0.0, 0.0, 1.0),
                time: 0.0,
                duration: self.duration.as_secs_f32(),
            })
        };

        world.spawn((
            Explosion {
                timer: Timer::new(self.duration, TimerMode::Once),
                strength_modifier: self.strength_modifier,
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

fn update_material_times(mut materials: ResMut<Assets<ExplodingRingMaterial>>, time: Res<Time>) {
    for (_id, material) in materials.iter_mut() {
        material.time += time.delta_secs();
    }
}
