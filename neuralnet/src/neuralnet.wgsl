@group(0) @binding(0) var<uniform> X: vec3f;

@compute @workgroup_size(1, 1)
fn cs_main(@builtin(global_invocation_id) id: vec3u) {
    let a = 1 + 1;
}