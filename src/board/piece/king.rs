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
    fn test_king_get_moveset_white_b1() {
        let piece = PieceKind::King(Color::White);
        let start_square = Square::new("b1");

        let result = piece.get_moveset(&start_square);
        let mut correct = Vec::new();
        correct.push(Ply::new(start_square, Square::new("b2")));
        correct.push(Ply::new(start_square, Square::new("a2")));
        correct.push(Ply::new(start_square, Square::new("c2")));
        correct.push(Ply::new(start_square, Square::new("c1")));
        correct.push(Ply::new(start_square, Square::new("a1")));

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_king_get_moveset_white_d4() {
        let piece = PieceKind::King(Color::White);
        let start_square = Square::new("d4");

        let result = piece.get_moveset(&start_square);
        let mut correct = Vec::new();
        correct.push(Ply::new(start_square, Square::new("c3")));
        correct.push(Ply::new(start_square, Square::new("d3")));
        correct.push(Ply::new(start_square, Square::new("e3")));
        correct.push(Ply::new(start_square, Square::new("c4")));
        correct.push(Ply::new(start_square, Square::new("e4")));
        correct.push(Ply::new(start_square, Square::new("c5")));
        correct.push(Ply::new(start_square, Square::new("d5")));
        correct.push(Ply::new(start_square, Square::new("e5")));

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_king_get_moveset_white_h6() {
        let piece = PieceKind::King(Color::White);
        let start_square = Square::new("h6");

        let result = piece.get_moveset(&start_square);
        let mut correct = Vec::new();
        correct.push(Ply::new(start_square, Square::new("g5")));
        correct.push(Ply::new(start_square, Square::new("h5")));
        correct.push(Ply::new(start_square, Square::new("g6")));
        correct.push(Ply::new(start_square, Square::new("g7")));
        correct.push(Ply::new(start_square, Square::new("h7")));

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_king_get_moveset_black_b1() {
        let piece = PieceKind::King(Color::Black);
        let start_square = Square::new("b1");

        let result = piece.get_moveset(&start_square);
        let mut correct = Vec::new();
        correct.push(Ply::new(start_square, Square::new("a1")));
        correct.push(Ply::new(start_square, Square::new("c1")));
        correct.push(Ply::new(start_square, Square::new("a2")));
        correct.push(Ply::new(start_square, Square::new("b2")));
        correct.push(Ply::new(start_square, Square::new("c2")));

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_king_get_moveset_black_d4() {
        let piece = PieceKind::King(Color::Black);
        let start_square = Square::new("d4");

        let result = piece.get_moveset(&start_square);
        let mut correct = Vec::new();
        correct.push(Ply::new(start_square, Square::new("c3")));
        correct.push(Ply::new(start_square, Square::new("d3")));
        correct.push(Ply::new(start_square, Square::new("e3")));
        correct.push(Ply::new(start_square, Square::new("c4")));
        correct.push(Ply::new(start_square, Square::new("e4")));
        correct.push(Ply::new(start_square, Square::new("c5")));
        correct.push(Ply::new(start_square, Square::new("d5")));
        correct.push(Ply::new(start_square, Square::new("e5")));

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }

    #[test]
    fn test_king_get_moveset_black_h6() {
        let piece = PieceKind::King(Color::Black);
        let start_square = Square::new("h6");

        let result = piece.get_moveset(&start_square);
        let mut correct = Vec::new();

        correct.push(Ply::new(start_square, Square::new("g5")));
        correct.push(Ply::new(start_square, Square::new("h5")));
        correct.push(Ply::new(start_square, Square::new("g6")));
        correct.push(Ply::new(start_square, Square::new("g7")));
        correct.push(Ply::new(start_square, Square::new("h7")));

        let result_set: HashSet<Ply> = result.into_iter().collect();
        let correct_set: HashSet<Ply> = correct.into_iter().collect();
        assert_eq!(result_set, correct_set);
    }
}
