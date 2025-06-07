#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var<uniform> time: f32;
@group(2) @binding(2) var<uniform> duration: f32;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Skip if the time is up
    if time > duration {
        return vec4f(material_color.rgb, 0.0);
    }

    let scaled_time = time / duration;

    // Normalize fragment coordinates to a range of -1.0 to 1.0, with 0.0 at the center
    let uv = in.uv * 2.0 - 1.0;
    let dist = length(uv);

    // Ring radius expands over time
    let radius = scaled_time * 1.2;

    // Ring thickness, gets thinner as it expands
    let ring_width = 0.3 * (1.0 - scaled_time);
    
    // Calculate the intensity of the ring at the current pixel
    let inner_edge = radius - ring_width / 2.0;
    let outer_edge = radius + ring_width / 2.0;
    let ring_intensity = step(inner_edge, dist) - step(outer_edge, dist);
    
    // Fade the ring out as it expands
    let fade = pow(1.0 - scaled_time, 4.0);
    
    return vec4<f32>(material_color.rgb, ring_intensity * fade);
}
