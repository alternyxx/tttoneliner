struct vec9f {
    elements: array<f32, 9>
}

@group(0) @binding(0) var<storage> X: array<vec9f>;
@group(0) @binding(1) var<storage> weights: array<vec9f>;
@group(0) @binding(2) var<storage> biases: array<vec9f>;
@group(0) @binding(3) var<storage> expected_outputs: array<vec9f>;
@group(0) @binding(4) var<storage, read_write> costs: array<f32>;

@compute @workgroup_size(64, 1, 1)
fn cs_main(@builtin(global_invocation_id) id: vec3u) {
    costs[id.x] = 1.0;
}