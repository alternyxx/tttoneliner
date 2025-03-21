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

    // let nn = pollster::block_on(
    //     neuralnet::NeuralNet::new(&mut inputs, &mut outputs, vec![9i32], 64u32)
    // );
    // nn.train();

    println!("{}", template_wgsl("var ?{uhmmm}; \n like that's so ?{weird}"));

    Ok(())
}

fn template_wgsl(wgsl: &str) -> String {
    let mut templating = false;
    let mut template_variable: String = String::new();
    let mut templated_wgsl: String = String::new();

    for (_i, char) in wgsl.chars().enumerate() {
        
        // in the process of templating
        if templating {
            if char == '}' {
                templated_wgsl += "bruh";
                template_variable = String::new();
                templating = false;
            } else if char == '{' {
                continue
            } else {
                template_variable += &char.to_string();    
            }

            continue
        } else if char == '?' {
            templating = true;
        } else {
            templated_wgsl += &char.to_string();
        }

    }

    templated_wgsl
}