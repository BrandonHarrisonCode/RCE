#[allow(dead_code)]
pub fn debug_bitboard(bitboard: u64) -> String {
    debug_bitboard_helper(bitboard)
}

pub fn debug_bitboard_helper(bitboard: u64) -> String {
    let mut builder = String::new();
    let mask = 0xFF;

    builder.push_str(&format!("Debug bitboard: {bitboard:0>64b}\n"));
    for i in (0..8).rev() {
        builder.push_str(&format!("{:0>8b}\n", (bitboard >> (8 * i)) & mask));
    }

    builder
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use crate::board::Board;
    use indoc::indoc;
    use test::Bencher;

    pub fn perft(board: &mut Board, depth: u32) -> u64 {
        if depth == 0 {
            return 1;
        }

        let moves = board.get_legal_moves();
        if depth == 1 {
            return moves.len() as u64;
        }

        let mut nodes = 0;
        for mv in moves {
            board.make_move(mv);
            nodes += perft(board, depth - 1);
            board.unmake_move();
        }

        nodes
    }

    #[test]
    fn test_debug_bitboard_no_panic() {
        let bb = 0b_00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000;

        debug_bitboard(bb);
    }

    #[test]
    fn test_debug_bitboard1() {
        let bb = 0b_00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000;

        let msg = debug_bitboard_helper(bb);
        let correct = indoc! {"
            Debug bitboard: 0000000000000000000000000000000000000000000000001111111100000000
            00000000
            00000000
            00000000
            00000000
            00000000
            00000000
            11111111
            00000000
        "};

        assert_eq!(msg, correct);
    }

    #[test]
    fn test_debug_bitboard2() {
        let bb = 0b_01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000;

        let msg = debug_bitboard_helper(bb);
        let correct = indoc! {"
            Debug bitboard: 0100001000000000000000000000000000000000000000000000000000000000
            01000010
            00000000
            00000000
            00000000
            00000000
            00000000
            00000000
            00000000
        "};

        assert_eq!(msg, correct);
    }

    #[test]
    fn test_perft_depth_1() {
        let mut board = Board::construct_starting_board();
        let nodes = perft(&mut board, 1);
        assert_eq!(nodes, 20);
    }

    #[test]
    fn test_perft_depth_2() {
        let mut board = Board::construct_starting_board();
        let nodes = perft(&mut board, 2);
        assert_eq!(nodes, 400);
    }

    #[test]
    fn test_perft_depth_3() {
        let mut board = Board::construct_starting_board();
        let nodes = perft(&mut board, 3);
        assert_eq!(nodes, 8902);
    }

    #[test]
    fn test_perft_depth_4() {
        let mut board = Board::construct_starting_board();
        let nodes = perft(&mut board, 4);
        assert_eq!(nodes, 197281);
    }

    #[bench]
    fn bench_perft_depth_3(bencher: &mut Bencher) {
        let mut board = Board::construct_starting_board();
        bencher.iter(|| perft(&mut board, 3));
    }
}
