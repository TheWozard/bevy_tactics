#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> seed: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> base: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> highlight: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> accent: vec4<f32>;

// Based on https://www.shadertoy.com/view/33cfzf

fn random(st: vec2<f32>) -> f32 {
    return fract(sin(dot(st, vec2<f32>(12.9898, 78.233))) * 43758.5453123);
}

fn noise(st: vec2<f32>) -> f32 {
    let i = floor(st);
    let f = fract(st);

    let a = random(i);
    let b = random(i + vec2<f32>(1.0, 0.0));
    let c = random(i + vec2<f32>(0.0, 1.0));
    let d = random(i + vec2<f32>(1.0, 1.0));

    let u = f * f * (3.0 - 2.0 * f);

    return mix(a, b, u.x) +
           (c - a) * u.y * (1.0 - u.x) +
           (d - b) * u.x * u.y;
}

fn fbm(st_in: vec2<f32>) -> f32 {
    var st = st_in;
    var value = 0.0;
    var amplitude = 0.5;
    for (var i: i32 = 0; i < 6; i = i + 1) {
        value = value + amplitude * noise(st);
        st = st * 2.0;
        amplitude = amplitude * 0.5;
    }
    return value;
}

struct PatternResult {
    f: f32,
    q: vec2<f32>,
    r: vec2<f32>,
}

fn pattern(p: vec2<f32>) -> PatternResult {
    let t = globals.time * 0.15;

    let qx = fbm(p + vec2<f32>(seed, 0.0) + vec2<f32>(sin(t), cos(t)) * 0.5);
    let qy = fbm(p + vec2<f32>(5.2, 1.3) + vec2<f32>(cos(t * 1.3), sin(t * 0.7)) * 0.5);
    let q = vec2<f32>(qx, qy);

    let rx = fbm(p + 4.0 * q + vec2<f32>(1.7, 9.2) + vec2<f32>(sin(t * 0.8), cos(t * 1.1)) * 0.3);
    let ry = fbm(p + 4.0 * q + vec2<f32>(8.3, 2.8) + vec2<f32>(cos(t * 0.9), sin(t * 1.2)) * 0.3);
    let r = vec2<f32>(rx, ry);

    let f = fbm(p + 4.0 * r);

    return PatternResult(f, q, r);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = (in.uv - 0.5) * 3.0;
    let res = pattern(uv);

    var color = base.rgb;

    color = mix(color, highlight.rgb,
                clamp(res.f * res.f * 4.0, 0.0, 1.0));

    color = mix(color, accent.rgb,
                clamp(length(res.q), 0.0, 1.0));

    color = mix(color, highlight.rgb,
                clamp(abs(res.r.x), 0.0, 1.0));

    let intensity = res.f * res.f * res.f + 0.6 * res.f * res.f + 0.5 * res.f;

    var final_color = color * intensity;
    return vec4<f32>(final_color, 1.0);
}
