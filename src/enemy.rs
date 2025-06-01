use bevy::prelude::*;

use crate::player::Player;

const MOVE_SPEED: f32 = 3.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyAssets>()
            .add_systems(Startup, load_enemy_assets.in_set(EnemySet))
            .add_systems(Update, follow_player.in_set(EnemySet));
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnemySet;

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct Enemy;

#[derive(Resource, Default)]
pub struct EnemyAssets {
    mesh_handle: Option<Handle<Mesh>>,
    material_handle: Option<Handle<StandardMaterial>>,
}

pub struct SpawnEnemiesCommand {
    pub positions: Vec<Vec3>,
}

impl Command for SpawnEnemiesCommand {
    fn apply(self, world: &mut World) -> () {
        let Some(enemy_assets) = world.get_resource::<EnemyAssets>() else {
            panic!("Failed to get EnemyAssets resource in SpawnEnemiesCommand");
        };
        let Some(mesh_handle) = enemy_assets.mesh_handle.clone() else {
            panic!("mesh_handle wasn't present in EnemyAssets");
        };
        let Some(material_handle) = enemy_assets.material_handle.clone() else {
            panic!("material_handle wasn't present in EnemyAssets");
        };

        for pos in self.positions {
            world.spawn((
                Enemy,
                Transform::from_xyz(pos.x, 0.0, pos.z),
                Mesh3d(mesh_handle.clone()),
                MeshMaterial3d(material_handle.clone()),
            ));
        }
    }
}

fn load_enemy_assets(
    mut enemy_assets: ResMut<EnemyAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    enemy_assets.mesh_handle = Some(meshes.add(Cuboid::new(0.5, 0.3, 0.5)));
    enemy_assets.material_handle = Some(materials.add(StandardMaterial {
        base_color: Color::hsl(350.0, 1.0, 0.5),
        perceptual_roughness: 1.0,
        unlit: true,
        ..default()
    }));
}

fn follow_player(
    time: Res<Time>,
    q_player_transform: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut q_enemy_transform: Query<&mut Transform, With<Enemy>>,
) -> Result {
    let player_translation = q_player_transform.single()?.translation;

    for mut enemy_transform in q_enemy_transform.iter_mut() {
        let y = enemy_transform.translation.y;
        let direction =
            (player_translation.with_y(y) - enemy_transform.translation).normalize_or_zero();
        enemy_transform.translation += direction * MOVE_SPEED * time.delta_secs();
        enemy_transform.look_at(player_translation.with_y(y), Vec3::Y);
    }

    Ok(())
}
