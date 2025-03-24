// custom wgsl file allowing javascript string interpolation :questionable:
// and since i have that why use pipeline constants :shrug:
// there are examples provided below the template used to provide an example
// of what the passed values could be, for exact values, check neuralnet.rs:323
const e = 2.71828182846;
const n_inputs = ${n_inputs};
const n_batches = ${n_batches};
const n_outputs = ${n_outputs};

struct Weights {
${i_weights} 
// ie, weights0: array<array<f32, 12>, 9>
}

struct Biases {
${i_biases} 
// ie, biases0: array<f32, 12>
}

@group(0) @binding(0) var<storage> X: array<array<f32, n_inputs>, n_batches>;
@group(0) @binding(1) var<storage> weights: Weights;
@group(0) @binding(2) var<storage> biases: Biases;
@group(0) @binding(3) var<storage> expected_outputs: array<array<f32, n_outputs>, n_batches>;
@group(0) @binding(4) var<storage, read_write> costs: array<f32, n_batches>;

@compute @workgroup_size(${n_batches}, 1, 1)
fn forward_pass(@builtin(global_invocation_id) id: vec3u) {
    ${forward}
    // ie 
    // let al0 = layer0(X[id.x], weights.weights0, biases.biases0);
    // let al1 = layer1(al0, weights.weights1, biases.biases1);

    costs[id.x] = categorial_cross_entropy(
        expected_outputs[id.x],
        softmax_activation(al${n_al}),
    );
}

${layers} 
// ie
// fn layer0(
//     X_i: array<f32, 9>, 
//     weights_i: array<array<f32, 12>, 9>, 
//     biases_i: array<f32, 12>
// ) -> array<f32, 12> {
//     var al = array<f32, 12>();
//     for (var i = 0; i < 12; i += 1) {
//         for (var j = 0; j < 9; j += 1) {
//             al[i] += weights_i[i][j] * X_i[j];
//         }
//         al[i] += biases_i[i];
//         al[i] = ReLU(al[i]); // if not last layer
//     }
// 
//     return al;
// }


fn ReLU(zl: f32) -> f32 {
    return max(0.0, zl);
}

// O(n^2) :sob: there is easily a better way but im too lazy :3
fn softmax_activation(zl: array<f32, n_outputs>) -> array<f32, n_outputs> {
    var softmax_outputs = array<f32, n_outputs>();

    // calculate e_i^zl
    var sum = 1.0e-20;
    for (var i = 0; i < n_outputs; i += 1) {
        let tmp = pow(e, zl[i]);
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
    expected_outputs_i: array<f32, n_outputs>, 
    softmax_outputs: array<f32, n_outputs>,
) -> f32 {
    var cost = 0.0;
    for (var i = 0; i < n_outputs; i += 1) {
        cost += expected_outputs_i[i] 
                * log(clamp(softmax_outputs[i], 1.0e-7, 1.0));
    }
    return -cost;
}
