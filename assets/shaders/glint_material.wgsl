#import bevy_ui::ui_vertex_output::UiVertexOutput

@group(1) @binding(0) var<uniform> tint: vec4<f32>;
@group(1) @binding(1) var<uniform> time: f32;
@group(1) @binding(2) var material_color_texture: texture_2d<f32>;
@group(1) @binding(3) var material_color_sampler: sampler;
@group(1) @binding(4) var material_specular_texture: texture_2d<f32>;
@group(1) @binding(5) var material_specular_sampler: sampler;


@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    if textureSample(
        material_specular_texture,
        material_specular_sampler,
        in.uv
    ).r > 0 && sin(time + (in.uv.x)) > 0.999 {
        return mix(textureSample(material_color_texture, material_color_sampler, in.uv), vec4<f32>(1.0, 1.0, 1.0, 1.0), 0.5);
    }
    return textureSample(material_color_texture, material_color_sampler, in.uv);
}
