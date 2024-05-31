use super::{Color, Direction, Piece, Ply, Square};
use crate::board::Board;

#[derive(Clone, PartialEq, Debug)]
pub struct King;

impl Eq for King {}

impl Piece for King {
    const WHITE_SYMBOL: &'static str = "♚";
    const BLACK_SYMBOL: &'static str = "♔";

    fn get_moveset(square: Square, _: &Board, _: Color) -> Vec<Ply> {
        let mut moveset = vec![
            Ply::new(square, square + Direction::North),
            Ply::new(square, square + Direction::East),
            Ply::new(square, square + Direction::South),
            Ply::new(square, square + Direction::West),
            Ply::new(square, square + Direction::NorthEast),
            Ply::new(square, square + Direction::NorthWest),
            Ply::new(square, square + Direction::SouthEast),
            Ply::new(square, square + Direction::SouthWest),
        ];

        if square == Square::from("e1") {
            moveset.push(
                Ply::builder(square, Square::from("g1"))
                    .castles(true)
                    .build(),
            );
            moveset.push(
                Ply::builder(square, Square::from("c1"))
                    .castles(true)
                    .build(),
            );
        }

        if square == Square::from("e8") {
            moveset.push(
                Ply::builder(square, Square::from("g8"))
                    .castles(true)
                    .build(),
            );
            moveset.push(
                Ply::builder(square, Square::from("c8"))
                    .castles(true)
                    .build(),
            );
        }

        moveset
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{Color, King, Piece, Ply, Square};
    use crate::board::Board;
    use crate::board::Kind;
    use pretty_assertions::{assert_eq, assert_ne};
    use std::collections::HashSet;

    #[test]
    fn test_king_derived_traits() {
        let piece = King {};
        dbg!(&piece);

        assert_eq!(piece, piece.clone());
    }

    #[test]
    fn test_king_display_white() {
        let output = King::WHITE_SYMBOL;
        let correct = "♚";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_king_display_black() {
        let output = King::BLACK_SYMBOL;
        let correct = "♔";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_king_get_piece_symbol_white() {
        let piece = Kind::King(Color::White);
        let correct = "♚";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_king_get_piece_symbol_black() {
        let piece = Kind::King(Color::Black);
        let correct = "♔";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_king_eq() {
        let left = Kind::King(Color::White);
        let right = Kind::King(Color::White);

        assert_eq!(left, right);
    }

    #[test]
    fn test_king_neq() {
        let left = Kind::King(Color::White);
        let right = Kind::King(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_king_neq_rev() {
        // Test if addition is commutative
        let right = Kind::King(Color::White);
        let left = Kind::King(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_king_get_moveset_white_b1() {
        let board = Board::construct_empty_board();
        let piece = Kind::King(Color::White);
        let start_square = Square::from("b1");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("b2")),
            Ply::new(start_square, Square::from("a2")),
            Ply::new(start_square, Square::from("c2")),
            Ply::new(start_square, Square::from("c1")),
            Ply::new(start_square, Square::from("a1")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_king_get_moveset_white_d4() {
        let board = Board::construct_empty_board();
        let piece = Kind::King(Color::White);
        let start_square = Square::from("d4");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("c3")),
            Ply::new(start_square, Square::from("d3")),
            Ply::new(start_square, Square::from("e3")),
            Ply::new(start_square, Square::from("c4")),
            Ply::new(start_square, Square::from("e4")),
            Ply::new(start_square, Square::from("c5")),
            Ply::new(start_square, Square::from("d5")),
            Ply::new(start_square, Square::from("e5")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_king_get_moveset_white_h6() {
        let board = Board::construct_empty_board();
        let piece = Kind::King(Color::White);
        let start_square = Square::from("h6");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("g5")),
            Ply::new(start_square, Square::from("h5")),
            Ply::new(start_square, Square::from("g6")),
            Ply::new(start_square, Square::from("g7")),
            Ply::new(start_square, Square::from("h7")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_king_get_moveset_black_b1() {
        let board = Board::construct_empty_board();
        let piece = Kind::King(Color::Black);
        let start_square = Square::from("b1");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("a1")),
            Ply::new(start_square, Square::from("c1")),
            Ply::new(start_square, Square::from("a2")),
            Ply::new(start_square, Square::from("b2")),
            Ply::new(start_square, Square::from("c2")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_king_get_moveset_black_d4() {
        let board = Board::construct_empty_board();
        let piece = Kind::King(Color::Black);
        let start_square = Square::from("d4");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("c3")),
            Ply::new(start_square, Square::from("d3")),
            Ply::new(start_square, Square::from("e3")),
            Ply::new(start_square, Square::from("c4")),
            Ply::new(start_square, Square::from("e4")),
            Ply::new(start_square, Square::from("c5")),
            Ply::new(start_square, Square::from("d5")),
            Ply::new(start_square, Square::from("e5")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_king_get_moveset_black_h6() {
        let board = Board::construct_empty_board();
        let piece = Kind::King(Color::Black);
        let start_square = Square::from("h6");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("g5")),
            Ply::new(start_square, Square::from("h5")),
            Ply::new(start_square, Square::from("g6")),
            Ply::new(start_square, Square::from("g7")),
            Ply::new(start_square, Square::from("h7")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_king_get_moveset_white_e1() {
        let board = Board::construct_empty_board();
        let piece = Kind::King(Color::White);
        let start_square = Square::from("e1");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("d1")),
            Ply::new(start_square, Square::from("d2")),
            Ply::new(start_square, Square::from("e2")),
            Ply::new(start_square, Square::from("f1")),
            Ply::new(start_square, Square::from("f2")),
            Ply::builder(start_square, Square::from("g1"))
                .castles(true)
                .build(),
            Ply::builder(start_square, Square::from("c1"))
                .castles(true)
                .build(),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_king_get_moveset_black_e1() {
        let board = Board::construct_empty_board();
        let piece = Kind::King(Color::White);
        let start_square = Square::from("e1");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("d1")),
            Ply::new(start_square, Square::from("d2")),
            Ply::new(start_square, Square::from("e2")),
            Ply::new(start_square, Square::from("f1")),
            Ply::new(start_square, Square::from("f2")),
            Ply::builder(start_square, Square::from("g1"))
                .castles(true)
                .build(),
            Ply::builder(start_square, Square::from("c1"))
                .castles(true)
                .build(),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_king_get_moveset_white_e8() {
        let board = Board::construct_empty_board();
        let piece = Kind::King(Color::White);
        let start_square = Square::from("e8");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("d8")),
            Ply::new(start_square, Square::from("d7")),
            Ply::new(start_square, Square::from("e7")),
            Ply::new(start_square, Square::from("f8")),
            Ply::new(start_square, Square::from("f7")),
            Ply::builder(start_square, Square::from("g8"))
                .castles(true)
                .build(),
            Ply::builder(start_square, Square::from("c8"))
                .castles(true)
                .build(),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_king_get_moveset_black_e8() {
        let board = Board::construct_empty_board();
        let piece = Kind::King(Color::White);
        let start_square = Square::from("e8");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("d8")),
            Ply::new(start_square, Square::from("d7")),
            Ply::new(start_square, Square::from("e7")),
            Ply::new(start_square, Square::from("f8")),
            Ply::new(start_square, Square::from("f7")),
            Ply::builder(start_square, Square::from("g8"))
                .castles(true)
                .build(),
            Ply::builder(start_square, Square::from("c8"))
                .castles(true)
                .build(),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }
}
