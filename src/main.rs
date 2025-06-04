use bevy::prelude::{App, ClearColor, Color, DefaultPlugins, States, AppExtStates};

mod arena;
mod enemy;
mod game_assets;
mod health;
mod player;
mod waves;
mod menu;
mod pause;
mod force;
mod materials;
mod arena_index;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Menu,
    InGame,
    GameOver,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PauseState {
    Running,
    Paused,
}

fn main() {
    App::new()
        // Defaults
        .add_plugins(DefaultPlugins)
        // States
        .insert_state(GameState::Menu)
        .insert_state(PauseState::Running)
        // Resources
        .insert_resource(ClearColor(Color::hsl(0.0, 0.0, 0.015)))
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
        .run();
}
