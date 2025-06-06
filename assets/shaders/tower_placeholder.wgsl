#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var texture: texture_2d<f32>;
@group(2) @binding(1) var texture_sampler: sampler;

const BASE_COLOR: vec4f = vec4f(0.5, 0.5, 0.8, 0.3);
const HIGHLIGHT_COLOR: vec4f = vec4f(0.5, 0.5, 0.8, 1.0);

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> @location(0) vec4<f32> {
    if in.world_normal.y != 1.0 {
        // This isn't the top face, so it's a side
        if in.uv.y > 0.9 {
            // Draw a line along the top of the side
            return HIGHLIGHT_COLOR;
        } else {
            return BASE_COLOR;
        }
    }

    let sample = textureSample(texture, texture_sampler, in.uv);
    if sample.r > 0.5 {
        return HIGHLIGHT_COLOR;
    } else {
        return BASE_COLOR;
    };
}
