@group(0) @binding(0) var<uniform> X: vec3f;
@group(0) @binding(1) var<uniform> weights: f32;
@group(0) @binding(2) var<uniform> biases: f32;
@group(0) @binding(3) var<storage, read_write> cost: f32;

@compute @workgroup_size(1, 1)
fn cs_main(@builtin(global_invocation_id) id: vec3u) {
    let two = 1 + 1;
    cost = weights + biases;
}