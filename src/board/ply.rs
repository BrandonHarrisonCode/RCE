use std::fmt;

use super::{piece::Kind, square::Square, CastlingStatus};

mod builder;
pub mod castling;

use builder::Builder;
use castling::CastlingRights;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ply {
    pub start: Square,
    pub dest: Square,
    pub captured_piece: Option<Kind>,
    pub promoted_to: Option<Kind>,

    pub is_castles: bool,
    pub en_passant: bool,
    pub is_double_pawn_push: bool,

    pub halfmove_clock: u16,
    pub castling_rights: CastlingRights,
}

impl Ord for Ply {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start
            .cmp(&other.start)
            .then(self.dest.cmp(&other.dest))
    }
}

impl PartialOrd for Ply {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Default for Ply {
    fn default() -> Self {
        Self {
            start: Square::from("a1"),
            dest: Square::from("a1"),
            captured_piece: None,
            promoted_to: None,

            is_castles: false,
            en_passant: false,
            is_double_pawn_push: false,

            halfmove_clock: 0,
            castling_rights: CastlingRights {
                white_kingside: CastlingStatus::Availiable,
                white_queenside: CastlingStatus::Availiable,
                black_kingside: CastlingStatus::Availiable,
                black_queenside: CastlingStatus::Availiable,
            },
        }
    }
}

impl Ply {
    pub const fn new(start: Square, dest: Square) -> Self {
        Self {
            start,
            dest,
            captured_piece: None,
            promoted_to: None,

            is_castles: false,
            en_passant: false,
            is_double_pawn_push: false,

            halfmove_clock: 0,
            castling_rights: CastlingRights {
                white_kingside: CastlingStatus::Availiable,
                white_queenside: CastlingStatus::Availiable,
                black_kingside: CastlingStatus::Availiable,
                black_queenside: CastlingStatus::Availiable,
            },
        }
    }

    pub fn parse_move(move_str: &str) -> Self {
        let start = Square::from(&move_str[0..2]);
        let dest = Square::from(&move_str[2..4]);

        Self::new(start, dest)
    }

    #[allow(dead_code)]
    pub const fn builder(start: Square, dest: Square) -> Builder {
        Builder::new(start, dest)
    }
}

impl fmt::Display for Ply {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.start, self.dest,)?;
        if let Some(captured_piece) = self.captured_piece {
            write!(f, " (captured: {captured_piece})")?;
        }

        if let Some(promoted_to) = self.promoted_to {
            write!(f, " (promoted to: {promoted_to})")?;
        }

        if self.is_castles {
            write!(f, " (castles)")?;
        }

        Ok(())
    }
}
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::super::piece::{Color, Kind};
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_derived_traits() {
        let start = Square::from("f4");
        let dest = Square::from("d6");
        let ply = Ply::new(start, dest);
        dbg!(&ply);

        assert_eq!(ply, ply.clone());
    }

    #[test]
    fn test_display() {
        let start = Square::from("f4");
        let dest = Square::from("d6");
        let ply = Ply::new(start, dest);

        let result = ply.to_string();
        let correct = format!("{start} -> {dest}");

        assert_eq!(result, correct);
    }

    #[test]
    fn test_display_with_capture() {
        let start = Square::from("f4");
        let dest = Square::from("d6");
        let captured_piece = Kind::Pawn(Color::White);
        let ply = Ply::builder(start, dest).captured(captured_piece).build();

        let result = ply.to_string();
        let correct = format!("{start} -> {dest} (captured: ♟)");

        assert_eq!(result, correct);
    }

    #[test]
    fn test_display_with_promotion() {
        let start = Square::from("d7");
        let dest = Square::from("d8");
        let promoted_to = Kind::Queen(Color::White);
        let ply = Ply::builder(start, dest).promoted_to(promoted_to).build();

        let result = ply.to_string();
        let correct = format!("{start} -> {dest} (promoted to: ♛)");

        assert_eq!(result, correct);
    }

    #[test]
    fn test_display_with_capture_and_promotion() {
        let start = Square::from("d7");
        let dest = Square::from("e8");
        let captured_piece = Kind::Rook(Color::Black);
        let promoted_to = Kind::Queen(Color::White);
        let ply = Ply::builder(start, dest)
            .captured(captured_piece)
            .promoted_to(promoted_to)
            .build();

        let result = ply.to_string();
        let correct = format!("{start} -> {dest} (captured: ♖) (promoted to: ♛)");

        assert_eq!(result, correct);
    }

    #[test]
    fn test_builder() {
        let start = Square::from("f4");
        let dest = Square::from("d6");
        let ply = Ply::builder(start, dest).build();

        assert_eq!(ply.start, start);
        assert_eq!(ply.dest, dest);
        assert_eq!(ply.captured_piece, None);
        assert_eq!(ply.promoted_to, None);
    }

    #[test]
    fn test_builder_captured() {
        let start = Square::from("f4");
        let dest = Square::from("d6");
        let captured_piece = Kind::Rook(Color::Black);
        let ply = Ply::builder(start, dest).captured(captured_piece).build();

        assert_eq!(ply.start, start);
        assert_eq!(ply.dest, dest);
        assert_eq!(ply.captured_piece, Some(captured_piece));
        assert_eq!(ply.promoted_to, None);
    }

    #[test]
    fn test_builder_promoted() {
        let start = Square::from("f7");
        let dest = Square::from("f8");
        let promoted_to = Kind::Rook(Color::Black);
        let ply = Ply::builder(start, dest).promoted_to(promoted_to).build();

        assert_eq!(ply.start, start);
        assert_eq!(ply.dest, dest);
        assert_eq!(ply.captured_piece, None);
        assert_eq!(ply.promoted_to, Some(promoted_to));
    }

    #[test]
    fn test_builder_captured_and_promoted() {
        let start = Square::from("f7");
        let dest = Square::from("f8");
        let captured_piece = Kind::Queen(Color::White);
        let promoted_to = Kind::Rook(Color::Black);
        let ply = Ply::builder(start, dest)
            .captured(captured_piece)
            .promoted_to(promoted_to)
            .build();

        assert_eq!(ply.start, start);
        assert_eq!(ply.dest, dest);
        assert_eq!(ply.captured_piece, Some(captured_piece));
        assert_eq!(ply.promoted_to, Some(promoted_to));
    }

    #[test]
    fn test_builder_castles() {
        let start = Square::from("e1");
        let dest = Square::from("g1");
        let ply = Ply::builder(start, dest).castles(true).build();

        assert_eq!(ply.start, start);
        assert_eq!(ply.dest, dest);
        assert!(ply.is_castles);
    }

    #[test]
    fn test_builder_en_passant() {
        let start = Square::from("e6");
        let dest = Square::from("d7");
        let ply = Ply::builder(start, dest).en_passant(true).build();

        assert_eq!(ply.start, start);
        assert_eq!(ply.dest, dest);
        assert!(ply.en_passant);
    }
}
