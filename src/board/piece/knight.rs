use super::{Color, Direction, Piece, Ply, Square};

#[derive(Clone, PartialEq, Debug)]
pub struct Knight;

impl Eq for Knight {}

impl Piece for Knight {
    const WHITE_SYMBOL: &'static str = "♞";
    const BLACK_SYMBOL: &'static str = "♘";

    fn get_moveset(square: Square, _: Color) -> Vec<Ply> {
        vec![
            Ply::new(
                square,
                square + Direction::North + Direction::North + Direction::West,
            ),
            Ply::new(
                square,
                square + Direction::North + Direction::North + Direction::East,
            ),
            Ply::new(
                square,
                square + Direction::South + Direction::South + Direction::West,
            ),
            Ply::new(
                square,
                square + Direction::South + Direction::South + Direction::East,
            ),
            Ply::new(
                square,
                square + Direction::East + Direction::East + Direction::North,
            ),
            Ply::new(
                square,
                square + Direction::East + Direction::East + Direction::South,
            ),
            Ply::new(
                square,
                square + Direction::West + Direction::West + Direction::North,
            ),
            Ply::new(
                square,
                square + Direction::West + Direction::West + Direction::South,
            ),
        ]
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{Color, Knight, Piece, Ply, Square};
    use crate::board::Kind;
    use std::collections::HashSet;

    #[test]
    fn test_knight_derived_traits() {
        let piece = Knight {};
        dbg!(&piece);

        assert_eq!(piece, piece.clone());
    }

    #[test]
    fn test_knight_display_white() {
        let output = Knight::WHITE_SYMBOL;
        let correct = "♞";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_knight_display_black() {
        let output = Knight::BLACK_SYMBOL;
        let correct = "♘";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_knight_get_piece_symbol_white() {
        let piece = Kind::Knight(Color::White);
        let correct = "♞";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_knight_get_piece_symbol_black() {
        let piece = Kind::Knight(Color::Black);
        let correct = "♘";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_knight_eq() {
        let left = Kind::Knight(Color::White);
        let right = Kind::Knight(Color::White);

        assert_eq!(left, right);
    }

    #[test]
    fn test_knight_neq() {
        let left = Kind::Knight(Color::White);
        let right = Kind::Knight(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_knight_neq_rev() {
        // Test if addition is commutative
        let right = Kind::Knight(Color::White);
        let left = Kind::Knight(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_knight_get_moveset_white_b1() {
        let piece = Kind::Knight(Color::White);
        let start_square = Square::new("b1");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("a3")),
            Ply::new(start_square, Square::new("c3")),
            Ply::new(start_square, Square::new("d2")),
        ];

        assert_eq!(result, correct);
    }

    #[test]
    fn test_knight_get_moveset_white_d4() {
        let piece = Kind::Knight(Color::White);
        let start_square = Square::new("d4");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("c2")), // Down 2, Left 1
            Ply::new(start_square, Square::new("e2")), // Down 2, Right 1
            Ply::new(start_square, Square::new("c6")), // Up 2, Left 1
            Ply::new(start_square, Square::new("e6")), // Up 2, Right 1
            Ply::new(start_square, Square::new("b5")), // Left 2, Up 1
            Ply::new(start_square, Square::new("b3")), // Left 2, Down 1
            Ply::new(start_square, Square::new("f5")), // Right 2, Up 1
            Ply::new(start_square, Square::new("f3")), // Right 2, Down 1
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_knight_get_moveset_white_h6() {
        let piece = Kind::Knight(Color::White);
        let start_square = Square::new("h6");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("g4")), // Down 2, Left 1
            Ply::new(start_square, Square::new("g8")), // Up 2, Left 1
            Ply::new(start_square, Square::new("f7")), // Left 2, Up 1
            Ply::new(start_square, Square::new("f5")), // Left 2, Down 1
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_knight_get_moveset_black_b1() {
        let piece = Kind::Knight(Color::Black);
        let start_square = Square::new("b1");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("a3")),
            Ply::new(start_square, Square::new("c3")),
            Ply::new(start_square, Square::new("d2")),
        ];

        assert_eq!(result, correct);
    }

    #[test]
    fn test_knight_get_moveset_black_d4() {
        let piece = Kind::Knight(Color::Black);
        let start_square = Square::new("d4");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("c2")), // Down 2, Left 1
            Ply::new(start_square, Square::new("e2")), // Down 2, Right 1
            Ply::new(start_square, Square::new("c6")), // Up 2, Left 1
            Ply::new(start_square, Square::new("e6")), // Up 2, Right 1
            Ply::new(start_square, Square::new("b5")), // Left 2, Up 1
            Ply::new(start_square, Square::new("b3")), // Left 2, Down 1
            Ply::new(start_square, Square::new("f5")), // Right 2, Up 1
            Ply::new(start_square, Square::new("f3")), // Right 2, Down 1
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_knight_get_moveset_black_h6() {
        let piece = Kind::Knight(Color::Black);
        let start_square = Square::new("h6");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("g4")), // Down 2, Left 1
            Ply::new(start_square, Square::new("g8")), // Up 2, Left 1
            Ply::new(start_square, Square::new("f7")), // Left 2, Up 1
            Ply::new(start_square, Square::new("f5")), // Left 2, Down 1
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }
}
