use super::{Color, Direction, Piece, Ply, Square};

#[derive(Clone, PartialEq, Debug)]
pub struct King;

impl Eq for King {}

impl Piece for King {
    const WHITE_SYMBOL: &'static str = "♚";
    const BLACK_SYMBOL: &'static str = "♔";

    fn get_moveset(square: Square, _: Color) -> Vec<Ply> {
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

        if square == Square::new("e1") {
            moveset.push(Ply::builder(square, Square::new("g1")).is_castles(true).build());
            moveset.push(Ply::builder(square, Square::new("c1")).is_castles(true).build());
        }

        if square == Square::new("e8") {
            moveset.push(Ply::builder(square, Square::new("g8")).is_castles(true).build());
            moveset.push(Ply::builder(square, Square::new("c8")).is_castles(true).build());
        }

        moveset
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::board::Kind;
    use super::{Color, King, Piece, Ply, Square};
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
        let piece = Kind::King(Color::White);
        let start_square = Square::new("b1");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("b2")),
            Ply::new(start_square, Square::new("a2")),
            Ply::new(start_square, Square::new("c2")),
            Ply::new(start_square, Square::new("c1")),
            Ply::new(start_square, Square::new("a1")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_king_get_moveset_white_d4() {
        let piece = Kind::King(Color::White);
        let start_square = Square::new("d4");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("c3")),
            Ply::new(start_square, Square::new("d3")),
            Ply::new(start_square, Square::new("e3")),
            Ply::new(start_square, Square::new("c4")),
            Ply::new(start_square, Square::new("e4")),
            Ply::new(start_square, Square::new("c5")),
            Ply::new(start_square, Square::new("d5")),
            Ply::new(start_square, Square::new("e5")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_king_get_moveset_white_h6() {
        let piece = Kind::King(Color::White);
        let start_square = Square::new("h6");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("g5")),
            Ply::new(start_square, Square::new("h5")),
            Ply::new(start_square, Square::new("g6")),
            Ply::new(start_square, Square::new("g7")),
            Ply::new(start_square, Square::new("h7")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_king_get_moveset_black_b1() {
        let piece = Kind::King(Color::Black);
        let start_square = Square::new("b1");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("a1")),
            Ply::new(start_square, Square::new("c1")),
            Ply::new(start_square, Square::new("a2")),
            Ply::new(start_square, Square::new("b2")),
            Ply::new(start_square, Square::new("c2")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_king_get_moveset_black_d4() {
        let piece = Kind::King(Color::Black);
        let start_square = Square::new("d4");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("c3")),
            Ply::new(start_square, Square::new("d3")),
            Ply::new(start_square, Square::new("e3")),
            Ply::new(start_square, Square::new("c4")),
            Ply::new(start_square, Square::new("e4")),
            Ply::new(start_square, Square::new("c5")),
            Ply::new(start_square, Square::new("d5")),
            Ply::new(start_square, Square::new("e5")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_king_get_moveset_black_h6() {
        let piece = Kind::King(Color::Black);
        let start_square = Square::new("h6");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("g5")),
            Ply::new(start_square, Square::new("h5")),
            Ply::new(start_square, Square::new("g6")),
            Ply::new(start_square, Square::new("g7")),
            Ply::new(start_square, Square::new("h7")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_king_get_moveset_white_e1() {
        let piece = Kind::King(Color::White);
        let start_square = Square::new("e1");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("d1")),
            Ply::new(start_square, Square::new("d2")),
            Ply::new(start_square, Square::new("e2")),
            Ply::new(start_square, Square::new("f1")),
            Ply::new(start_square, Square::new("f2")),
            Ply::builder(start_square, Square::new("g1")).is_castles(true).build(),
            Ply::builder(start_square, Square::new("c1")).is_castles(true).build(),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_king_get_moveset_black_e1() {
        let piece = Kind::King(Color::White);
        let start_square = Square::new("e1");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("d1")),
            Ply::new(start_square, Square::new("d2")),
            Ply::new(start_square, Square::new("e2")),
            Ply::new(start_square, Square::new("f1")),
            Ply::new(start_square, Square::new("f2")),
            Ply::builder(start_square, Square::new("g1")).is_castles(true).build(),
            Ply::builder(start_square, Square::new("c1")).is_castles(true).build(),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_king_get_moveset_white_e8() {
        let piece = Kind::King(Color::White);
        let start_square = Square::new("e8");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("d8")),
            Ply::new(start_square, Square::new("d7")),
            Ply::new(start_square, Square::new("e7")),
            Ply::new(start_square, Square::new("f8")),
            Ply::new(start_square, Square::new("f7")),
            Ply::builder(start_square, Square::new("g8")).is_castles(true).build(),
            Ply::builder(start_square, Square::new("c8")).is_castles(true).build(),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_king_get_moveset_black_e8() {
        let piece = Kind::King(Color::White);
        let start_square = Square::new("e8");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("d8")),
            Ply::new(start_square, Square::new("d7")),
            Ply::new(start_square, Square::new("e7")),
            Ply::new(start_square, Square::new("f8")),
            Ply::new(start_square, Square::new("f7")),
            Ply::builder(start_square, Square::new("g8")).is_castles(true).build(),
            Ply::builder(start_square, Square::new("c8")).is_castles(true).build(),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }
}
