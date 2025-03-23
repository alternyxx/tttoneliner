use std::fs::File;
use std::io::prelude::*;
mod neuralnet;

fn main() -> std::io::Result<()> {
    env_logger::init();

    let mut data = String::new();
    {
        let mut dataset = File::open("./src/datasets/dataset.json")?;
        dataset.read_to_string(&mut data)?;
    }
    
    let data: serde_json::Value = serde_json::from_str(&data).expect("uhm");

    let mut inputs: Vec<Vec<f32>> = Vec::new();
    let mut outputs: Vec<Vec<f32>> = Vec::new();
    for (i, (board, optimal_move)) in data.as_object().unwrap().iter().enumerate() {
        inputs.insert(i, board.chars().map(|c| c.to_digit(10).unwrap() as f32).collect::<Vec<f32>>());
        
        let mut output_vec = vec![0.0f32; 9];
        output_vec[optimal_move.as_u64().unwrap() as usize] = 1.0;
        outputs.insert(i, output_vec);
    }   
    drop(data);

    let nn = pollster::block_on(
        neuralnet::NeuralNet::new(&mut inputs, &mut outputs, vec![12, 9, 9], 64u32)
    ).unwrap();
    nn.train();

    Ok(())
}