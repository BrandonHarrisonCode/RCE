#[cfg(test)]
pub mod tests {
    extern crate test;

    use crate::board::boardbuilder::BoardBuilder;
    use crate::board::Board;
    use pretty_assertions::assert_eq;
    use std::time::Instant;
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
    /// Check that two vectors are equal after sorting and deduping them.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The first vector to compare.
    /// * `rhs` - The second vector to compare.
    ///
    /// # Panics
    ///
    /// Panics if the two vectors are not equal after sorting and deduping them.
    ///
    /// # Example
    /// ```
    /// use crate::utils::tests::check_unique_equality;
    ///
    /// let lhs = vec![1, 2, 3, 4, 5];
    /// let rhs = vec![5, 4, 3, 2, 1];
    /// check_unique_equality(lhs, rhs);
    /// ```
    pub fn check_unique_equality<T: Ord + std::fmt::Debug>(mut lhs: Vec<T>, mut rhs: Vec<T>) {
        (lhs, rhs) = sort_and_dedup(lhs, rhs);
        assert_eq!(lhs, rhs);
    }

    /// Returns the total number of moves reachable from the current position.
    ///
    /// # Arguments
    ///
    /// * `board` - The board to analyze.
    /// * `depth` - The depth to search.
    pub fn perft(board: &mut Board, depth: u32) -> u64 {
        let start = Instant::now();
        let total_nodes = perft_helper(board, depth, depth);
        let time_elapsed = start.elapsed().as_secs_f64();
        let time_elapsed_in_ms = start.elapsed().as_millis();

        assert!(time_elapsed > 0.0, "Zero time elapsed during perft!");

        println!("==========================");
        println!("Total time (ms) : {time_elapsed_in_ms}");
        println!("Nodes searched  : {total_nodes}");
        println!("Nodes / second  : {}", total_nodes as f64 / time_elapsed);

        total_nodes
    }

    /// Runs perft and summarize the first level of moves.
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
        let mut board = BoardBuilder::construct_starting_board().build();
        let nodes = perft(&mut board, 1);
        assert_eq!(nodes, 20);
    }

    #[test]
    fn test_perft_depth_2() {
        let mut board = BoardBuilder::construct_starting_board().build();
        let nodes = perft(&mut board, 2);
        assert_eq!(nodes, 400);
    }

    #[test]
    fn test_perft_depth_3() {
        let mut board = BoardBuilder::construct_starting_board().build();
        let nodes = perft(&mut board, 3);
        assert_eq!(nodes, 8902);
    }

    #[test]
    fn test_perft_depth_4() {
        let mut board = BoardBuilder::construct_starting_board().build();
        let nodes = perft(&mut board, 4);
        assert_eq!(nodes, 197_281);
    }

    #[test]
    #[ignore]
    fn test_perft_depth_5() {
        let mut board = BoardBuilder::construct_starting_board().build();
        let nodes = perft(&mut board, 5);
        assert_eq!(nodes, 4_865_609);
    }

    #[test]
    #[ignore]
    fn test_perft_depth_6() {
        let mut board = BoardBuilder::construct_starting_board().build();
        let nodes = perft(&mut board, 6);
        assert_eq!(nodes, 119_060_324);
    }

    #[bench]
    fn bench_perft_depth_1(bencher: &mut Bencher) {
        let mut board = BoardBuilder::construct_starting_board().build();
        bencher.iter(|| perft(&mut board, 1));
    }

    #[bench]
    fn bench_perft_depth_2(bencher: &mut Bencher) {
        let mut board = BoardBuilder::construct_starting_board().build();
        bencher.iter(|| perft(&mut board, 2));
    }

    #[bench]
    fn bench_perft_depth_3(bencher: &mut Bencher) {
        let mut board = BoardBuilder::construct_starting_board().build();
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

    #[test]
    fn test_perft_from_position_5() {
        let mut board =
            Board::from_fen("rn1qkbnr/p1pppppp/bp6/8/8/N3PN2/PPPP1PPP/R1BQKB1R b KQkq - 0 3");
        let nodes = perft(&mut board, 2);
        assert_eq!(nodes, 636);
    }

    #[test]
    fn test_perft_from_position_6() {
        let mut board =
            Board::from_fen("rn1qkbnr/p1pppppp/1p6/8/8/N3PN2/PPPP1PPP/R1BQKb1R w KQkq - 0 4");
        let nodes = perft(&mut board, 1);
        assert_eq!(nodes, 24);
    }

    #[test]
    fn test_perft_from_position_7() {
        let mut board =
            Board::from_fen("rnb1kbnr/1p1p1ppp/8/2p5/p1QPP3/2N4q/PPP1NP2/R1B1K1R1 w Qkq - 1 13");
        let nodes = perft(&mut board, 2);
        assert_eq!(nodes, 1515);
    }

    #[test]
    fn test_perft_from_position_8() {
        let mut board = Board::from_fen("rnb1kqRQ/1p1p3p/8/2p5/p3P3/8/PPP1NP2/R3K3 b Q - 2 24");
        let nodes = perft(&mut board, 2);
        assert_eq!(nodes, 591);
    }

    #[test]
    fn test_perft_from_position_9() {
        let mut board = Board::from_fen("rnb1k1qQ/1p1p3p/8/2p5/p3P3/8/PPP1NP2/R3K3 w Q - 0 25");
        let nodes = perft(&mut board, 1);
        assert_eq!(nodes, 28);
    }

    #[test]
    #[ignore]
    // Kiwipete position
    fn test_perft_from_position_10() {
        let mut board =
            Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -");
        let nodes = perft(&mut board, 4);
        assert_eq!(nodes, 4085603);
    }

    #[test]
    fn test_perft_from_position_11() {
        let mut board =
            Board::from_fen("r3k2r/p1ppqNb1/bn2pnp1/3P4/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 1");
        let nodes = perft(&mut board, 3);
        assert_eq!(nodes, 88799);
    }

    #[test]
    fn test_perft_from_position_12() {
        let mut board =
            Board::from_fen("r3k2r/pbppqNb1/1n2pnp1/3P4/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 2");
        let nodes = perft(&mut board, 2);
        assert_eq!(nodes, 2050);
    }

    #[test]
    fn test_perft_from_position_13() {
        let mut board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -");
        let nodes = perft(&mut board, 5);
        assert_eq!(nodes, 674624);
    }

    #[test]
    fn test_perft_from_position_14() {
        let mut board =
            Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
        let nodes = perft(&mut board, 4);
        assert_eq!(nodes, 422333);
    }

    #[test]
    fn test_perft_from_position_15() {
        let mut board =
            Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
        let nodes = perft(&mut board, 3);
        assert_eq!(nodes, 62379);
    }

    #[test]
    fn test_perft_from_position_16() {
        let mut board = Board::from_fen(
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
        );
        let nodes = perft(&mut board, 3);
        assert_eq!(nodes, 89890);
    }
}
