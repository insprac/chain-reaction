use bevy::{color::Color, prelude::{App, DefaultPlugins}, render::camera::ClearColor};

mod player;
mod level;
mod enemy;
mod waves;
mod lighting;
mod hex_grid;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::hsl(0.0, 0.0, 0.015)))
        .add_plugins(player::PlayerPlugin)
        .add_plugins(level::LevelPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(waves::WavePlugin)
        .add_plugins(lighting::LightingPlugin)
        .add_plugins(hex_grid::HexGridPlugin)
        .run();
}
