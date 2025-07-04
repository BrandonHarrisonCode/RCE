use super::super::bitboard::Bitboard;
use super::{Bishop, Color, Kind, Piece, PieceMoveset, Ply, Rook, Square};
use crate::board::Board;

#[derive(Clone, PartialEq, Debug)]
pub struct Queen;

impl Eq for Queen {}

impl Piece for Queen {
    const WHITE_SYMBOL: &'static str = "♛";
    const BLACK_SYMBOL: &'static str = "♕";

    fn get_moveset(square: Square, board: &Board, color: Color) -> PieceMoveset {
        let same_pieces = match color {
            Color::White => board.bitboards.white_pieces,
            Color::Black => board.bitboards.black_pieces,
        };

        let move_mask = Self::get_attacks(square, board.bitboards.all_pieces) & !same_pieces;
        move_mask
            .into_iter()
            .map(|dest| {
                Ply::builder(square, dest, Kind::Queen(color))
                    .captured(board.get_piece(dest))
                    .build()
            })
            .collect()
    }
}

impl Queen {
    pub fn get_attacks(square: Square, blockers: Bitboard) -> Bitboard {
        Rook::get_attacks_wrapper(square, blockers) | Bishop::get_attacks_wrapper(square, blockers)
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{Color, Piece, Ply, Queen, Square};
    use crate::board::boardbuilder::BoardBuilder;
    use crate::board::Kind;
    use std::collections::HashSet;

    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_queen_derived_traits() {
        let piece = Queen {};
        dbg!(&piece);

        assert_eq!(piece, piece.clone());
    }

    #[test]
    fn test_queen_display_white() {
        let output = Queen::WHITE_SYMBOL;
        let correct = "♛";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_queen_display_black() {
        let output = Queen::BLACK_SYMBOL;
        let correct = "♕";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_queen_get_piece_symbol_white() {
        let piece = Kind::Queen(Color::White);
        let correct = "♛";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_queen_get_piece_symbol_black() {
        let piece = Kind::Queen(Color::Black);
        let correct = "♕";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_queen_eq() {
        let left = Kind::Queen(Color::White);
        let right = Kind::Queen(Color::White);

        assert_eq!(left, right);
    }

    #[test]
    fn test_queen_neq() {
        let left = Kind::Queen(Color::White);
        let right = Kind::Queen(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_queen_neq_rev() {
        // Test if addition is commutative
        let right = Kind::Queen(Color::White);
        let left = Kind::Queen(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_queen_get_moveset_white_a1() {
        let board = BoardBuilder::construct_empty_board().build();
        let piece = Kind::Queen(Color::White);
        let start_square = Square::from("a1");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("b2"), piece),
            Ply::new(start_square, Square::from("c3"), piece),
            Ply::new(start_square, Square::from("d4"), piece),
            Ply::new(start_square, Square::from("e5"), piece),
            Ply::new(start_square, Square::from("f6"), piece),
            Ply::new(start_square, Square::from("g7"), piece),
            Ply::new(start_square, Square::from("h8"), piece),
            Ply::new(start_square, Square::from("a2"), piece),
            Ply::new(start_square, Square::from("a3"), piece),
            Ply::new(start_square, Square::from("a4"), piece),
            Ply::new(start_square, Square::from("a5"), piece),
            Ply::new(start_square, Square::from("a6"), piece),
            Ply::new(start_square, Square::from("a7"), piece),
            Ply::new(start_square, Square::from("a8"), piece),
            Ply::new(start_square, Square::from("b1"), piece),
            Ply::new(start_square, Square::from("c1"), piece),
            Ply::new(start_square, Square::from("d1"), piece),
            Ply::new(start_square, Square::from("e1"), piece),
            Ply::new(start_square, Square::from("f1"), piece),
            Ply::new(start_square, Square::from("g1"), piece),
            Ply::new(start_square, Square::from("h1"), piece),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_queen_get_moveset_white_b1() {
        let board = BoardBuilder::construct_empty_board().build();
        let piece = Kind::Queen(Color::White);
        let start_square = Square::from("b1");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("c2"), piece),
            Ply::new(start_square, Square::from("d3"), piece),
            Ply::new(start_square, Square::from("e4"), piece),
            Ply::new(start_square, Square::from("f5"), piece),
            Ply::new(start_square, Square::from("g6"), piece),
            Ply::new(start_square, Square::from("h7"), piece),
            Ply::new(start_square, Square::from("a2"), piece),
            Ply::new(start_square, Square::from("b2"), piece),
            Ply::new(start_square, Square::from("b3"), piece),
            Ply::new(start_square, Square::from("b4"), piece),
            Ply::new(start_square, Square::from("b5"), piece),
            Ply::new(start_square, Square::from("b6"), piece),
            Ply::new(start_square, Square::from("b7"), piece),
            Ply::new(start_square, Square::from("b8"), piece),
            Ply::new(start_square, Square::from("a1"), piece),
            Ply::new(start_square, Square::from("c1"), piece),
            Ply::new(start_square, Square::from("d1"), piece),
            Ply::new(start_square, Square::from("e1"), piece),
            Ply::new(start_square, Square::from("f1"), piece),
            Ply::new(start_square, Square::from("g1"), piece),
            Ply::new(start_square, Square::from("h1"), piece),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_queen_get_moveset_white_e4() {
        let board = BoardBuilder::construct_empty_board().build();
        let piece = Kind::Queen(Color::White);
        let start_square = Square::from("e4");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("f5"), piece),
            Ply::new(start_square, Square::from("g6"), piece),
            Ply::new(start_square, Square::from("h7"), piece),
            Ply::new(start_square, Square::from("d5"), piece),
            Ply::new(start_square, Square::from("c6"), piece),
            Ply::new(start_square, Square::from("b7"), piece),
            Ply::new(start_square, Square::from("a8"), piece),
            Ply::new(start_square, Square::from("b1"), piece),
            Ply::new(start_square, Square::from("c2"), piece),
            Ply::new(start_square, Square::from("d3"), piece),
            Ply::new(start_square, Square::from("f3"), piece),
            Ply::new(start_square, Square::from("g2"), piece),
            Ply::new(start_square, Square::from("h1"), piece),
            Ply::new(start_square, Square::from("e1"), piece),
            Ply::new(start_square, Square::from("e2"), piece),
            Ply::new(start_square, Square::from("e3"), piece),
            Ply::new(start_square, Square::from("e5"), piece),
            Ply::new(start_square, Square::from("e6"), piece),
            Ply::new(start_square, Square::from("e7"), piece),
            Ply::new(start_square, Square::from("e8"), piece),
            Ply::new(start_square, Square::from("a4"), piece),
            Ply::new(start_square, Square::from("b4"), piece),
            Ply::new(start_square, Square::from("c4"), piece),
            Ply::new(start_square, Square::from("d4"), piece),
            Ply::new(start_square, Square::from("f4"), piece),
            Ply::new(start_square, Square::from("g4"), piece),
            Ply::new(start_square, Square::from("h4"), piece),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_queen_get_moveset_white_d4() {
        let board = BoardBuilder::construct_empty_board().build();
        let piece = Kind::Queen(Color::White);
        let start_square = Square::from("d4");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("e5"), piece),
            Ply::new(start_square, Square::from("f6"), piece),
            Ply::new(start_square, Square::from("g7"), piece),
            Ply::new(start_square, Square::from("h8"), piece),
            Ply::new(start_square, Square::from("c5"), piece),
            Ply::new(start_square, Square::from("b6"), piece),
            Ply::new(start_square, Square::from("a7"), piece),
            Ply::new(start_square, Square::from("a1"), piece),
            Ply::new(start_square, Square::from("b2"), piece),
            Ply::new(start_square, Square::from("c3"), piece),
            Ply::new(start_square, Square::from("e3"), piece),
            Ply::new(start_square, Square::from("f2"), piece),
            Ply::new(start_square, Square::from("g1"), piece),
            Ply::new(start_square, Square::from("d1"), piece),
            Ply::new(start_square, Square::from("d2"), piece),
            Ply::new(start_square, Square::from("d3"), piece),
            Ply::new(start_square, Square::from("d5"), piece),
            Ply::new(start_square, Square::from("d6"), piece),
            Ply::new(start_square, Square::from("d7"), piece),
            Ply::new(start_square, Square::from("d8"), piece),
            Ply::new(start_square, Square::from("a4"), piece),
            Ply::new(start_square, Square::from("b4"), piece),
            Ply::new(start_square, Square::from("c4"), piece),
            Ply::new(start_square, Square::from("e4"), piece),
            Ply::new(start_square, Square::from("f4"), piece),
            Ply::new(start_square, Square::from("g4"), piece),
            Ply::new(start_square, Square::from("h4"), piece),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_queen_get_moveset_white_g6() {
        let board = BoardBuilder::construct_empty_board().build();
        let piece = Kind::Queen(Color::White);
        let start_square = Square::from("g6");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("h7"), piece),
            Ply::new(start_square, Square::from("h5"), piece),
            Ply::new(start_square, Square::from("f7"), piece),
            Ply::new(start_square, Square::from("e8"), piece),
            Ply::new(start_square, Square::from("f5"), piece),
            Ply::new(start_square, Square::from("e4"), piece),
            Ply::new(start_square, Square::from("d3"), piece),
            Ply::new(start_square, Square::from("c2"), piece),
            Ply::new(start_square, Square::from("b1"), piece),
            Ply::new(start_square, Square::from("g1"), piece),
            Ply::new(start_square, Square::from("g2"), piece),
            Ply::new(start_square, Square::from("g3"), piece),
            Ply::new(start_square, Square::from("g4"), piece),
            Ply::new(start_square, Square::from("g5"), piece),
            Ply::new(start_square, Square::from("g7"), piece),
            Ply::new(start_square, Square::from("g8"), piece),
            Ply::new(start_square, Square::from("a6"), piece),
            Ply::new(start_square, Square::from("b6"), piece),
            Ply::new(start_square, Square::from("c6"), piece),
            Ply::new(start_square, Square::from("d6"), piece),
            Ply::new(start_square, Square::from("e6"), piece),
            Ply::new(start_square, Square::from("f6"), piece),
            Ply::new(start_square, Square::from("h6"), piece),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_queen_get_moveset_white_h6() {
        let board = BoardBuilder::construct_empty_board().build();
        let piece = Kind::Queen(Color::White);
        let start_square = Square::from("h6");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("g7"), piece),
            Ply::new(start_square, Square::from("f8"), piece),
            Ply::new(start_square, Square::from("g5"), piece),
            Ply::new(start_square, Square::from("f4"), piece),
            Ply::new(start_square, Square::from("e3"), piece),
            Ply::new(start_square, Square::from("d2"), piece),
            Ply::new(start_square, Square::from("c1"), piece),
            Ply::new(start_square, Square::from("h1"), piece),
            Ply::new(start_square, Square::from("h2"), piece),
            Ply::new(start_square, Square::from("h3"), piece),
            Ply::new(start_square, Square::from("h4"), piece),
            Ply::new(start_square, Square::from("h5"), piece),
            Ply::new(start_square, Square::from("h7"), piece),
            Ply::new(start_square, Square::from("h8"), piece),
            Ply::new(start_square, Square::from("a6"), piece),
            Ply::new(start_square, Square::from("b6"), piece),
            Ply::new(start_square, Square::from("c6"), piece),
            Ply::new(start_square, Square::from("d6"), piece),
            Ply::new(start_square, Square::from("e6"), piece),
            Ply::new(start_square, Square::from("f6"), piece),
            Ply::new(start_square, Square::from("g6"), piece),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_queen_get_moveset_black_a1() {
        let board = BoardBuilder::construct_empty_board().build();
        let piece = Kind::Queen(Color::Black);
        let start_square = Square::from("a1");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("b2"), piece),
            Ply::new(start_square, Square::from("c3"), piece),
            Ply::new(start_square, Square::from("d4"), piece),
            Ply::new(start_square, Square::from("e5"), piece),
            Ply::new(start_square, Square::from("f6"), piece),
            Ply::new(start_square, Square::from("g7"), piece),
            Ply::new(start_square, Square::from("h8"), piece),
            Ply::new(start_square, Square::from("a2"), piece),
            Ply::new(start_square, Square::from("a3"), piece),
            Ply::new(start_square, Square::from("a4"), piece),
            Ply::new(start_square, Square::from("a5"), piece),
            Ply::new(start_square, Square::from("a6"), piece),
            Ply::new(start_square, Square::from("a7"), piece),
            Ply::new(start_square, Square::from("a8"), piece),
            Ply::new(start_square, Square::from("b1"), piece),
            Ply::new(start_square, Square::from("c1"), piece),
            Ply::new(start_square, Square::from("d1"), piece),
            Ply::new(start_square, Square::from("e1"), piece),
            Ply::new(start_square, Square::from("f1"), piece),
            Ply::new(start_square, Square::from("g1"), piece),
            Ply::new(start_square, Square::from("h1"), piece),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_queen_get_moveset_black_b1() {
        let board = BoardBuilder::construct_empty_board().build();
        let piece = Kind::Queen(Color::Black);
        let start_square = Square::from("b1");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("c2"), piece),
            Ply::new(start_square, Square::from("d3"), piece),
            Ply::new(start_square, Square::from("e4"), piece),
            Ply::new(start_square, Square::from("f5"), piece),
            Ply::new(start_square, Square::from("g6"), piece),
            Ply::new(start_square, Square::from("h7"), piece),
            Ply::new(start_square, Square::from("a2"), piece),
            Ply::new(start_square, Square::from("b2"), piece),
            Ply::new(start_square, Square::from("b3"), piece),
            Ply::new(start_square, Square::from("b4"), piece),
            Ply::new(start_square, Square::from("b5"), piece),
            Ply::new(start_square, Square::from("b6"), piece),
            Ply::new(start_square, Square::from("b7"), piece),
            Ply::new(start_square, Square::from("b8"), piece),
            Ply::new(start_square, Square::from("a1"), piece),
            Ply::new(start_square, Square::from("c1"), piece),
            Ply::new(start_square, Square::from("d1"), piece),
            Ply::new(start_square, Square::from("e1"), piece),
            Ply::new(start_square, Square::from("f1"), piece),
            Ply::new(start_square, Square::from("g1"), piece),
            Ply::new(start_square, Square::from("h1"), piece),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_queen_get_moveset_black_e4() {
        let board = BoardBuilder::construct_empty_board().build();
        let piece = Kind::Queen(Color::Black);
        let start_square = Square::from("e4");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("f5"), piece),
            Ply::new(start_square, Square::from("g6"), piece),
            Ply::new(start_square, Square::from("h7"), piece),
            Ply::new(start_square, Square::from("d5"), piece),
            Ply::new(start_square, Square::from("c6"), piece),
            Ply::new(start_square, Square::from("b7"), piece),
            Ply::new(start_square, Square::from("a8"), piece),
            Ply::new(start_square, Square::from("b1"), piece),
            Ply::new(start_square, Square::from("c2"), piece),
            Ply::new(start_square, Square::from("d3"), piece),
            Ply::new(start_square, Square::from("f3"), piece),
            Ply::new(start_square, Square::from("g2"), piece),
            Ply::new(start_square, Square::from("h1"), piece),
            Ply::new(start_square, Square::from("e1"), piece),
            Ply::new(start_square, Square::from("e2"), piece),
            Ply::new(start_square, Square::from("e3"), piece),
            Ply::new(start_square, Square::from("e5"), piece),
            Ply::new(start_square, Square::from("e6"), piece),
            Ply::new(start_square, Square::from("e7"), piece),
            Ply::new(start_square, Square::from("e8"), piece),
            Ply::new(start_square, Square::from("a4"), piece),
            Ply::new(start_square, Square::from("b4"), piece),
            Ply::new(start_square, Square::from("c4"), piece),
            Ply::new(start_square, Square::from("d4"), piece),
            Ply::new(start_square, Square::from("f4"), piece),
            Ply::new(start_square, Square::from("g4"), piece),
            Ply::new(start_square, Square::from("h4"), piece),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_queen_get_moveset_black_d4() {
        let board = BoardBuilder::construct_empty_board().build();
        let piece = Kind::Queen(Color::Black);
        let start_square = Square::from("d4");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("e5"), piece),
            Ply::new(start_square, Square::from("f6"), piece),
            Ply::new(start_square, Square::from("g7"), piece),
            Ply::new(start_square, Square::from("h8"), piece),
            Ply::new(start_square, Square::from("c5"), piece),
            Ply::new(start_square, Square::from("b6"), piece),
            Ply::new(start_square, Square::from("a7"), piece),
            Ply::new(start_square, Square::from("a1"), piece),
            Ply::new(start_square, Square::from("b2"), piece),
            Ply::new(start_square, Square::from("c3"), piece),
            Ply::new(start_square, Square::from("e3"), piece),
            Ply::new(start_square, Square::from("f2"), piece),
            Ply::new(start_square, Square::from("g1"), piece),
            Ply::new(start_square, Square::from("d1"), piece),
            Ply::new(start_square, Square::from("d2"), piece),
            Ply::new(start_square, Square::from("d3"), piece),
            Ply::new(start_square, Square::from("d5"), piece),
            Ply::new(start_square, Square::from("d6"), piece),
            Ply::new(start_square, Square::from("d7"), piece),
            Ply::new(start_square, Square::from("d8"), piece),
            Ply::new(start_square, Square::from("a4"), piece),
            Ply::new(start_square, Square::from("b4"), piece),
            Ply::new(start_square, Square::from("c4"), piece),
            Ply::new(start_square, Square::from("e4"), piece),
            Ply::new(start_square, Square::from("f4"), piece),
            Ply::new(start_square, Square::from("g4"), piece),
            Ply::new(start_square, Square::from("h4"), piece),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_queen_get_moveset_black_g6() {
        let board = BoardBuilder::construct_empty_board().build();
        let piece = Kind::Queen(Color::Black);
        let start_square = Square::from("g6");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("h7"), piece),
            Ply::new(start_square, Square::from("h5"), piece),
            Ply::new(start_square, Square::from("f7"), piece),
            Ply::new(start_square, Square::from("e8"), piece),
            Ply::new(start_square, Square::from("f5"), piece),
            Ply::new(start_square, Square::from("e4"), piece),
            Ply::new(start_square, Square::from("d3"), piece),
            Ply::new(start_square, Square::from("c2"), piece),
            Ply::new(start_square, Square::from("b1"), piece),
            Ply::new(start_square, Square::from("g1"), piece),
            Ply::new(start_square, Square::from("g2"), piece),
            Ply::new(start_square, Square::from("g3"), piece),
            Ply::new(start_square, Square::from("g4"), piece),
            Ply::new(start_square, Square::from("g5"), piece),
            Ply::new(start_square, Square::from("g7"), piece),
            Ply::new(start_square, Square::from("g8"), piece),
            Ply::new(start_square, Square::from("a6"), piece),
            Ply::new(start_square, Square::from("b6"), piece),
            Ply::new(start_square, Square::from("c6"), piece),
            Ply::new(start_square, Square::from("d6"), piece),
            Ply::new(start_square, Square::from("e6"), piece),
            Ply::new(start_square, Square::from("f6"), piece),
            Ply::new(start_square, Square::from("h6"), piece),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_queen_get_moveset_black_h6() {
        let board = BoardBuilder::construct_empty_board().build();
        let piece = Kind::Queen(Color::Black);
        let start_square = Square::from("h6");

        let result = piece.get_moveset(start_square, &board);
        let correct = vec![
            Ply::new(start_square, Square::from("g7"), piece),
            Ply::new(start_square, Square::from("f8"), piece),
            Ply::new(start_square, Square::from("g5"), piece),
            Ply::new(start_square, Square::from("f4"), piece),
            Ply::new(start_square, Square::from("e3"), piece),
            Ply::new(start_square, Square::from("d2"), piece),
            Ply::new(start_square, Square::from("c1"), piece),
            Ply::new(start_square, Square::from("h1"), piece),
            Ply::new(start_square, Square::from("h2"), piece),
            Ply::new(start_square, Square::from("h3"), piece),
            Ply::new(start_square, Square::from("h4"), piece),
            Ply::new(start_square, Square::from("h5"), piece),
            Ply::new(start_square, Square::from("h7"), piece),
            Ply::new(start_square, Square::from("h8"), piece),
            Ply::new(start_square, Square::from("a6"), piece),
            Ply::new(start_square, Square::from("b6"), piece),
            Ply::new(start_square, Square::from("c6"), piece),
            Ply::new(start_square, Square::from("d6"), piece),
            Ply::new(start_square, Square::from("e6"), piece),
            Ply::new(start_square, Square::from("f6"), piece),
            Ply::new(start_square, Square::from("g6"), piece),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }
}
