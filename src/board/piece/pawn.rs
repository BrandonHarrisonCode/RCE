use super::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Pawn;

const WHITE_SYMBOL: &str = "♟";
const BLACK_SYMBOL: &str = "♙";

impl Eq for Pawn {}

impl Piece for Pawn {
    fn get_piece_symbol(color: &Color) -> &'static str {
        match color {
            Color::White => WHITE_SYMBOL,
            Color::Black => BLACK_SYMBOL,
        }
    }

    /// [X] Advances 1 square forward
    /// [X] Advances 2 squares forward if on second rank
    /// [ ] Takes diagonally forward
    /// [ ] En passant
    /// [ ] Promotion
    fn get_moveset(square: &Square, color: &Color) -> Vec<Ply> {
        let (direction, starting_rank) = match color {
            Color::White => (Direction::North, 1),
            Color::Black => (Direction::South, 6),
        };
        let mut output: Vec<Ply> = vec![
            Ply::new(*square, *square + direction.unit_square()),
            Ply::builder(
                *square,
                *square + direction.unit_square() + Direction::East.unit_square(),
            )
            .build(),
            Ply::builder(
                *square,
                *square + direction.unit_square() + Direction::West.unit_square(),
            )
            .build(),
        ];

        if square.rank == starting_rank {
            output.push(Ply::new(
                *square,
                *square + direction.unit_square() + direction.unit_square(),
            ));
        }
        output
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_pawn_derived_traits() {
        let piece = Pawn {};
        dbg!(&piece);

        assert_eq!(piece, piece.clone());
    }

    #[test]
    fn test_pawn_display_white() {
        let output = super::WHITE_SYMBOL;
        let correct = "♟";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_pawn_display_black() {
        let output = super::BLACK_SYMBOL;
        let correct = "♙";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_pawn_get_piece_symbol_white() {
        let piece = PieceKind::Pawn(Color::White);
        let correct = "♟";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_pawn_get_piece_symbol_black() {
        let piece = PieceKind::Pawn(Color::Black);
        let correct = "♙";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_pawn_eq() {
        let left = PieceKind::Pawn(Color::White);
        let right = PieceKind::Pawn(Color::White);

        assert_eq!(left, right);
    }

    #[test]
    fn test_pawn_neq() {
        let left = PieceKind::Pawn(Color::White);
        let right = PieceKind::Pawn(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_pawn_neq_rev() {
        // Test if addition is commutative
        let right = PieceKind::Pawn(Color::White);
        let left = PieceKind::Pawn(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_pawn_get_moveset_white_a2() {
        let piece = PieceKind::Pawn(Color::White);
        let start_square = Square::new("a2");

        let result = piece.get_moveset(&start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("a3")),
            Ply::new(start_square, Square::new("b3")),
            Ply::new(start_square, Square::new("a4")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_pawn_get_moveset_white_d2() {
        let piece = PieceKind::Pawn(Color::White);
        let start_square = Square::new("d2");

        let result = piece.get_moveset(&start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("d3")),
            Ply::new(start_square, Square::new("c3")),
            Ply::new(start_square, Square::new("e3")),
            Ply::new(start_square, Square::new("d4")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_pawn_get_moveset_white_h6() {
        let piece = PieceKind::Pawn(Color::White);
        let start_square = Square::new("h6");

        let result = piece.get_moveset(&start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("h7")),
            Ply::new(start_square, Square::new("g7")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_pawn_get_moveset_black_a3() {
        let piece = PieceKind::Pawn(Color::Black);
        let start_square = Square::new("a3");

        let result = piece.get_moveset(&start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("a2")),
            Ply::new(start_square, Square::new("b2")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_pawn_get_moveset_black_d5() {
        let piece = PieceKind::Pawn(Color::Black);
        let start_square = Square::new("d5");

        let result = piece.get_moveset(&start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("d4")),
            Ply::new(start_square, Square::new("c4")),
            Ply::new(start_square, Square::new("e4")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_pawn_get_moveset_black_h7() {
        let piece = PieceKind::Pawn(Color::Black);
        let start_square = Square::new("h7");

        let result = piece.get_moveset(&start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("h6")),
            Ply::new(start_square, Square::new("h5")),
            Ply::new(start_square, Square::new("g6")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }
}
