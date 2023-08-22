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
        let mut moveset: Vec<Ply> = Vec::new();
        moveset.push(Ply::new(
            square.clone(),
            square.clone()
                + Direction::North.unit_square()
                + Direction::North.unit_square()
                + Direction::West.unit_square(),
        ));

        moveset.push(Ply::new(
            square.clone(),
            square.clone()
                + Direction::North.unit_square()
                + Direction::North.unit_square()
                + Direction::East.unit_square(),
        ));

        moveset.push(Ply::new(
            square.clone(),
            square.clone()
                + Direction::South.unit_square()
                + Direction::South.unit_square()
                + Direction::West.unit_square(),
        ));

        moveset.push(Ply::new(
            square.clone(),
            square.clone()
                + Direction::South.unit_square()
                + Direction::South.unit_square()
                + Direction::East.unit_square(),
        ));

        moveset.push(Ply::new(
            square.clone(),
            square.clone()
                + Direction::East.unit_square()
                + Direction::East.unit_square()
                + Direction::North.unit_square(),
        ));

        moveset.push(Ply::new(
            square.clone(),
            square.clone()
                + Direction::East.unit_square()
                + Direction::East.unit_square()
                + Direction::South.unit_square(),
        ));

        moveset.push(Ply::new(
            square.clone(),
            square.clone()
                + Direction::West.unit_square()
                + Direction::West.unit_square()
                + Direction::North.unit_square(),
        ));

        moveset.push(Ply::new(
            square.clone(),
            square.clone()
                + Direction::West.unit_square()
                + Direction::West.unit_square()
                + Direction::South.unit_square(),
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
    fn test_knight_get_moveset_white_b0() {
        let piece = PieceKind::Knight(Color::White);
        let start_square = Square::new(0, 1);

        let result = piece.get_moveset(&start_square);
        let mut correct = Vec::new();
        correct.push(Ply::new(start_square, Square::new(2, 0)));
        correct.push(Ply::new(start_square, Square::new(2, 2)));
        correct.push(Ply::new(start_square, Square::new(1, 3)));

        assert_eq!(result, correct);
    }

    #[test]
    fn test_knight_get_moveset_white_d4() {
        let piece = PieceKind::Knight(Color::White);
        let start_square = Square::new(3, 3);

        let result = piece.get_moveset(&start_square);
        let mut correct = Vec::new();
        correct.push(Ply::new(start_square, Square::new(1, 2))); // Down 2, Left 1
        correct.push(Ply::new(start_square, Square::new(1, 4))); // Down 2, Right 1
        correct.push(Ply::new(start_square, Square::new(5, 2))); // Up 2, Left 1
        correct.push(Ply::new(start_square, Square::new(5, 4))); // Up 2, Right 1
        correct.push(Ply::new(start_square, Square::new(4, 1))); // Left 2, Up 1
        correct.push(Ply::new(start_square, Square::new(2, 1))); // Left 2, Down 1
        correct.push(Ply::new(start_square, Square::new(4, 5))); // Right 2, Up 1
        correct.push(Ply::new(start_square, Square::new(2, 5))); // Right 2, Down 1

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_knight_get_moveset_white_h6() {
        let piece = PieceKind::Knight(Color::White);
        let start_square = Square::new(5, 7);

        let result = piece.get_moveset(&start_square);
        let mut correct = Vec::new();
        correct.push(Ply::new(start_square, Square::new(3, 6))); // Down 2, Left 1
        correct.push(Ply::new(start_square, Square::new(7, 6))); // Up 2, Left 1
        correct.push(Ply::new(start_square, Square::new(6, 5))); // Left 2, Up 1
        correct.push(Ply::new(start_square, Square::new(4, 5))); // Left 2, Down 1

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_knight_get_moveset_black_b0() {
        let piece = PieceKind::Knight(Color::Black);
        let start_square = Square::new(0, 1);

        let result = piece.get_moveset(&start_square);
        let mut correct = Vec::new();
        correct.push(Ply::new(start_square, Square::new(2, 0)));
        correct.push(Ply::new(start_square, Square::new(2, 2)));
        correct.push(Ply::new(start_square, Square::new(1, 3)));

        assert_eq!(result, correct);
    }

    #[test]
    fn test_knight_get_moveset_black_d4() {
        let piece = PieceKind::Knight(Color::Black);
        let start_square = Square::new(3, 3);

        let result = piece.get_moveset(&start_square);
        let mut correct = Vec::new();
        correct.push(Ply::new(start_square, Square::new(1, 2))); // Down 2, Left 1
        correct.push(Ply::new(start_square, Square::new(1, 4))); // Down 2, Right 1
        correct.push(Ply::new(start_square, Square::new(5, 2))); // Up 2, Left 1
        correct.push(Ply::new(start_square, Square::new(5, 4))); // Up 2, Right 1
        correct.push(Ply::new(start_square, Square::new(4, 1))); // Left 2, Up 1
        correct.push(Ply::new(start_square, Square::new(2, 1))); // Left 2, Down 1
        correct.push(Ply::new(start_square, Square::new(4, 5))); // Right 2, Up 1
        correct.push(Ply::new(start_square, Square::new(2, 5))); // Right 2, Down 1

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_knight_get_moveset_black_h6() {
        let piece = PieceKind::Knight(Color::Black);
        let start_square = Square::new(5, 7);

        let result = piece.get_moveset(&start_square);
        let mut correct = Vec::new();
        correct.push(Ply::new(start_square, Square::new(3, 6))); // Down 2, Left 1
        correct.push(Ply::new(start_square, Square::new(7, 6))); // Up 2, Left 1
        correct.push(Ply::new(start_square, Square::new(6, 5))); // Left 2, Up 1
        correct.push(Ply::new(start_square, Square::new(4, 5))); // Left 2, Down 1

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }
}
