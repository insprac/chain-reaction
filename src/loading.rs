use bevy::prelude::*;

use crate::{AppState, game_assets::GameAssetSet};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, finish_loading.after(GameAssetSet));
    }
}

fn finish_loading(mut next_app_state: ResMut<NextState<AppState>>) {
    next_app_state.set(AppState::Menu);
}
