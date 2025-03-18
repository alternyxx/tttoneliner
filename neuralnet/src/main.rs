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

    let mut inputs: Vec<Vec<f32>> = vec!();
    let mut expected_outputs: Vec<f32> = Vec::new();
    for (i, (board, optimal_move)) in data.as_object().unwrap().iter().enumerate() {
        inputs.insert(i, board.chars().map(|c| c.to_digit(10).unwrap() as f32).collect::<Vec<f32>>());
        expected_outputs.insert(i, optimal_move.as_f64().unwrap() as f32);
    }   
    drop(data);

    pollster::block_on(
        neuralnet::NeuralNet::new(&mut inputs, &mut vec!([9]), expected_outputs, 64u32)
    );

    Ok(())
}