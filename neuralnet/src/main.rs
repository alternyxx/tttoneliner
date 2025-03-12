use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use serde_json;

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


    let mut weights: [i8; 9] = [2, 5, 7, 3, 4, 1, 6, 8, 1];
    while incrementation(&mut weights) {
        let mut mapped: i16 = 0;
        
        'outer: for (board, optimal_move) in &optimal_moves { 
            let mut s: i64 = 0;
            for (i, weight) in weights.iter().enumerate() {
                s += (board[i] * weight) as i64;
            }

            if (s % 9) as i8 != *optimal_move {
                mapped += 1;
                break 'outer;
            } 
        }

        // if mapped == got_it {
        //     println!("{:?}", weights);
        //     panic!();
        // } else if mapped >= 200 {
        //     println!("{:?}", weights);
        //     panic!();
        // } else {
        // }
        println!("mapped {} with weights, {:?}", mapped, weights);
    }
    println!("{:?}", weights);

    Ok(())  
}   

fn incrementation(weights: &mut [i8; 9]) -> bool {
    let reversed: Vec<i8> = weights.iter().copied().rev().collect();

    for (i, digit) in reversed.iter().enumerate() {
        let i = i as i32;
        if digit != &9 {
            weights[(8 - i).abs() as usize] += 1;
            break
        } else {
            if i == 8 {
                return false;
            }
            weights[(8 - i).abs() as usize] = 0;
        }
    }
    return true;
}