use dataset::tictactoe::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // sanity check
    fn test_initial() {
        let expected = [
            0, 0, 0,
            0, 0, 0,
            0, 0, 0,
        ];
        let result = initial_state();
        
        assert_eq!(expected, result);
    }

    #[test]
    fn test_middle_move_player_one() {
        let expected = [
            0, 0, 0,
            0, 1, 0,
            0, 0, 0,
        ];
        let result = result(&initial_state(), 4);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_middle_move_player_two() {
        let expected = [
            0, 0, 0,
            0, 2, 0,
            0, 0, 1,
        ];
        let result = result(&[
            0, 0, 0,
            0, 0, 0,
            0, 0, 1,
        ], 4);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_current_player_one() {
        let expected = 1;
        let result = player(&[
            1, 2, 1,
            2, 1, 1,
            2, 0, 2,
        ]);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_current_player_two() {
        let expected = 2;
        let result = player(&[
            1, 2, 1,
            2, 1, 1,
            2, 0, 0,
        ]);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_player_one_won_diagonal() {
        let expected = 1;
        let result = winner(&[
            1, 2, 0,
            2, 1, 0,
            0, 0, 1,
        ]);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_player_one_won_horizontal() {
        let expected = 1;
        let result = winner(&[
            2, 0, 0,
            0, 2, 0,
            1, 1, 1,
        ]);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_player_one_won_vertical() {
        let expected = 1;
        let result = winner(&[
            2, 0, 1,
            0, 2, 1,
            0, 0, 1,
        ]);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_player_two_won_diagonal() {
        let expected = 2;
        let result = winner(&[
            1, 2, 0,
            2, 1, 0,
            0, 0, 1,
        ]);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_player_two_won_horizontal() {
        let expected = 2;
        let result = winner(&[
            2, 0, 0,
            0, 2, 0,
            1, 1, 1,
        ]);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_player_two_won_vertical() {
        let expected = 2;
        let result = winner(&[
            2, 0, 1,
            0, 2, 1,
            0, 0, 1,
        ]);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_no_one_won() {
        let expected = 0;
        let result = winner(&[
            2, 1, 1,
            1, 2, 2,
            2, 1, 1,
        ]);

        assert_eq!(expected, result);
    }
}