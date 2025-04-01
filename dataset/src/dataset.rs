use std::collections::HashMap;
use crate::tictactoe;

pub fn generate_dataset() -> HashMap<String, i8> {
    let mut ds: HashMap<String, i8> = HashMap::new(); 
    
    let board: &[i8; 9] = &tictactoe::initial_state();
    
    generate_board(&mut ds, board);

    ds
}

pub fn board_state(board: &[i8; 9]) -> i32 {
    let mut o: i32 = 0;
    
    for (i, pos) in board.iter().enumerate() {
        let val: i32;
        match *pos {
            0 => val = 1,
            1 => val = 5,
            2 => val = 9,
            _ => panic!("wahh"),
        }
        let tmp = i as i32;
        let digit = (tmp - 8).abs();
        o += val * i32::pow(10, digit as u32);
    }
 
    o
}

// btw, we dont have to worry about getting the same position from different
// starting points, as the hashmap should replace the original one but they
// will be the same value, (i think ;D)
fn generate_board(ds: &mut HashMap<String, i8>, board: &[i8; 9]) {
    for action in tictactoe::actions(board) {
        let board = &tictactoe::result(board, action);
        if tictactoe::terminal(board) {
            return;
        }
        
        let optimal_move = tictactoe::minimax(board);
        ds.insert(board_state(board).to_string(), (8-optimal_move).abs());    
        let board = &tictactoe::result(board, optimal_move);

        generate_board(ds, board);
    }
}