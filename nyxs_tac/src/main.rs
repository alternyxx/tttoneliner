use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use gcd::Gcd;
use serde_json;

fn main() -> std::io::Result<()> {
    let mut data = String::new();
    {
        let mut dataset = File::open("./src/datasets/dataset.json")?;
        dataset.read_to_string(&mut data)?;
    }
    
    let data: serde_json::Value = serde_json::from_str(&data).expect("uhm");

    // Make the object into a hashmap for no reason
    let mut optimal_moves: HashMap<usize, usize> = HashMap::new();
    for (board, optimal_move) in data.as_object().unwrap() {
        optimal_moves.insert(
            board.parse::<usize>().unwrap(), 
            optimal_move.as_i64().unwrap() as usize
        );
    }   
    drop(data);

    let got_it = optimal_moves.len();
    for i in 1.. {
        if i % 3 == 0 {
            continue // i may remove this, we'll see
        }
        let mut s: usize = 0;
        let mut gcds: usize = 0;
        for (board, optimal_move) in &optimal_moves {
            if ((board * i) as usize) % 9 == *optimal_move {
                s += 1;
            }

            if (board * i).gcd(9) == 1 {
                gcds += 1;
            }
        }
        if s >= got_it {
            println!("value {} reached", i);
            break
        } else if s >= 80 {
            println!("value of {} reached greather than {} mappings", i, s);
            break
        } else {
            println!("{} mapped {} with denominators having reached {}", i, s, gcds);
        }
    }

    Ok(())
}