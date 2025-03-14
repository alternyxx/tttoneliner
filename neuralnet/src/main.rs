use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use serde_json;
use pollster;
mod neuralnet;

fn main() -> std::io::Result<()> {
    let mut data = String::new();
    {
        let mut dataset = File::open("./src/datasets/dataset.json")?;
        dataset.read_to_string(&mut data)?;
    }
    
    let data: serde_json::Value = serde_json::from_str(&data).expect("uhm");

    // Make the object into a hashmap for no reason
    let mut optimal_moves: HashMap<Vec<i8>, i8> = HashMap::new();
    for (board, optimal_move) in data.as_object().unwrap() {
        optimal_moves.insert(
            board.chars().map(|c| c.to_digit(10).unwrap() as i8).collect(), 
            optimal_move.as_i64().unwrap() as i8
        );
    }   
    drop(data);

    pollster::block_on(neuralnet::neuralnet(&mut optimal_moves));

    Ok(())
}