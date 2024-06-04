#[cfg(test)]
pub mod tests {
    extern crate test;

    use crate::board::Board;
    use pretty_assertions::assert_eq;
    use test::Bencher;

    fn sort_and_dedup<T, U>(mut lhs: Vec<T>, mut rhs: Vec<U>) -> (Vec<T>, Vec<U>)
    where
        T: Ord,
        U: Ord,
    {
        lhs.sort();
        lhs.dedup();
        rhs.sort();
        rhs.dedup();

        (lhs, rhs)
    }

    #[allow(dead_code)]
    pub fn check_unique_equality<T: Ord + std::fmt::Debug>(mut lhs: Vec<T>, mut rhs: Vec<T>) {
        (lhs, rhs) = sort_and_dedup(lhs, rhs);
        assert_eq!(lhs, rhs);
    }

    pub fn perft(board: &mut Board, depth: u32) -> u64 {
        perft_helper(board, depth, depth)
    }

    fn perft_helper(board: &mut Board, depth: u32, max_depth: u32) -> u64 {
        if board.is_game_over() {
            return 0;
        }
        if depth == 0 {
            return 1;
        }

        let moves = board.get_legal_moves();
        if depth == 1 {
            return moves.len() as u64;
        }

        let mut nodes = 0;
        let mut output: Vec<String> = Vec::new();
        for mv in moves {
            board.make_move(mv);
            let new_nodes = perft_helper(board, depth - 1, max_depth);
            if depth == max_depth {
                output.push(format!("{mv}: {new_nodes}"));
            }
            nodes += new_nodes;
            board.unmake_move();
        }

        if depth == max_depth {
            output.sort();
            for line in output {
                println!("{line}");
            }
        }

        nodes
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
        assert_eq!(nodes, 197_281);
    }

    #[test]
    #[ignore]
    fn test_perft_depth_5() {
        let mut board = Board::construct_starting_board();
        let nodes = perft(&mut board, 5);
        assert_eq!(nodes, 4_865_609);
    }

    #[test]
    #[ignore]
    fn test_perft_depth_6() {
        let mut board = Board::construct_starting_board();
        let nodes = perft(&mut board, 6);
        assert_eq!(nodes, 119_060_324);
    }

    #[test]
    #[ignore]
    fn test_perft_depth_7() {
        let mut board = Board::construct_starting_board();
        let nodes = perft(&mut board, 7);
        assert_eq!(nodes, 3_195_901_860);
    }

    #[test]
    #[ignore]
    fn test_perft_depth_8() {
        let mut board = Board::construct_starting_board();
        let nodes = perft(&mut board, 8);
        assert_eq!(nodes, 84_998_978_956);
    }

    #[test]
    #[ignore]
    fn test_perft_depth_9() {
        let mut board = Board::construct_starting_board();
        let nodes = perft(&mut board, 9);
        assert_eq!(nodes, 2_439_530_234_167);
    }

    #[test]
    #[ignore]
    fn test_perft_depth_10() {
        let mut board = Board::construct_starting_board();
        let nodes = perft(&mut board, 10);
        assert_eq!(nodes, 69_352_859_712_417);
    }

    #[bench]
    fn bench_perft_depth_1(bencher: &mut Bencher) {
        let mut board = Board::construct_starting_board();
        bencher.iter(|| perft(&mut board, 1));
    }

    #[bench]
    fn bench_perft_depth_2(bencher: &mut Bencher) {
        let mut board = Board::construct_starting_board();
        bencher.iter(|| perft(&mut board, 2));
    }

    #[bench]
    fn bench_perft_depth_3(bencher: &mut Bencher) {
        let mut board = Board::construct_starting_board();
        bencher.iter(|| perft(&mut board, 3));
    }

    #[test]
    fn test_perft_from_position_1() {
        let mut board =
            Board::from_fen("rnbqkbnr/1ppppppp/p7/P7/8/8/1PPPPPPP/RNBQKBNR b KQkq - 0 2");
        let nodes = perft(&mut board, 2);
        assert_eq!(nodes, 380);
    }

    #[test]
    fn test_perft_from_position_2() {
        let mut board =
            Board::from_fen("rnbqkbnr/2pppppp/p7/Pp6/8/8/1PPPPPPP/RNBQKBNR w KQkq b6 0 3");
        let nodes = perft(&mut board, 1);
        assert_eq!(nodes, 22);
    }

    #[test]
    fn test_perft_from_position_3() {
        let mut board =
            Board::from_fen("rnbqkbnr/pppppppp/8/8/8/5P2/PPPPP1PP/RNBQKBNR b KQkq - 0 1");
        let nodes = perft(&mut board, 4);
        assert_eq!(nodes, 178_889);
    }

    #[test]
    fn test_perft_from_position_4() {
        let mut board =
            Board::from_fen("rnbqkbnr/pppp1ppp/8/4p3/8/5P2/PPPPP1PP/RNBQKBNR w KQkq - 0 2");
        let nodes = perft(&mut board, 3);
        assert_eq!(nodes, 11_679);
    }
}
