use super::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Knight;

const WHITE_SYMBOL: &str = "♞";
const BLACK_SYMBOL: &str = "♘";

impl Eq for Knight {}

impl Piece for Knight {
    fn get_piece_symbol(color: &Color) -> &'static str {
        match color {
            Color::White => WHITE_SYMBOL,
            Color::Black => BLACK_SYMBOL,
        }
    }

    fn get_moveset(square: &Square) -> Vec<Ply> {
        vec![
            Ply::new(
                *square,
                *square
                    + Direction::North.unit_square()
                    + Direction::North.unit_square()
                    + Direction::West.unit_square(),
            ),
            Ply::new(
                *square,
                *square
                    + Direction::North.unit_square()
                    + Direction::North.unit_square()
                    + Direction::East.unit_square(),
            ),
            Ply::new(
                *square,
                *square
                    + Direction::South.unit_square()
                    + Direction::South.unit_square()
                    + Direction::West.unit_square(),
            ),
            Ply::new(
                *square,
                *square
                    + Direction::South.unit_square()
                    + Direction::South.unit_square()
                    + Direction::East.unit_square(),
            ),
            Ply::new(
                *square,
                *square
                    + Direction::East.unit_square()
                    + Direction::East.unit_square()
                    + Direction::North.unit_square(),
            ),
            Ply::new(
                *square,
                *square
                    + Direction::East.unit_square()
                    + Direction::East.unit_square()
                    + Direction::South.unit_square(),
            ),
            Ply::new(
                *square,
                *square
                    + Direction::West.unit_square()
                    + Direction::West.unit_square()
                    + Direction::North.unit_square(),
            ),
            Ply::new(
                *square,
                *square
                    + Direction::West.unit_square()
                    + Direction::West.unit_square()
                    + Direction::South.unit_square(),
            ),
        ]
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_knight_derived_traits() {
        let piece = Knight {};
        dbg!(&piece);

        assert_eq!(piece, piece.clone());
    }

    #[test]
    fn test_knight_display_white() {
        let output = super::WHITE_SYMBOL;
        let correct = "♞";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_knight_display_black() {
        let output = super::BLACK_SYMBOL;
        let correct = "♘";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_knight_get_piece_symbol_white() {
        let piece = PieceKind::Knight(Color::White);
        let correct = "♞";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_knight_get_piece_symbol_black() {
        let piece = PieceKind::Knight(Color::Black);
        let correct = "♘";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_knight_eq() {
        let left = PieceKind::Knight(Color::White);
        let right = PieceKind::Knight(Color::White);

        assert_eq!(left, right);
    }

    #[test]
    fn test_knight_neq() {
        let left = PieceKind::Knight(Color::White);
        let right = PieceKind::Knight(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_knight_neq_rev() {
        // Test if addition is commutative
        let right = PieceKind::Knight(Color::White);
        let left = PieceKind::Knight(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_knight_get_moveset_white_b1() {
        let piece = PieceKind::Knight(Color::White);
        let start_square = Square::new("b1");

        let result = piece.get_moveset(&start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("a3")),
            Ply::new(start_square, Square::new("c3")),
            Ply::new(start_square, Square::new("d2")),
        ];

        assert_eq!(result, correct);
    }

    #[test]
    fn test_knight_get_moveset_white_d4() {
        let piece = PieceKind::Knight(Color::White);
        let start_square = Square::new("d4");

        let result = piece.get_moveset(&start_square);
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
        let piece = PieceKind::Knight(Color::White);
        let start_square = Square::new("h6");

        let result = piece.get_moveset(&start_square);
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
        let piece = PieceKind::Knight(Color::Black);
        let start_square = Square::new("b1");

        let result = piece.get_moveset(&start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("a3")),
            Ply::new(start_square, Square::new("c3")),
            Ply::new(start_square, Square::new("d2")),
        ];

        assert_eq!(result, correct);
    }

    #[test]
    fn test_knight_get_moveset_black_d4() {
        let piece = PieceKind::Knight(Color::Black);
        let start_square = Square::new("d4");

        let result = piece.get_moveset(&start_square);
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
        let piece = PieceKind::Knight(Color::Black);
        let start_square = Square::new("h6");

        let result = piece.get_moveset(&start_square);
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
