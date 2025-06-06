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
        // This isn't the top face, so it's a side
        if in.uv.y > 0.9 {
            // Draw a line along the top of the side
            out.color = HIGHLIGHT_COLOR;
        }
        return out;
    }

    let sample = textureSample(detail_texture, detail_texture_sampler, in.uv);
    if sample.r > 0.5 {
        out.color = HIGHLIGHT_COLOR;
    }

    return out;
}
