use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

pub struct MaterialsPlugin;

impl Plugin for MaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<BulletMaterial>::default())
            .add_plugins(MaterialPlugin::<
                ExtendedMaterial<StandardMaterial, TowerMaterial>,
            >::default());
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BulletMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}

impl Material for BulletMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/bullet.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TowerMaterial {
    #[texture(100, dimension = "2d")]
    #[sampler(101)]
    pub texture: Handle<Image>,
}

impl MaterialExtension for TowerMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/tower.wgsl".into()
    }
}
