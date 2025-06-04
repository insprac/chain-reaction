use std::collections::HashMap;

use bevy::prelude::*;
use hexx::Hex;

pub struct IndexPlugin;

impl Plugin for IndexPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyIndex>();
    }
}

#[derive(Resource, Default)]
pub struct EnemyIndex {
    index: HashMap<Hex, Vec<Entity>>,
}
