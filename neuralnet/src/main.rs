use std::fs::File;
use std::io::BufReader;
use neuralnyx::{
    NeuralNet,
    Structure,
    Layer,
    TrainingOptions,
};
use neuralnyx::Activation::{Relu, Softmax};
use neuralnyx::CostFunction::CrossEntropy;
use neuralnyx::Optimizer::Adam;

fn main() -> std::io::Result<()> {
    // read the json file 
    let dataset = File::open("./dataset.json")?;
    let reader = BufReader::new(dataset);   // with a buffer
    let data: serde_json::Value = serde_json::from_reader(reader)?;
    
    // reorder the json to a collection of vectors
    let mut boards: Vec<Vec<f32>> = Vec::new();
    let mut moves: Vec<Vec<f32>> = Vec::new();

    for (board, optimal_move) in data.as_object().unwrap() {
        boards.push(board.chars().map(          // a move was encoded as a string so,
            |c| c.to_digit(10).unwrap() as f32  // change that to chars and then convert
        ).collect::<Vec<f32>>());               // them to a Vec<f32>

        let mut output_vec = vec![0.0f32; 9]; // one hot encode the outputs
        output_vec[optimal_move.as_u64().unwrap() as usize] = 1.0;
        moves.push(output_vec);
    }
    
    // create the neural network with our dataset
    let mut nn = NeuralNet::new(&mut boards, &mut moves, Structure {
        batch_size: 64,
        layers: vec![
            Layer {
                neurons: 15,
                activation: Relu,
            }, Layer {
                neurons: 9,
                activation: Softmax,    // probability distribution
            },
        ],
        cost_function: CrossEntropy,
    }).unwrap();

    // train the neuralnet with a custom learning rate function and 100,000 iterations
    nn.train(&TrainingOptions {
        optimizer: Adam(0.001),
        epochs: 150_000,
        verbose: true,
    });

    println!("{}", nn); // just print the weights and biases of the neural network to stdout

    Ok(())
}