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
            >::default())
            .add_plugins(MaterialPlugin::<TowerPlaceholderMaterial>::default())
            .add_plugins(MaterialPlugin::<ExplodingRingMaterial>::default());
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

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TowerPlaceholderMaterial {
    #[texture(0, dimension = "2d")]
    #[sampler(1)]
    pub texture: Handle<Image>,
}

impl Material for TowerPlaceholderMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/tower_placeholder.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct ExplodingRingMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    /// Gets ticked every frame to update the texture animation.
    #[uniform(0)]
    pub time: f32,
    /// The total duration of the duration, nothing is rendered past this point in time.
    #[uniform(0)]
    pub duration: f32,
}

impl Material for ExplodingRingMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/exploding_ring.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}
