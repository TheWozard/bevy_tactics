#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> highlight: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> base: vec4<f32>;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let distance_to_center = distance(in.uv, vec2<f32>(0.5)) * 1.4;
    let t_2 = cos(globals.time * 2.0 + distance_to_center * 10.0) * 0.5 + 0.5;
    return vec4<f32>(mix(highlight, base, t_2));
}
