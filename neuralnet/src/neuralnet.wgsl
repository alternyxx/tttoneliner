// custom wgsl file allowing javascript string interpolation
// and since i have that why use pipeline constants :shrug:
const e = 2.71828182846;
const n_batches = ${n_batches};
const n_outputs = ${n_outputs};

struct vec9f {
    elements: array<f32, 9>
}

struct Weights {
    ${weights} // ie, weights0: array<array<f32, 12>, 9>
}

struct Biases {
    ${biases} // ie, biases0: array<f32, 12>
}

@group(0) @binding(0) var<storage> inputs: array<vec9f, n_batches>;
@group(0) @binding(1) var<storage> weights: array<vec9f, 9>;
@group(0) @binding(2) var<storage> biases: array<f32>;
@group(0) @binding(3) var<storage> expected_outputs: array<vec9f, n_batches>;
@group(0) @binding(4) var<storage, read_write> costs: array<f32, n_batches>;

@compute @workgroup_size(n_batches, 1, 1)
fn cs_main(@builtin(global_invocation_id) id: vec3u) {
    let current_batch = inputs[id.x];

    var zl = dot(weights, current_batch);
    var highest = -9.0e-10;
    for (var i = 0; i < 9; i += 1) {
        zl.elements[i] += biases[i];
    }

    var softmax_outputs = softmax_activation(zl);

    costs[id.x] = categorial_cross_entropy(expected_outputs[id.x], softmax_outputs);
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

fn reLU(output: f32) -> f32 {
    return max(0.0, output);
}

// O(n^3) function :sob:
fn softmax_activation(zl: vec9f) -> vec9f {
    var outputs = vec9f();

    var highest = 0.0;
    for (var i = 0; i < 9; i += 1) {
        if zl.elements[i] > highest {
            highest = zl.elements[i];
        }
    }

    var sum = 0.0;
    // calculate e_i^zl
    for (var i = 0; i < 9; i += 1) {
        let tmp = pow(e, zl.elements[i] - highest);
        outputs.elements[i] = tmp;
        sum += tmp; 
    }

    // e_i^zl / sum(e^zl)
    for (var i = 0; i < 9; i += 1) {
        outputs.elements[i] /= sum;
    }

    return outputs;
}

fn categorial_cross_entropy(expected_outputs_d: vec9f, softmax_outputs: vec9f) -> f32 {
    var cost = 0.0;
    for (var i = 0; i < 9; i += 1) {
        cost += expected_outputs_d.elements[i] * log(clamp(softmax_outputs.elements[i], 1.0e-7, 1.0));
    }
    return -cost;
}
