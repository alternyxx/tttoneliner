// idek how i would write a test for the other two functions
// considering they're recursive functions that do what they need
use dataset::dataset::*;

#[cfg(test)]
mod tests {
    use super::board_state;

    #[test]
    fn test_board_state() {
        let expected = 111212131;
        let result = board_state(&[
            0, 0, 0,
            1, 0, 1,
            0, 2, 0,
        ]);

        assert_eq!(expected, result);
    }
}