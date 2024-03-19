struct Quad {
    position: vec2f,
    size: vec2f,
    color: vec3f,
}

struct QuadStorage {
    cnt: u32,
    quads: array<Quad>
}

// struct QuadVarying {
//     @builtin(position) position: vec4f,
//     @location(0) @interpolate(flat) background_color: vec4<f32>,
//     @location(1) @interpolate(flat) border_color: vec4<f32>,
//     @location(2) @interpolate(flat) quad_id: u32,
// }

@group(0) @binding(0) var<storage> quads: array<Quad>;

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32, @builtin(instance_index) instance_index: u32) -> @builtin(position) vec4<f32> {
    // 0 -> 0, 0
    // 1 -> 1, 0
    // 2 -> 0, 1
    // 3 -> 1, 1
    let unit_vertex = vec2<f32>(f32(vertex_index & 1u), 0.5 * f32(vertex_index & 2u));
    let quad = quads[instance_index];

    // var out = QuadVarying();
    // out.position = to_device_position(unit_vertex, quad.bounds);
    // out.background_color = hsla_to_rgba(quad.background);
    // out.border_color = hsla_to_rgba(quad.border_color);
    // out.quad_id = instance_id;
    
    // [0, 1] -> [-1, 1] and flip y
    // 2x - 1, 1 - 2y

    let x = quad.position.x + unit_vertex.x * quad.size.x;
    let y = quad.position.y + unit_vertex.y * quad.size.y;

    return vec4<f32>(x * 2.0 - 1.0, 1.0 - 2.0 * y, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
