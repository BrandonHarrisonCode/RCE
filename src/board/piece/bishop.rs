use super::super::bitboard::Bitboard;
use super::{Color, Piece, Ply, Square};
use crate::board::square::rays::RAYS;
use crate::board::square::Direction;

#[derive(Clone, PartialEq, Debug)]
pub struct Bishop {
    masks: [Bitboard; 64],
}

impl Eq for Bishop {}

impl Piece for Bishop {
    const WHITE_SYMBOL: &'static str = "♝";
    const BLACK_SYMBOL: &'static str = "♗";

    fn get_moveset(square: Square, _: Color) -> Vec<Ply> {
        Self::init_bishop_masks();
        let move_mask = square.get_diagonals_mask();
        let squares = Square::get_squares_from_mask(move_mask);

        squares.into_iter().map(|s| Ply::new(square, s)).collect()
    }
}

impl Bishop {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            masks: Self::init_bishop_masks(),
        }
    }

    fn init_bishop_masks() -> [Bitboard; 64] {
        let mut masks: [Bitboard; 64] = [Bitboard::new(0); 64];
        let rays = RAYS
            .get_or_init(|| crate::board::square::rays::Rays::new())
            .rays;

        for i in 0..64u8 {
            let mask: Bitboard = rays[i as usize][Direction::NorthEast as usize]
                | rays[i as usize][Direction::SouthEast as usize]
                | rays[i as usize][Direction::SouthWest as usize]
                | rays[i as usize][Direction::NorthWest as usize];
            let trimmed = mask.trim_edges();

            masks[i as usize] = trimmed;
        }

        dbg!(masks);

        masks
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{Bishop, Color, Piece, Ply, Square};
    use crate::board::Kind;
    use pretty_assertions::{assert_eq, assert_ne};
    use std::collections::HashSet;

    #[test]
    fn test_bishop_derived_traits() {
        let piece = Bishop::new();
        dbg!(&piece);

        assert_eq!(piece, piece.clone());
    }

    #[test]
    fn test_bishop_display_white() {
        let output = Bishop::WHITE_SYMBOL;
        let correct = "♝";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_bishop_display_black() {
        let output = Bishop::BLACK_SYMBOL;
        let correct = "♗";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_bishop_get_piece_symbol_white() {
        let piece = Kind::Bishop(Color::White);
        let correct = "♝";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_bishop_get_piece_symbol_black() {
        let piece = Kind::Bishop(Color::Black);
        let correct = "♗";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_bishop_eq() {
        let left = Kind::Bishop(Color::White);
        let right = Kind::Bishop(Color::White);

        assert_eq!(left, right);
    }

    #[test]
    fn test_bishop_neq() {
        let left = Kind::Bishop(Color::White);
        let right = Kind::Bishop(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_bishop_neq_rev() {
        // Test if addition is commutative
        let right = Kind::Bishop(Color::White);
        let left = Kind::Bishop(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_bishop_get_moveset_white_a1() {
        let piece = Kind::Bishop(Color::White);
        let start_square = Square::from("a1");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::from("b2")),
            Ply::new(start_square, Square::from("c3")),
            Ply::new(start_square, Square::from("d4")),
            Ply::new(start_square, Square::from("e5")),
            Ply::new(start_square, Square::from("f6")),
            Ply::new(start_square, Square::from("g7")),
            Ply::new(start_square, Square::from("h8")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_bishop_get_moveset_white_b1() {
        let piece = Kind::Bishop(Color::White);
        let start_square = Square::from("b1");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::from("c2")),
            Ply::new(start_square, Square::from("d3")),
            Ply::new(start_square, Square::from("e4")),
            Ply::new(start_square, Square::from("f5")),
            Ply::new(start_square, Square::from("g6")),
            Ply::new(start_square, Square::from("h7")),
            Ply::new(start_square, Square::from("a2")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_bishop_get_moveset_white_e4() {
        let piece = Kind::Bishop(Color::White);
        let start_square = Square::from("e4");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::from("f5")),
            Ply::new(start_square, Square::from("g6")),
            Ply::new(start_square, Square::from("h7")),
            Ply::new(start_square, Square::from("d5")),
            Ply::new(start_square, Square::from("c6")),
            Ply::new(start_square, Square::from("b7")),
            Ply::new(start_square, Square::from("a8")),
            Ply::new(start_square, Square::from("b1")),
            Ply::new(start_square, Square::from("c2")),
            Ply::new(start_square, Square::from("d3")),
            Ply::new(start_square, Square::from("f3")),
            Ply::new(start_square, Square::from("g2")),
            Ply::new(start_square, Square::from("h1")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_bishop_get_moveset_white_d4() {
        let piece = Kind::Bishop(Color::White);
        let start_square = Square::from("d4");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::from("e5")),
            Ply::new(start_square, Square::from("f6")),
            Ply::new(start_square, Square::from("g7")),
            Ply::new(start_square, Square::from("h8")),
            Ply::new(start_square, Square::from("c5")),
            Ply::new(start_square, Square::from("b6")),
            Ply::new(start_square, Square::from("a7")),
            Ply::new(start_square, Square::from("a1")),
            Ply::new(start_square, Square::from("b2")),
            Ply::new(start_square, Square::from("c3")),
            Ply::new(start_square, Square::from("e3")),
            Ply::new(start_square, Square::from("f2")),
            Ply::new(start_square, Square::from("g1")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_bishop_get_moveset_white_g6() {
        let piece = Kind::Bishop(Color::White);
        let start_square = Square::from("g6");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::from("h7")),
            Ply::new(start_square, Square::from("h5")),
            Ply::new(start_square, Square::from("f7")),
            Ply::new(start_square, Square::from("e8")),
            Ply::new(start_square, Square::from("f5")),
            Ply::new(start_square, Square::from("e4")),
            Ply::new(start_square, Square::from("d3")),
            Ply::new(start_square, Square::from("c2")),
            Ply::new(start_square, Square::from("b1")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_bishop_get_moveset_white_h6() {
        let piece = Kind::Bishop(Color::White);
        let start_square = Square::from("h6");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::from("g7")),
            Ply::new(start_square, Square::from("f8")),
            Ply::new(start_square, Square::from("g5")),
            Ply::new(start_square, Square::from("f4")),
            Ply::new(start_square, Square::from("e3")),
            Ply::new(start_square, Square::from("d2")),
            Ply::new(start_square, Square::from("c1")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_bishop_get_moveset_black_a1() {
        let piece = Kind::Bishop(Color::Black);
        let start_square = Square::from("a1");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::from("b2")),
            Ply::new(start_square, Square::from("c3")),
            Ply::new(start_square, Square::from("d4")),
            Ply::new(start_square, Square::from("e5")),
            Ply::new(start_square, Square::from("f6")),
            Ply::new(start_square, Square::from("g7")),
            Ply::new(start_square, Square::from("h8")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_bishop_get_moveset_black_b1() {
        let piece = Kind::Bishop(Color::Black);
        let start_square = Square::from("b1");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::from("c2")),
            Ply::new(start_square, Square::from("d3")),
            Ply::new(start_square, Square::from("e4")),
            Ply::new(start_square, Square::from("f5")),
            Ply::new(start_square, Square::from("g6")),
            Ply::new(start_square, Square::from("h7")),
            Ply::new(start_square, Square::from("a2")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_bishop_get_moveset_black_e4() {
        let piece = Kind::Bishop(Color::Black);
        let start_square = Square::from("e4");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::from("f5")),
            Ply::new(start_square, Square::from("g6")),
            Ply::new(start_square, Square::from("h7")),
            Ply::new(start_square, Square::from("d5")),
            Ply::new(start_square, Square::from("c6")),
            Ply::new(start_square, Square::from("b7")),
            Ply::new(start_square, Square::from("a8")),
            Ply::new(start_square, Square::from("b1")),
            Ply::new(start_square, Square::from("c2")),
            Ply::new(start_square, Square::from("d3")),
            Ply::new(start_square, Square::from("f3")),
            Ply::new(start_square, Square::from("g2")),
            Ply::new(start_square, Square::from("h1")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_bishop_get_moveset_black_d4() {
        let piece = Kind::Bishop(Color::Black);
        let start_square = Square::from("d4");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::from("e5")),
            Ply::new(start_square, Square::from("f6")),
            Ply::new(start_square, Square::from("g7")),
            Ply::new(start_square, Square::from("h8")),
            Ply::new(start_square, Square::from("c5")),
            Ply::new(start_square, Square::from("b6")),
            Ply::new(start_square, Square::from("a7")),
            Ply::new(start_square, Square::from("a1")),
            Ply::new(start_square, Square::from("b2")),
            Ply::new(start_square, Square::from("c3")),
            Ply::new(start_square, Square::from("e3")),
            Ply::new(start_square, Square::from("f2")),
            Ply::new(start_square, Square::from("g1")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_bishop_get_moveset_black_g6() {
        let piece = Kind::Bishop(Color::Black);
        let start_square = Square::from("g6");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::from("h7")),
            Ply::new(start_square, Square::from("h5")),
            Ply::new(start_square, Square::from("f7")),
            Ply::new(start_square, Square::from("e8")),
            Ply::new(start_square, Square::from("f5")),
            Ply::new(start_square, Square::from("e4")),
            Ply::new(start_square, Square::from("d3")),
            Ply::new(start_square, Square::from("c2")),
            Ply::new(start_square, Square::from("b1")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_bishop_get_moveset_black_h6() {
        let piece = Kind::Bishop(Color::Black);
        let start_square = Square::from("h6");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::from("g7")),
            Ply::new(start_square, Square::from("f8")),
            Ply::new(start_square, Square::from("g5")),
            Ply::new(start_square, Square::from("f4")),
            Ply::new(start_square, Square::from("e3")),
            Ply::new(start_square, Square::from("d2")),
            Ply::new(start_square, Square::from("c1")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }
}
