struct CameraUniform {
    view_proj: mat4x4<f32>,
}
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct InstanceRaw {
    model: mat4x4<f32>,
}

@group(1) @binding(0)
var<storage, read> instances: array<InstanceRaw>;

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    @builtin(instance_index) instance_index: u32,
) -> VertexOutput {
    let model_matrix = instances[instance_index].model;
    var out: VertexOutput;
    out.clip_position = camera.view_proj * model_matrix * vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.4, 0.5, 1.0);
} 