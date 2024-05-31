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
}
