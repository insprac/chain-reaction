#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let center = vec2<f32>(0.5, 0.5);

    if distance(in.uv, center) < 0.5 {
        let alpha = -log(abs(in.uv.y)) * 0.2;
        return vec4<f32>(material_color.rgb, alpha + distort);
    } else {
        return vec4<f32>(material_color.rgb, 0.0);
    }
}
