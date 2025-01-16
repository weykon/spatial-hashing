struct StaticUniforms {
    window_size: vec2<f32>,
}
struct DynUniforms {
    delta_time: f32,
    _padding1: f32,
    _padding2: f32,
    _padding3: f32
}
@group(0) @binding(0) var<uniform> static_uniforms: StaticUniforms;
@group(1) @binding(0) var<uniform> dyn_uniforms: DynUniforms;

struct VertexIn {
    @location(0) v: vec2<f32>,
}
struct Instance {
    @location(1) position: vec2<f32>,
    @location(2) velocity: vec2<f32>,
    @location(3) radius: f32,
}
struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) local_pos: vec2<f32>,
    @location(1) radius: f32,
}

@vertex
fn vs_main(input: VertexIn, instance: Instance) -> VertexOut {
    var output: VertexOut;
    output.local_pos = input.v;
    let world_pos = input.v * instance.radius + instance.position;
    let clip_pos = (world_pos / static_uniforms.window_size) * 2.0 - 1.0;
    output.position = vec4<f32>(clip_pos, 0.0, 1.0);
    output.radius = instance.radius;
    return output;
}

@fragment
fn fs_main(input: VertexOut) -> @location(0) vec4<f32> {
    // 计算当前片元到圆心的距离
    let dist = length(input.local_pos);

    // 如果距离大于1，表示在圆形之外
    if (dist > 1.0) {
        discard;
    }

    // 基础颜色：根据速度变化的绿色
    let base_color = vec3<f32>(0.0, 1.0, 0.0);

    // 根据到圆心的距离计算亮度
    let brightness = 1.0 - dist;

    // 添加平滑边缘
    let edge_smoothness = 0.1;
    let alpha = 1.0 - smoothstep(1.0 - edge_smoothness, 1.0, dist);

    // 计算最终颜色
    // 使用 delta_time 添加一些动态效果
    let time_factor = sin(dyn_uniforms.delta_time) * 0.2 + 0.8;
    let radius_factor = input.radius * 0.5;
    let final_color = base_color * (brightness + radius_factor) * time_factor;

    return vec4<f32>(final_color, alpha);
}
