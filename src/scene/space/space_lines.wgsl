struct StaticUniforms {
    window_size: vec2<f32>,
}
@group(0) @binding(0) var<uniform> static_uniforms: StaticUniforms;

struct _Vertex {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
}
struct VOut {
    @builtin(position) position: vec4<f32>,
    @location(0) clip_pos: vec2<f32>,
    @location(1) color: vec4<f32>,
}
@vertex
fn vs_main(v: _Vertex) -> VOut {
    var output: VOut;
    output.color = v.color;
    var clip_pos = (v.position / static_uniforms.window_size ) * 2.0 - 1.0;
    output.position = vec4<f32>(clip_pos, 0.0, 1.0);
    return output;
}

@fragment
fn fs_main(v: VOut) -> @location(0) vec4<f32> {
    return v.color;
}
