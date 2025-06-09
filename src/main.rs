use bevy::{asset::AssetMetaCheck, prelude::*};

mod arena;
mod arena_index;
mod building;
mod enemy;
mod explosion;
mod force;
mod game_assets;
mod game_over;
mod health;
mod hotbar;
mod loading;
mod materials;
mod menu;
mod pause;
mod player;
mod pointer_tracking;
mod reward_select;
mod score;
mod score_ui;
mod tower;
mod waves;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Loading,
    Menu,
    InGame,
    GameOver,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Running,
    Paused,
    Building,
    RewardSelect,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Team {
    Player,
    Enemy,
}

#[derive(Component, Default)]
pub struct PlayerTeam;

#[derive(Component, Default)]
pub struct EnemyTeam;

fn main() {
    App::new()
        // Defaults
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Wasm builds will check for meta files (that don't exist) if this isn't set.
            // This causes errors and even panics in web builds on itch.
            // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
            meta_check: AssetMetaCheck::Never,
            ..Default::default()
        }))
        .add_plugins(MeshPickingPlugin)
        // Resources
        .insert_resource(MeshPickingSettings {
            require_markers: true,
            ray_cast_visibility: RayCastVisibility::Any,
        })
        .insert_resource(ClearColor(Color::hsl(0.0, 0.0, 0.015)))
        // States
        .insert_state(AppState::Loading)
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
        .add_plugins(score::ScorePlugin)
        .add_plugins(score_ui::ScoreUiPlugin)
        .add_plugins(reward_select::RewardSelectPlugin)
        .add_plugins(hotbar::HotbarPlugin)
        .add_plugins(loading::LoadingPlugin)
        .add_plugins(game_over::GameOverPlugin)
        .run();
}
