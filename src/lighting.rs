use bevy::prelude::*;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, update_ambient_light);
    }
}

fn update_ambient_light(mut ambient_light: ResMut<AmbientLight>) {
    ambient_light.brightness = 1000.0;
}
