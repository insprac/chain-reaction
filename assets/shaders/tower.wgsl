#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var texture: texture_2d<f32>;
@group(2) @binding(1) var texture_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    if mesh.world_normal.y == 1.0 {
        let scale = 1.5;

        // Translate to center, scale, translate back
        let scaled_uv = (mesh.uv - vec2(0.5)) * scale + vec2(0.5);

        if (scaled_uv.x < 0.0 || scaled_uv.x > 1.0 || scaled_uv.y < 0.0 || scaled_uv.y > 1.0) {
            // Outside the texture
            return vec4(0.0, 0.0, 0.0, 0.0);
        }

        let sample = textureSample(texture, texture_sampler, scaled_uv);
        return sample;
    } else {
        return vec4(0.0, 0.0, 0.0, 0.0);
    }
}
