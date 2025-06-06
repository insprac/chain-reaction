#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::apply_pbr_lighting,
}

@group(2) @binding(100) var detail_texture: texture_2d<f32>;
@group(2) @binding(101) var detail_texture_sampler: sampler;

const BASE_COLOR: vec4f = vec4f(0.0, 0.0, 0.0, 1.0);
const HIGHLIGHT_COLOR: vec4f = vec4f(0.2, 0.8, 0.2, 1.0);

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);

    if in.world_normal.y != 1.0 {
        if in.uv.y > 0.9 {
            out.color = HIGHLIGHT_COLOR;
        }
        return out;
    }

    let scale = 1.0;
    // Translate to center, scale, translate back
    let scaled_uv = (in.uv - vec2(0.5)) * scale + vec2(0.5);
    if (scaled_uv.x < 0.0 || scaled_uv.x > 1.0 || scaled_uv.y < 0.0 || scaled_uv.y > 1.0) {
        // Outside the texture
        return out;
    }

    let sample = textureSample(detail_texture, detail_texture_sampler, scaled_uv);
    if sample.r > 0.5 {
        out.color = HIGHLIGHT_COLOR;
    }

    return out;
}
