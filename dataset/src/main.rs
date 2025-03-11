use std::fs::write;
use std::collections::HashMap;
use serde_json::to_string_pretty;
mod dataset;

fn main() -> std::io::Result<()> {
    let mut ds: HashMap<String, i8> = HashMap::new(); 
    
    let board: [i8; 9] = dataset::initial_state();
    
    for action in dataset::actions(board) {
        let mut board = dataset::result(board, action);

        let optimal_move = dataset::minimax(board);
        ds.insert(dataset::board_state(board).to_string(), optimal_move);    
        board = dataset::result(board, optimal_move);

        generate_board(&mut ds, board);
    }

    let json = to_string_pretty(&ds).unwrap();

    write("dataset.json", json)?;

    Ok(())
}

// btw, we dont have to worry about getting the same position from different
// starting points, as the hashmap should replace the original one but they
// will be the same value, (i think ;D)
fn generate_board(ds: &mut HashMap<String, i8>, board: [i8; 9]) {
    for action in dataset::actions(board) {
        let mut board = dataset::result(board, action);
        if dataset::terminal(board) {
            return;
        }
        
        let optimal_move = dataset::minimax(board);
        ds.insert(dataset::board_state(board).to_string(), optimal_move);    
        board = dataset::result(board, optimal_move);

        generate_board(ds, board);
    }
}