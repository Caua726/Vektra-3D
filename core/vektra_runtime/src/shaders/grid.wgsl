// Uniform buffer para matrizes e dados da câmera
struct Uniforms {
    view_proj: mat4x4<f32>,
    // Usamos vec4 para garantir alinhamento de 16 bytes. O componente 'w' é ignorado.
    camera_pos: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

// Saída do Vertex Shader, entrada para o Fragment Shader
struct VSOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
};

@vertex
fn vs_main(@location(0) in_pos: vec3<f32>) -> VSOut {
    var out: VSOut;
    let world_pos = vec4<f32>(in_pos, 1.0);
    out.clip_pos = uniforms.view_proj * world_pos;
    out.world_pos = in_pos;
    return out;
}

// Função para desenhar linhas de grade
fn draw_grid(p: vec2<f32>, spacing: f32, thickness: f32) -> f32 {
    let grid = abs(fract(p / spacing - 0.5) - 0.5) / fwidth(p / spacing);
    return smoothstep(0.0, thickness, 1.0 - min(min(grid.x, grid.y), 1.0));
}

@fragment
fn fs_main(in: VSOut) -> @location(0) vec4<f32> {
    let plane_pos = in.world_pos.xz;

    // Parâmetros fixos da grade
    let minor_spacing = 1.0;
    let major_spacing = 10.0;
    let minor_color = vec3(0.22);
    let major_color = vec3(0.35);
    let axis_x_color = vec3(0.8, 0.2, 0.2);
    let axis_z_color = vec3(0.2, 0.2, 0.8);

    // Linhas menores
    let minor = draw_grid(plane_pos, minor_spacing, 0.5);
    // Linhas maiores
    let major = draw_grid(plane_pos, major_spacing, 1.5);

    // Cor base da grade
    var color = mix(minor_color, major_color, major);
    var alpha = max(minor, major);

    // Eixos principais
    let axis_width = 2.0;
    let axis_x = abs(plane_pos.x) / (fwidth(plane_pos.x) * axis_width);
    let axis_z = abs(plane_pos.y) / (fwidth(plane_pos.y) * axis_width);
    color = mix(color, axis_x_color, (1.0 - min(axis_z, 1.0)));
    color = mix(color, axis_z_color, (1.0 - min(axis_x, 1.0)));
    alpha = max(alpha, 1.0 - min(min(axis_x, axis_z), 1.0));
    
    // Fade-out na distância
    let dist_from_cam = length(plane_pos - uniforms.camera_pos.xz);
    let fade_dist = 50.0;
    alpha *= 1.0 - smoothstep(fade_dist * 0.8, fade_dist, dist_from_cam);

    return vec4<f32>(color, alpha);
}
