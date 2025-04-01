mod neuralnet;
mod utils;
use std::fs::File;
use std::io::BufReader;
use neuralnet::NeuralNet;

fn main() -> std::io::Result<()> {
    let dataset = File::open("./src/datasets/dataset.json")?;
    let reader = BufReader::new(dataset);
    let data: serde_json::Value = serde_json::from_reader(reader)?;
    
    let mut inputs: Vec<Vec<f32>> = Vec::new();
    let mut outputs: Vec<Vec<f32>> = Vec::new();
    for (board, optimal_move) in data.as_object().unwrap() {
        inputs.push(board.chars().map(|c| c.to_digit(10).unwrap() as f32).collect::<Vec<f32>>());
        // inputs.push(vec![board.parse::<f32>().unwrap()]);

        let mut output_vec = vec![0.0f32; 9];
        output_vec[optimal_move.as_u64().unwrap() as usize] = 1.0;
        outputs.push(output_vec);
    }   

    let mut nn = NeuralNet::new(&mut inputs, &mut outputs, &[9]).unwrap();
    nn.train(0.01);

    Ok(())
}