use super::*;

#[derive(Clone, PartialEq, Debug)]
pub struct King;

const WHITE_SYMBOL: &str = "♚";
const BLACK_SYMBOL: &str = "♔";

impl Eq for King {}

impl Piece for King {
    fn get_piece_symbol(color: &Color) -> &'static str {
        match color {
            Color::White => WHITE_SYMBOL,
            Color::Black => BLACK_SYMBOL,
        }
    }

    fn get_moveset(square: &Square) -> Vec<Ply> {
        let mut moveset: Vec<Ply> = Vec::new();
        moveset.push(Ply::new(
            square.clone(),
            square.clone() + Direction::North.unit_square(),
        ));

        moveset.push(Ply::new(
            square.clone(),
            square.clone() + Direction::East.unit_square(),
        ));

        moveset.push(Ply::new(
            square.clone(),
            square.clone() + Direction::South.unit_square(),
        ));

        moveset.push(Ply::new(
            square.clone(),
            square.clone() + Direction::West.unit_square(),
        ));

        moveset.push(Ply::new(
            square.clone(),
            square.clone() + Direction::NorthEast.unit_square(),
        ));

        moveset.push(Ply::new(
            square.clone(),
            square.clone() + Direction::NorthWest.unit_square(),
        ));

        moveset.push(Ply::new(
            square.clone(),
            square.clone() + Direction::SouthEast.unit_square(),
        ));

        moveset.push(Ply::new(
            square.clone(),
            square.clone() + Direction::SouthWest.unit_square(),
        ));

        let output = moveset
            .into_iter()
            .filter(|mv| {
                mv.start.rank < 8
                    && mv.start.file < 8
                    && mv.dest.rank < 8
                    && mv.dest.file < 8
                    && mv.start != mv.dest
            })
            .collect();

        output
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_king_derived_traits() {
        let piece = King {};
        dbg!(&piece);

        assert_eq!(piece, piece.clone());
    }

    #[test]
    fn test_king_display_white() {
        let output = super::WHITE_SYMBOL;
        let correct = "♚";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_king_display_black() {
        let output = super::BLACK_SYMBOL;
        let correct = "♔";

        assert_eq!(output, correct);
    }

    #[test]
    fn test_king_get_piece_symbol_white() {
        let piece = PieceKind::King(Color::White);
        let correct = "♚";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_king_get_piece_symbol_black() {
        let piece = PieceKind::King(Color::Black);
        let correct = "♔";

        assert_eq!(piece.get_piece_symbol(), correct);
    }

    #[test]
    fn test_king_eq() {
        let left = PieceKind::King(Color::White);
        let right = PieceKind::King(Color::White);

        assert_eq!(left, right);
    }

    #[test]
    fn test_king_neq() {
        let left = PieceKind::King(Color::White);
        let right = PieceKind::King(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_king_neq_rev() {
        // Test if addition is commutative
        let right = PieceKind::King(Color::White);
        let left = PieceKind::King(Color::Black);

        assert_ne!(left, right);
    }

    #[test]
    fn test_king_get_moveset_white_b0() {
        let piece = PieceKind::King(Color::White);
        let start_square = Square::new(0, 1);

        let result = piece.get_moveset(&start_square);
        let mut correct = Vec::new();
        correct.push(Ply::new(start_square, Square::new(1, 1)));
        correct.push(Ply::new(start_square, Square::new(1, 0)));
        correct.push(Ply::new(start_square, Square::new(1, 2)));
        correct.push(Ply::new(start_square, Square::new(0, 2)));
        correct.push(Ply::new(start_square, Square::new(0, 0)));

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_king_get_moveset_white_d4() {
        let piece = PieceKind::King(Color::White);
        let start_square = Square::new(3, 3);

        let result = piece.get_moveset(&start_square);
        let mut correct = Vec::new();
        correct.push(Ply::new(start_square, Square::new(2, 2))); // Down 2, Left 1
        correct.push(Ply::new(start_square, Square::new(2, 3))); // Down 2, Right 1
        correct.push(Ply::new(start_square, Square::new(2, 4))); // Up 2, Left 1
        correct.push(Ply::new(start_square, Square::new(3, 2))); // Down 2, Left 1
        correct.push(Ply::new(start_square, Square::new(3, 4))); // Up 2, Left 1
        correct.push(Ply::new(start_square, Square::new(4, 2))); // Down 2, Left 1
        correct.push(Ply::new(start_square, Square::new(4, 3))); // Down 2, Right 1
        correct.push(Ply::new(start_square, Square::new(4, 4))); // Up 2, Left 1

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_king_get_moveset_white_h6() {
        let piece = PieceKind::King(Color::White);
        let start_square = Square::new(5, 7);

        let result = piece.get_moveset(&start_square);
        let mut correct = Vec::new();
        correct.push(Ply::new(start_square, Square::new(4, 6))); // Down 2, Left 1
        correct.push(Ply::new(start_square, Square::new(4, 7))); // Down 2, Right 1
        correct.push(Ply::new(start_square, Square::new(5, 6))); // Down 2, Left 1
        correct.push(Ply::new(start_square, Square::new(6, 6))); // Down 2, Left 1
        correct.push(Ply::new(start_square, Square::new(6, 7))); // Down 2, Right 1

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_king_get_moveset_black_b0() {
        let piece = PieceKind::King(Color::Black);
        let start_square = Square::new(0, 1);

        let result = piece.get_moveset(&start_square);
        let mut correct = Vec::new();
        correct.push(Ply::new(start_square, Square::new(0, 2)));
        correct.push(Ply::new(start_square, Square::new(0, 0)));
        correct.push(Ply::new(start_square, Square::new(1, 2)));
        correct.push(Ply::new(start_square, Square::new(1, 1)));
        correct.push(Ply::new(start_square, Square::new(1, 0)));

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_king_get_moveset_black_d4() {
        let piece = PieceKind::King(Color::Black);
        let start_square = Square::new(3, 3);

        let result = piece.get_moveset(&start_square);
        let mut correct = Vec::new();
        correct.push(Ply::new(start_square, Square::new(2, 2))); // Down 2, Left 1
        correct.push(Ply::new(start_square, Square::new(2, 3))); // Down 2, Left 1
        correct.push(Ply::new(start_square, Square::new(2, 4))); // Down 2, Left 1
        correct.push(Ply::new(start_square, Square::new(3, 2))); // Down 2, Left 1
        correct.push(Ply::new(start_square, Square::new(3, 4))); // Down 2, Left 1
        correct.push(Ply::new(start_square, Square::new(4, 2))); // Down 2, Left 1
        correct.push(Ply::new(start_square, Square::new(4, 3))); // Down 2, Left 1
        correct.push(Ply::new(start_square, Square::new(4, 4))); // Down 2, Left 1

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_king_get_moveset_black_h6() {
        let piece = PieceKind::King(Color::Black);
        let start_square = Square::new(5, 7);

        let result = piece.get_moveset(&start_square);
        let mut correct = Vec::new();
        correct.push(Ply::new(start_square, Square::new(4, 6))); // Down 2, Left 1
        correct.push(Ply::new(start_square, Square::new(4, 7))); // Down 2, Left 1
        correct.push(Ply::new(start_square, Square::new(5, 6))); // Down 2, Left 1
        correct.push(Ply::new(start_square, Square::new(6, 6))); // Down 2, Left 1
        correct.push(Ply::new(start_square, Square::new(6, 7))); // Down 2, Left 1

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }
}
