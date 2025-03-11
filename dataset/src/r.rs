// use std::fs::write;
use std::collections::HashMap;
// use serde_json::to_string_pretty;
mod dataset;

use gcd::Gcd;

fn main() -> std::io::Result<()> {
    
    for i in 0..10 {
        for j in 0..10 {
            if i == j {
                continue
            }
            let i = i as i32;
            let j = j as i32;
            
            println!("trying values {i}, {j}");
            
            let mut ds: HashMap<i32, i8> = HashMap::new(); 
            let board: [i8; 9] = dataset::initial_state();
            for action in dataset::actions(board) {
                let mut board = dataset::result(board, action);

                let optimal_move = dataset::minimax(board);
                ds.insert(dataset::board_state(board, i, j), optimal_move);    
                board = dataset::result(board, optimal_move);

                generate_board(&mut ds, board, i, j);
            }
            
            let mut gcds: i32 = 0;
            let mut d = [0, 0, 0, 0, 0, 0, 0, 0, 0];
            for (board, _optimal_move) in &ds {
                let a = board % 9;
                d[a as usize] += 1;
                
                if (*board as u64).gcd(9) > 1{
                    gcds += 1;
                }
            }
            
            let mut b: i8 = 0;
            for val in &d {
                if *val > 20 {
                    b += 1;
                }
            }
            println!("{:?}, {}", d, gcds);
            // if b == 9 {
            //     for (board, _optimal_move) in &ds {
            //         println!("{}, {}", board % 9, board);
            //     }
            //     println!("{:?}", d);
            //     panic!("values {i}, {j} successfully maps");
            // }
        }
    }

    Ok(())
}

// btw, we dont have to worry about getting the same position from different
// starting points, as the hashmap should replace the original one but they
// will be the same value, (i think ;D)
fn generate_board(ds: &mut HashMap<i32, i8>, board: [i8; 9], i: i32, j: i32) {
    for action in dataset::actions(board) {
        let mut board = dataset::result(board, action);
        if dataset::terminal(board) {
            return;
        }
        
        let optimal_move = dataset::minimax(board);
        ds.insert(dataset::board_state(board, i, j), optimal_move);    
        board = dataset::result(board, optimal_move);

        generate_board(ds, board, i, j);
    }
}