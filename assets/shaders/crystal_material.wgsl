#import bevy_ui::ui_vertex_output::UiVertexOutput

@group(1) @binding(0) var<uniform> tint: vec4<f32>;
@group(1) @binding(1) var<uniform> time: f32;
@group(1) @binding(2) var material_color_texture: texture_2d<f32>;
@group(1) @binding(3) var material_color_sampler: sampler;
@group(1) @binding(4) var material_specular_texture: texture_2d<f32>;
@group(1) @binding(5) var material_specular_sampler: sampler;
@group(1) @binding(6) var material_noise_texture: texture_2d<f32>;
@group(1) @binding(7) var material_noise: sampler;


@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let noise = textureSample(
        material_noise_texture,
        material_noise,
        in.uv
    );
    let value = sin(time * 0.5) - noise.r;
    let baseline = textureSample(material_color_texture, material_color_sampler, in.uv);
    if value > 0 && value < 0.2 {
        let color = mix(baseline, tint, value).rgb;
        return vec4<f32>(color, baseline.a);
    } else {
        return baseline;
    }
}
