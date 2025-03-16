 struct vec9f {    
    x: vec3f,     // i tried setting this to f32s
    y: vec3f,     // but i got a lot like A LOT of 
    z: vec3f,     // alignment errors ;-;
}                 // this means we're gonna have to deal with
// doing things like vec9f.x.x sadly ;-;

struct mat9x9f {
    a: vec9f, b: vec9f, c: vec9f,
    d: vec9f, e: vec9f, f: vec9f,
    g: vec9f, h: vec9f, i: vec9f,
}

@group(0) @binding(0) var<uniform> X: array<vec9f, 64>;
@group(0) @binding(1) var<uniform> weights: mat9x9f;
@group(0) @binding(2) var<uniform> biases: f32;
@group(0) @binding(3) var<storage, read_write> cost: f32;

@compute @workgroup_size(64, 1, 1)
fn cs_main(@builtin(global_invocation_id) id: vec3u) {
    let two = 1.0 + 1.0;
    var a = vec9f(
        vec3f(1.0, 1.0, 1.0),
        vec3f(1.0, 1.0, 1.0),
        vec3f(1.0, 1.0, 1.0),
    );
    a.x += two;
    a.y += two;
    a.z += two;
    cost = X[0].x.x;
}