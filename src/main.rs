use bevy::{
    picking::mesh_picking::{ray_cast::RayCastVisibility, MeshPickingPlugin, MeshPickingSettings},
    prelude::{App, AppExtStates, ClearColor, Color, DefaultPlugins, States},
};

mod arena;
mod arena_index;
mod enemy;
mod force;
mod game_assets;
mod health;
mod materials;
mod menu;
mod pause;
mod player;
mod tower;
mod waves;
mod pointer_tracking;
mod explosion;
mod building;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Menu,
    InGame,
    GameOver,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Running,
    Paused,
    Building,
}

fn main() {
    App::new()
        // Defaults
        .add_plugins(DefaultPlugins)
        .add_plugins(MeshPickingPlugin)
        // Resources
        .insert_resource(MeshPickingSettings {
            require_markers: true,
            ray_cast_visibility: RayCastVisibility::Any,
        })
        .insert_resource(ClearColor(Color::hsl(0.0, 0.0, 0.015)))
        // States
        .insert_state(AppState::Menu)
        .insert_state(GameState::Running)
        // Game Plugins
        .add_plugins(menu::MenuPlugin)
        .add_plugins(pause::PausePlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(waves::WavePlugin)
        .add_plugins(arena::ArenaPlugin)
        .add_plugins(game_assets::GameAssetPlugin)
        .add_plugins(health::HealthPlugin)
        .add_plugins(force::ForcePlugin)
        .add_plugins(materials::MaterialsPlugin)
        .add_plugins(arena_index::ArenaIndexPlugin)
        .add_plugins(pointer_tracking::PointerTrackingPlugin)
        .add_plugins(tower::TowerPlugin)
        .add_plugins(building::BuildingPlugin)
        .add_plugins(explosion::ExplosionPlugin)
        .run();
}
