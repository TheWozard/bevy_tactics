#import bevy_ui::ui_vertex_output::UiVertexOutput

@group(1) @binding(0) var<uniform> tint: vec4<f32>;
@group(1) @binding(1) var<uniform> time: f32;
@group(1) @binding(2) var<uniform> direction: vec2<f32>;
@group(1) @binding(3) var material_color_texture: texture_2d<f32>;
@group(1) @binding(4) var material_color_sampler: sampler;
@group(1) @binding(5) var material_specular_texture: texture_2d<f32>;
@group(1) @binding(6) var material_specular_sampler: sampler;
@group(1) @binding(7) var material_noise_texture: texture_2d<f32>;
@group(1) @binding(8) var material_noise: sampler;


@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let spec = textureSample(
        material_specular_texture,
        material_specular_sampler,
        in.uv
    );
    if spec.r > 0 {
        let noise = textureSample(
            material_noise_texture,
            material_noise,
            vec2<f32>(in.uv.x, (in.uv.y + (time / 4.0)) % 1.0)
        );
        let offset = direction * noise.r * 0.1;
        return textureSample(material_color_texture, material_color_sampler, in.uv + offset) * tint;
    }
    return textureSample(material_color_texture, material_color_sampler, in.uv);
}
