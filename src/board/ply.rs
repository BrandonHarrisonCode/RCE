use std::fmt;

use super::{piece::Kind, square::Square};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ply {
    pub start: Square,
    pub dest: Square,
    pub captured_piece: Option<Kind>,
    pub promoted_to: Option<Kind>,
    pub is_castles: bool,
    pub en_passant: bool,
    pub is_double_pawn_push: bool,
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
        }
    }

    pub fn parse_move(move_str: &str) -> Self {
        let start = Square::new(&move_str[0..2]);
        let dest = Square::new(&move_str[2..4]);

        Self::new(start, dest)
    }

    #[allow(dead_code)]
    pub const fn builder(start: Square, dest: Square) -> Builder {
        Builder {
            start,
            dest,
            captured_piece: None,
            promoted_to: None,
            castles: false,
            en_passant: false,
            double_pawn_push: false,
        }
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

#[derive(Default)]
pub struct Builder {
    start: Square,
    dest: Square,
    captured_piece: Option<Kind>,
    promoted_to: Option<Kind>,
    castles: bool,
    en_passant: bool,
    double_pawn_push: bool,
}

impl Builder {
    #[allow(dead_code)]
    pub fn start(&mut self, start: Square) -> &mut Self {
        self.start = start;
        self
    }

    #[allow(dead_code)]
    pub fn dest(&mut self, dest: Square) -> &mut Self {
        self.dest = dest;
        self
    }

    #[allow(dead_code)]
    pub fn captured(&mut self, captured_piece: Kind) -> &mut Self {
        self.captured_piece = Some(captured_piece);
        self
    }

    #[allow(dead_code)]
    pub fn promoted_to(&mut self, promoted_to: Kind) -> &mut Self {
        self.promoted_to = Some(promoted_to);
        self
    }

    #[allow(dead_code)]
    pub fn castles(&mut self, is_castles: bool) -> &mut Self {
        self.castles = is_castles;
        self
    }

    #[allow(dead_code)]
    pub fn en_passant(&mut self, is_en_passant: bool) -> &mut Self {
        self.en_passant = is_en_passant;
        self
    }

    #[allow(dead_code)]
    pub fn double_pawn_push(&mut self, is_double_pawn_push: bool) -> &mut Self {
        self.double_pawn_push = is_double_pawn_push;
        self
    }

    pub const fn build(&self) -> Ply {
        Ply {
            start: self.start,
            dest: self.dest,
            captured_piece: self.captured_piece,
            promoted_to: self.promoted_to,
            is_castles: self.castles,
            en_passant: self.en_passant,
            is_double_pawn_push: self.double_pawn_push,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::super::piece::{Color, Kind};
    use super::*;

    #[test]
    fn test_derived_traits() {
        let start = Square::new("f4");
        let dest = Square::new("d6");
        let ply = Ply::new(start, dest);
        dbg!(&ply);

        assert_eq!(ply, ply.clone());
    }

    #[test]
    fn test_display() {
        let start = Square::new("f4");
        let dest = Square::new("d6");
        let ply = Ply::new(start, dest);

        let result = ply.to_string();
        let correct = format!("{start} -> {dest}");

        assert_eq!(result, correct);
    }

    #[test]
    fn test_display_with_capture() {
        let start = Square::new("f4");
        let dest = Square::new("d6");
        let captured_piece = Kind::Pawn(Color::White);
        let ply = Ply::builder(start, dest).captured(captured_piece).build();

        let result = ply.to_string();
        let correct = format!("{start} -> {dest} (captured: ♟)");

        assert_eq!(result, correct);
    }

    #[test]
    fn test_display_with_promotion() {
        let start = Square::new("d7");
        let dest = Square::new("d8");
        let promoted_to = Kind::Queen(Color::White);
        let ply = Ply::builder(start, dest).promoted_to(promoted_to).build();

        let result = ply.to_string();
        let correct = format!("{start} -> {dest} (promoted to: ♛)");

        assert_eq!(result, correct);
    }

    #[test]
    fn test_display_with_capture_and_promotion() {
        let start = Square::new("d7");
        let dest = Square::new("e8");
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
        let start = Square::new("f4");
        let dest = Square::new("d6");
        let ply = Ply::builder(start, dest).build();

        assert_eq!(ply.start, start);
        assert_eq!(ply.dest, dest);
        assert_eq!(ply.captured_piece, None);
        assert_eq!(ply.promoted_to, None);
    }

    #[test]
    fn test_builder_captured() {
        let start = Square::new("f4");
        let dest = Square::new("d6");
        let captured_piece = Kind::Rook(Color::Black);
        let ply = Ply::builder(start, dest).captured(captured_piece).build();

        assert_eq!(ply.start, start);
        assert_eq!(ply.dest, dest);
        assert_eq!(ply.captured_piece, Some(captured_piece));
        assert_eq!(ply.promoted_to, None);
    }

    #[test]
    fn test_builder_promoted() {
        let start = Square::new("f7");
        let dest = Square::new("f8");
        let promoted_to = Kind::Rook(Color::Black);
        let ply = Ply::builder(start, dest).promoted_to(promoted_to).build();

        assert_eq!(ply.start, start);
        assert_eq!(ply.dest, dest);
        assert_eq!(ply.captured_piece, None);
        assert_eq!(ply.promoted_to, Some(promoted_to));
    }

    #[test]
    fn test_builder_captured_and_promoted() {
        let start = Square::new("f7");
        let dest = Square::new("f8");
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
        let start = Square::new("e1");
        let dest = Square::new("g1");
        let ply = Ply::builder(start, dest).castles(true).build();

        assert_eq!(ply.start, start);
        assert_eq!(ply.dest, dest);
        assert!(ply.is_castles);
    }

    #[test]
    fn test_builder_en_passant() {
        let start = Square::new("e6");
        let dest = Square::new("d7");
        let ply = Ply::builder(start, dest).en_passant(true).build();

        assert_eq!(ply.start, start);
        assert_eq!(ply.dest, dest);
        assert!(ply.en_passant);
    }
}
