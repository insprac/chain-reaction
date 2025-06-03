use bevy::prelude::*;

pub struct GameAssetPlugin;

impl Plugin for GameAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_assets);
    }
}

#[derive(Resource)]
pub struct GameAssets {
    pub enemy_mesh: Handle<Mesh>,
    pub enemy_material: Handle<StandardMaterial>,
    pub player_bullet_mesh: Handle<Mesh>,
    pub player_bullet_material: Handle<StandardMaterial>,
}

fn load_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Enemy
    let enemy_mesh = meshes.add(Cuboid::new(0.5, 0.3, 0.5));
    let enemy_material = materials.add(StandardMaterial {
        base_color: Color::hsl(350.0, 1.0, 0.5),
        perceptual_roughness: 1.0,
        unlit: true,
        ..default()
    });

    // Player bullet
    let player_bullet_mesh = meshes.add(Plane3d::new(Vec3::Y, Vec2::new(0.05, 0.6)));
    let player_bullet_material = materials.add(StandardMaterial {
        base_color: Color::hsl(46.0, 1.0, 0.8),
        perceptual_roughness: 1.0,
        unlit: true,
        ..default()
    });

    commands.insert_resource(GameAssets {
        enemy_mesh,
        enemy_material,
        player_bullet_mesh,
        player_bullet_material,
    });
}
