use bevy::{color::Color, prelude::{App, DefaultPlugins}, render::camera::ClearColor};

mod player;
mod enemy;
mod waves;
mod lighting;
mod arena;
mod game_assets;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::hsl(0.0, 0.0, 0.015)))
        .add_plugins(player::PlayerPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(waves::WavePlugin)
        .add_plugins(lighting::LightingPlugin)
        .add_plugins(arena::ArenaPlugin)
        .add_plugins(game_assets::GameAssetPlugin)
        .run();
}
