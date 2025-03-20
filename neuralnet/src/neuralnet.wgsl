struct vec9f {
    elements: array<f32, 9>
}

@group(0) @binding(0) var<storage> X: array<vec9f>;
@group(0) @binding(1) var<storage> weights: array<vec9f, 9>;
@group(0) @binding(2) var<storage> biases: array<f32>;
@group(0) @binding(3) var<storage> expected_outputs: array<vec9f>;
@group(0) @binding(4) var<storage, read_write> costs: array<vec9f>;

const e = 2.718281;

@compute @workgroup_size(64, 1, 1)
fn cs_main(@builtin(global_invocation_id) id: vec3u) {
    let current_batch = X[id.x];

    var zl = dot(weights, current_batch);
    var highest = -99999999.0;
    for (var i = 0; i < 9; i += 1) {
        zl.elements[i] += biases[i];
        if zl.elements[i] > highest {
            highest = zl.elements[i];
        }
    }

    var outputs = vec9f();
    var sum = 0.0;
    for (var i = 0; i < 9; i += 1) {
        let tmp = pow(e, zl.elements[i] - highest);
        outputs.elements[i] = tmp;
        sum += tmp; 
    }

    for (var i = 0; i < 9; i += 1) {
        outputs.elements[i] /= sum;
    }

    for (var i = 0; i < 9; i += 1) {
        // costs[id.x].elements[i] = pow(outputs.elements[i] - expected_outputs[id.x].elements[i], 2.0);
        costs[id.x].elements[i] = zl.elements[i];
    }
}

fn dot(weights_d: array<vec9f, 9>, x_d: vec9f) -> vec9f {
    var result = vec9f();
    
    for (var i = 0; i < 9; i += 1) {
        for (var j = 0; j <9; j += 1) {
            result.elements[i] += x_d.elements[j] * weights_d[i].elements[j];
        }
    }

    return result;
}