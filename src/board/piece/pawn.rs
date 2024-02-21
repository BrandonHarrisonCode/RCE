use super::{Color, Direction, Kind, Piece, Ply, Square};

#[derive(Clone, PartialEq, Debug)]
pub struct Pawn;

impl Eq for Pawn {}

impl Pawn {
    fn explode_promotion(ply: Ply, color: Color, back_rank: u8) -> Vec<Ply> {
        if ply.dest.rank == back_rank {
            vec![
                Ply::builder(ply.start, ply.dest)
                    .promoted_to(Kind::Queen(color))
                    .build(),
                Ply::builder(ply.start, ply.dest)
                    .promoted_to(Kind::Rook(color))
                    .build(),
                Ply::builder(ply.start, ply.dest)
                    .promoted_to(Kind::Knight(color))
                    .build(),
                Ply::builder(ply.start, ply.dest)
                    .promoted_to(Kind::Bishop(color))
                    .build(),
            ]
        } else {
            vec![ply]
        }
    }
}

impl Piece for Pawn {
    const WHITE_SYMBOL: &'static str = "♟";
    const BLACK_SYMBOL: &'static str = "♙";

    /// - [X] Advances 1 square forward
    /// - [X] Advances 2 squares forward if on second rank
    /// - [X] Takes diagonally forward
    /// - [X] En passant
    /// - [X] Promotion
    fn get_moveset(square: Square, color: Color) -> Vec<Ply> {
        let (direction, starting_rank, en_passant_rank, back_rank) = match color {
            Color::White => (Direction::North, 1, 4, 7),
            Color::Black => (Direction::South, 6, 3, 0),
        };

        // Directional captures
        let mut output: Vec<Ply> = vec![
            Ply::new(square, square + direction),
            Ply::builder(square, square + direction + Direction::East).build(),
            Ply::builder(square, square + direction + Direction::West).build(),
        ];

        // Double pawn push
        if square.rank == starting_rank {
            output.push(
                Ply::builder(square, square + direction + direction)
                    .double_pawn_push(true)
                    .build(),
            );
        }

        // En Passant
        if square.rank == en_passant_rank {
            output.push(
                Ply::builder(square, square + direction + Direction::East)
                    .en_passant(true)
                    .captured(Kind::Pawn(color.opposite()))
                    .build(),
            );
            output.push(
                Ply::builder(square, square + direction + Direction::West)
                    .en_passant(true)
                    .captured(Kind::Pawn(color.opposite()))
                    .build(),
            );
        }

        // Promotion
        output
            .iter()
            .flat_map(|ply| Self::explode_promotion(*ply, color, back_rank))
            .collect()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{Color, Pawn, Piece, Ply, Square};
    use crate::board::Kind;
    use std::collections::HashSet;

    #[test]
    fn test_pawn_derived_traits() {
        let piece = Pawn {};
        dbg!(&piece);

        assert_eq!(piece, piece.clone());
    }

    #[test]
    fn test_pawn_display_white() {
        let output = Pawn::WHITE_SYMBOL;
        let correct = "♟";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_pawn_display_black() {
        let output = Pawn::BLACK_SYMBOL;
        let correct = "♙";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_pawn_get_piece_symbol_white() {
        let piece = Kind::Pawn(Color::White);
        let correct = "♟";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_pawn_get_piece_symbol_black() {
        let piece = Kind::Pawn(Color::Black);
        let correct = "♙";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_pawn_eq() {
        let left = Kind::Pawn(Color::White);
        let right = Kind::Pawn(Color::White);

        assert_eq!(left, right);
    }

    #[test]
    fn test_pawn_neq() {
        let left = Kind::Pawn(Color::White);
        let right = Kind::Pawn(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_pawn_neq_rev() {
        // Test if addition is commutative
        let right = Kind::Pawn(Color::White);
        let left = Kind::Pawn(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_pawn_get_moveset_white_a2() {
        let piece = Kind::Pawn(Color::White);
        let start_square = Square::new("a2");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("a3")),
            Ply::new(start_square, Square::new("b3")),
            Ply::builder(start_square, Square::new("a4"))
                .double_pawn_push(true)
                .build(),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_pawn_get_moveset_white_d2() {
        let piece = Kind::Pawn(Color::White);
        let start_square = Square::new("d2");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("d3")),
            Ply::new(start_square, Square::new("c3")),
            Ply::new(start_square, Square::new("e3")),
            Ply::builder(start_square, Square::new("d4"))
                .double_pawn_push(true)
                .build(),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_pawn_get_moveset_white_h6() {
        let piece = Kind::Pawn(Color::White);
        let start_square = Square::new("h6");

        let result = piece.get_moveset(start_square);
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
        let piece = Kind::Pawn(Color::Black);
        let start_square = Square::new("a3");

        let result = piece.get_moveset(start_square);
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
        let piece = Kind::Pawn(Color::Black);
        let start_square = Square::new("d5");

        let result = piece.get_moveset(start_square);
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
        let piece = Kind::Pawn(Color::Black);
        let start_square = Square::new("h7");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::new(start_square, Square::new("h6")),
            Ply::builder(start_square, Square::new("h5"))
                .double_pawn_push(true)
                .build(),
            Ply::new(start_square, Square::new("g6")),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_pawn_get_moveset_white_h7() {
        let piece = Kind::Pawn(Color::White);
        let start_square = Square::new("h7");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::builder(start_square, Square::new("h8"))
                .promoted_to(Kind::Queen(Color::White))
                .build(),
            Ply::builder(start_square, Square::new("h8"))
                .promoted_to(Kind::Rook(Color::White))
                .build(),
            Ply::builder(start_square, Square::new("h8"))
                .promoted_to(Kind::Knight(Color::White))
                .build(),
            Ply::builder(start_square, Square::new("h8"))
                .promoted_to(Kind::Bishop(Color::White))
                .build(),
            Ply::builder(start_square, Square::new("g8"))
                .promoted_to(Kind::Queen(Color::White))
                .build(),
            Ply::builder(start_square, Square::new("g8"))
                .promoted_to(Kind::Rook(Color::White))
                .build(),
            Ply::builder(start_square, Square::new("g8"))
                .promoted_to(Kind::Knight(Color::White))
                .build(),
            Ply::builder(start_square, Square::new("g8"))
                .promoted_to(Kind::Bishop(Color::White))
                .build(),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_pawn_get_moveset_black_h2() {
        let piece = Kind::Pawn(Color::Black);
        let start_square = Square::new("h2");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::builder(start_square, Square::new("h1"))
                .promoted_to(Kind::Queen(Color::Black))
                .build(),
            Ply::builder(start_square, Square::new("h1"))
                .promoted_to(Kind::Rook(Color::Black))
                .build(),
            Ply::builder(start_square, Square::new("h1"))
                .promoted_to(Kind::Knight(Color::Black))
                .build(),
            Ply::builder(start_square, Square::new("h1"))
                .promoted_to(Kind::Bishop(Color::Black))
                .build(),
            Ply::builder(start_square, Square::new("g1"))
                .promoted_to(Kind::Queen(Color::Black))
                .build(),
            Ply::builder(start_square, Square::new("g1"))
                .promoted_to(Kind::Rook(Color::Black))
                .build(),
            Ply::builder(start_square, Square::new("g1"))
                .promoted_to(Kind::Knight(Color::Black))
                .build(),
            Ply::builder(start_square, Square::new("g1"))
                .promoted_to(Kind::Bishop(Color::Black))
                .build(),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_pawn_get_moveset_white_d7() {
        let piece = Kind::Pawn(Color::White);
        let start_square = Square::new("d7");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::builder(start_square, Square::new("d8"))
                .promoted_to(Kind::Queen(Color::White))
                .build(),
            Ply::builder(start_square, Square::new("d8"))
                .promoted_to(Kind::Rook(Color::White))
                .build(),
            Ply::builder(start_square, Square::new("d8"))
                .promoted_to(Kind::Knight(Color::White))
                .build(),
            Ply::builder(start_square, Square::new("d8"))
                .promoted_to(Kind::Bishop(Color::White))
                .build(),
            Ply::builder(start_square, Square::new("e8"))
                .promoted_to(Kind::Queen(Color::White))
                .build(),
            Ply::builder(start_square, Square::new("e8"))
                .promoted_to(Kind::Rook(Color::White))
                .build(),
            Ply::builder(start_square, Square::new("e8"))
                .promoted_to(Kind::Knight(Color::White))
                .build(),
            Ply::builder(start_square, Square::new("e8"))
                .promoted_to(Kind::Bishop(Color::White))
                .build(),
            Ply::builder(start_square, Square::new("c8"))
                .promoted_to(Kind::Queen(Color::White))
                .build(),
            Ply::builder(start_square, Square::new("c8"))
                .promoted_to(Kind::Rook(Color::White))
                .build(),
            Ply::builder(start_square, Square::new("c8"))
                .promoted_to(Kind::Knight(Color::White))
                .build(),
            Ply::builder(start_square, Square::new("c8"))
                .promoted_to(Kind::Bishop(Color::White))
                .build(),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_pawn_get_moveset_black_d2() {
        let piece = Kind::Pawn(Color::Black);
        let start_square = Square::new("d2");

        let result = piece.get_moveset(start_square);
        let correct = vec![
            Ply::builder(start_square, Square::new("d1"))
                .promoted_to(Kind::Queen(Color::Black))
                .build(),
            Ply::builder(start_square, Square::new("d1"))
                .promoted_to(Kind::Rook(Color::Black))
                .build(),
            Ply::builder(start_square, Square::new("d1"))
                .promoted_to(Kind::Knight(Color::Black))
                .build(),
            Ply::builder(start_square, Square::new("d1"))
                .promoted_to(Kind::Bishop(Color::Black))
                .build(),
            Ply::builder(start_square, Square::new("e1"))
                .promoted_to(Kind::Queen(Color::Black))
                .build(),
            Ply::builder(start_square, Square::new("e1"))
                .promoted_to(Kind::Rook(Color::Black))
                .build(),
            Ply::builder(start_square, Square::new("e1"))
                .promoted_to(Kind::Knight(Color::Black))
                .build(),
            Ply::builder(start_square, Square::new("e1"))
                .promoted_to(Kind::Bishop(Color::Black))
                .build(),
            Ply::builder(start_square, Square::new("c1"))
                .promoted_to(Kind::Queen(Color::Black))
                .build(),
            Ply::builder(start_square, Square::new("c1"))
                .promoted_to(Kind::Rook(Color::Black))
                .build(),
            Ply::builder(start_square, Square::new("c1"))
                .promoted_to(Kind::Knight(Color::Black))
                .build(),
            Ply::builder(start_square, Square::new("c1"))
                .promoted_to(Kind::Bishop(Color::Black))
                .build(),
        ];

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }
}
