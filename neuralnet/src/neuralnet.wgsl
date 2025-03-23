// custom wgsl file allowing javascript string interpolation :questionable:
// and since i have that why use pipeline constants :shrug:
const e = 2.71828182846;
const n_batches = ${n_batches};
const n_outputs = ${n_outputs};

struct Weights {
    ${weights} // ie, weights0: array<array<f32, 12>, 9>
}

struct Biases {
    ${biases} // ie, biases0: array<f32, 12>
}

@group(0) @binding(0) var<storage> inputs: array<array<f32, 9>, n_batches>;
@group(0) @binding(1) var<storage> weights: Weights;
@group(0) @binding(2) var<storage> biases: Biases;
@group(0) @binding(3) var<storage> expected_outputs: array<array<f32, n_outputs>, n_batches>;
@group(0) @binding(4) var<storage, read_write> costs: array<f32>;

@compute @workgroup_size(n_batches, 1, 1)
fn forward_pass(@builtin(global_invocation_id) id: vec3u) {
    // let current_batch = inputs[id.x];

    // var zl = dot(weights, current_batch);

    // var softmax_outputs = softmax_activation(zl);

    // costs[id.x] = categorial_cross_entropy(expected_outputs[id.x], softmax_outputs);
    costs[id.x] = 69.0;
}

// fn dot(weights_d: array<vec9f, 9>, x_d: vec9f) -> vec9f {
//     var result = vec9f();
    
//     for (var i = 0; i < 9; i += 1) {
//         for (var j = 0; j <9; j += 1) {
//             result.elements[i] += weights_d[i].elements[j] * x_d.elements[j];
//         }
//     }

//     return result;
// }

fn reLU(zl: f32) -> f32 {
    return max(0.0, zl);
}

// O(n^3) function :sob:
fn softmax_activation(zl: array<f32, n_outputs>) -> array<f32, n_outputs> {
    var softmax_outputs = array<f32, n_outputs>();
    
    // find the highest :/ dek if this saves resources in this case sob
    var highest = 0.0;
    for (var i = 0; i < n_outputs; i += 1) {
        if zl[i] > highest {
            highest = zl[i];
        }
    }

    // calculate e_i^zl
    var sum = 0.0;
    for (var i = 0; i < n_outputs; i += 1) {
        let tmp = pow(e, zl[i] - highest);
        softmax_outputs[i] = tmp;
        sum += tmp; 
    }

    // e_i^zl / sum(e^zl)
    for (var i = 0; i < n_outputs; i += 1) {
        softmax_outputs[i] /= sum;
    }

    return softmax_outputs;
}

fn categorial_cross_entropy(
    expected_outputs_d: array<f32, n_outputs>, 
    softmax_outputs: array<f32, n_outputs>,
) -> f32 {
    var cost = 0.0;
    for (var i = 0; i < n_outputs; i += 1) {
        cost += expected_outputs_d[i] 
            * log(clamp(softmax_outputs[i], 1.0e-7, 1.0));
    }
    return -cost;
}
