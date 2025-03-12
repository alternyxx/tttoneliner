use std::cmp;

pub fn initial_state() -> [i8; 9] {
    return [
        0, 0, 0,
        0, 0, 0,
        0, 0, 0,
    ];
}

pub fn result(board: [i8; 9], action: i8) -> [i8; 9] {
    let player_move = player(board);

    let mut new_board: [i8; 9] = board.clone();
    
    if board[action as usize] == 0 {
        new_board[action as usize] = player_move;
    } else {
        println!("{:?}", board);
        panic!("uh, action cant be done");
    }
    return new_board;
}

fn player(board: [i8; 9]) -> i8{
    if terminal(board) {
        return 0;
    }

    let mut player_one = 0;
    let mut player_two = 0;

    for cell in board {
        if cell == 1 {
            player_one += 1;
        } else if cell == 2 {
            player_two += 1;
        }
    }

    if player_one <= player_two {
        return 1;
    } else {
        return 2;
    }
}

fn winner(board: [i8; 9]) -> i8 {
    // checking rows if theyre the same
    for i in (0..9).step_by(3) {
        if board[i] == board[i+1] && board[i+1] == board[i+2] {
            return board[i];
        }
    }

    // checking columns if theyre the same
    for i in 0..3 {
        if board[i] == board[i+3] && board[i+3] == board[i+6] {
            return board[i];
        }
    }

    // checking if diagonals are the same
    if board[0] == board[4] && board[4] == board[8] {
        return board[0];
    } 
    if board[2] == board[4] && board[4] == board[6] {
        return board[2]
    }
    return 0;
}

pub fn terminal(board: [i8; 9]) -> bool {
    if winner(board) != 0 {
        return true;
    }

    for cell in board {
        if cell == 0 {
            return false;
        }
    }
    return true;
}


fn utility(board: [i8; 9]) -> i8 {
    let winner = winner(board);
    if winner == 1 {
        return 1;
    } else if winner == 2 {
        return -1;
    } else {
        return 0;
    }
}

pub fn actions(board: [i8; 9]) -> Vec<i8> {
    let mut possible_actions: Vec<i8> = Vec::new();
    
    if terminal(board) {
        return possible_actions;
    }

    for (i, cell) in board.iter().enumerate() {
        if *cell == 0 {
            possible_actions.push(i as i8);
        }
    }
    return possible_actions;
}

pub fn minimax(board: [i8; 9]) -> i8 {
    if board == initial_state() {
        panic!("uhm!");
    }
    
    if terminal(board) {
        return utility(board);
    }

    let current_player = player(board);
    let mut optimal_action: i8 = 0;

    if current_player == 1 {
        let mut current_optimal_value = -2;

        for action in actions(board) {
            let val = min_value(result(board, action));
            if val > current_optimal_value {
                current_optimal_value = val;
                optimal_action = action;
            }
        }
    } else {
        let mut current_optimal_value = 2;

        for action in actions(board) {
            let val = max_value(result(board, action));
            if val < current_optimal_value {
                current_optimal_value = val;
                optimal_action = action;
            }
        }
    }

    return optimal_action;
}

fn max_value(board: [i8; 9]) -> i8 {
    if terminal(board) {
        return utility(board);
    }

    let mut value: i8 = -2;
    for action in actions(board) {
        value = cmp::max(value, min_value(result(board, action)));
    }

    return value;
}

fn min_value(board: [i8; 9]) -> i8 {
    if terminal(board) {
        return utility(board);
    }

    let mut value: i8 = 2;
    for action in actions(board) {
        value = cmp::min(value, max_value(result(board, action)));
    }

    return value;
}