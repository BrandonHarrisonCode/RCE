use std::fmt;

use super::{piece::Kind, square::Square, CastlingStatus, Color};

mod builder;
pub mod castling;

use builder::Builder;
use castling::CastlingRights;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct Ply {
    pub start: Square,
    pub dest: Square,
    pub piece: Kind,
    pub captured_piece: Option<Kind>,
    pub promoted_to: Option<Kind>,

    pub is_castles: bool,
    pub en_passant: bool,
    pub is_double_pawn_push: bool,

    pub halfmove_clock: u16,
    pub castling_rights: CastlingRights,
}

impl Default for Ply {
    fn default() -> Self {
        Self {
            start: Square::from("a1"),
            dest: Square::from("a1"),
            piece: Kind::Pawn(Color::White),
            captured_piece: None,
            promoted_to: None,

            is_castles: false,
            en_passant: false,
            is_double_pawn_push: false,

            halfmove_clock: 0,
            castling_rights: CastlingRights {
                white_kingside: CastlingStatus::Available,
                white_queenside: CastlingStatus::Available,
                black_kingside: CastlingStatus::Available,
                black_queenside: CastlingStatus::Available,
            },
        }
    }
}

impl Ply {
    pub const fn new(start: Square, dest: Square, piece: Kind) -> Self {
        Self {
            start,
            dest,
            piece,
            captured_piece: None,
            promoted_to: None,

            is_castles: false,
            en_passant: false,
            is_double_pawn_push: false,

            halfmove_clock: 0,
            castling_rights: CastlingRights {
                white_kingside: CastlingStatus::Available,
                white_queenside: CastlingStatus::Available,
                black_kingside: CastlingStatus::Available,
                black_queenside: CastlingStatus::Available,
            },
        }
    }

    #[allow(dead_code)]
    pub const fn builder(start: Square, dest: Square, piece: Kind) -> Builder {
        Builder::new(start, dest, piece)
    }

    pub const fn is_capture(&self) -> bool {
        self.captured_piece.is_some()
    }

    pub const fn is_promotion(&self) -> bool {
        self.promoted_to.is_some()
    }

    pub const fn is_quiet(&self) -> bool {
        !self.is_capture() && !self.is_promotion()
    }

    pub const fn is_en_passant(&self) -> bool {
        self.en_passant
    }

    pub const fn is_castles(&self) -> bool {
        self.is_castles
    }

    pub fn to_notation(self) -> String {
        let mut notation = format!("{}{}", self.start, self.dest);

        if let Some(promoted_to) = self.promoted_to {
            match promoted_to {
                Kind::Queen(_) => notation.push('q'),
                Kind::Rook(_) => notation.push('r'),
                Kind::Bishop(_) => notation.push('b'),
                Kind::Knight(_) => notation.push('n'),
                _ => unreachable!("Invalid promotion piece"),
            }
        }

        notation
    }
}

impl fmt::Display for Ply {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_notation())
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
        let ply = Ply::new(start, dest, Kind::Pawn(Color::White));
        dbg!(&ply);

        assert_eq!(ply, ply.clone());
    }

    #[test]
    fn test_display() {
        let start = Square::from("f4");
        let dest = Square::from("d6");
        let ply = Ply::new(start, dest, Kind::Pawn(Color::White));

        let result = ply.to_string();
        let correct = format!("{start}{dest}");

        assert_eq!(result, correct);
    }

    #[test]
    fn test_builder() {
        let start = Square::from("f4");
        let dest = Square::from("d6");
        let ply = Ply::builder(start, dest, Kind::Pawn(Color::White)).build();

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
        let ply = Ply::builder(start, dest, Kind::Queen(Color::White))
            .captured(Some(captured_piece))
            .build();

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
        let ply = Ply::builder(start, dest, Kind::Queen(Color::White))
            .promoted_to(promoted_to)
            .build();

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
        let ply = Ply::builder(start, dest, Kind::Pawn(Color::Black))
            .captured(Some(captured_piece))
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
        let ply = Ply::builder(start, dest, Kind::King(Color::White))
            .castles(true)
            .build();

        assert_eq!(ply.start, start);
        assert_eq!(ply.dest, dest);
        assert!(ply.is_castles);
    }

    #[test]
    fn test_builder_en_passant() {
        let start = Square::from("e6");
        let dest = Square::from("d7");
        let ply = Ply::builder(start, dest, Kind::Pawn(Color::Black))
            .en_passant(true)
            .build();

        assert_eq!(ply.start, start);
        assert_eq!(ply.dest, dest);
        assert!(ply.en_passant);
    }
}
