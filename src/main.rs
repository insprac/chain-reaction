use bevy::prelude::{App, DefaultPlugins};

mod player;
mod level;
mod enemy;
mod waves;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(level::LevelPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(waves::WavePlugin)
        .run();
}
