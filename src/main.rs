use bevy::prelude::{App, DefaultPlugins};

mod player;
mod level;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(level::LevelPlugin)
        .run();
}
