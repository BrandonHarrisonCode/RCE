use std::fmt;

use super::{piece::PieceKind, square::Square};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ply {
    pub start: Square,
    pub dest: Square,
    pub captured_piece: Option<PieceKind>,
    pub promoted_to: Option<PieceKind>,
}
impl Ply {
    pub fn new(start: Square, dest: Square) -> Ply {
        Ply {
            start,
            dest,
            captured_piece: None,
            promoted_to: None,
        }
    }

    #[allow(dead_code)]
    pub fn builder(start: Square, dest: Square) -> PlyBuilder {
        PlyBuilder {
            start,
            dest,
            captured_piece: None,
            promoted_to: None,
        }
    }
}

impl fmt::Display for Ply {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.start, self.dest,)?;
        if let Some(captured_piece) = self.captured_piece {
            write!(f, " (captured: {})", captured_piece)?;
        }

        if let Some(promoted_to) = self.promoted_to {
            write!(f, " (promoted to: {})", promoted_to)?;
        }

        Ok(())
    }
}

#[derive(Default)]
#[allow(dead_code)]
pub struct PlyBuilder {
    start: Square,
    dest: Square,
    captured_piece: Option<PieceKind>,
    promoted_to: Option<PieceKind>,
}

#[allow(dead_code)]
impl PlyBuilder {
    pub fn start(mut self, start: Square) -> PlyBuilder {
        self.start = start;
        self
    }

    pub fn dest(mut self, dest: Square) -> PlyBuilder {
        self.dest = dest;
        self
    }

    pub fn captured(mut self, captured_piece: PieceKind) -> PlyBuilder {
        self.captured_piece = Some(captured_piece);
        self
    }

    pub fn promoted_to(mut self, promoted_to: PieceKind) -> PlyBuilder {
        self.promoted_to = Some(promoted_to);
        self
    }

    pub fn build(self) -> Ply {
        Ply {
            start: self.start,
            dest: self.dest,
            captured_piece: self.captured_piece,
            promoted_to: self.promoted_to,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::piece::{PieceKind, Color};

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
        let correct = format!("{} -> {}", start, dest);

        assert_eq!(result, correct);
    }

    #[test]
    fn test_display_with_capture() {
        let start = Square::new("f4");
        let dest = Square::new("d6");
        let captured_piece = PieceKind::Pawn(Color::White);
        let ply = Ply::builder(start, dest).captured(captured_piece).build();

        let result = ply.to_string();
        let correct = format!("{} -> {} (captured: ♟)", start, dest);

        assert_eq!(result, correct);
    }

    #[test]
    fn test_display_with_promotion() {
        let start = Square::new("d7");
        let dest = Square::new("d8");
        let promoted_to = PieceKind::Queen(Color::White);
        let ply = Ply::builder(start, dest).promoted_to(promoted_to).build();

        let result = ply.to_string();
        let correct = format!("{} -> {} (promoted to: ♛)", start, dest);

        assert_eq!(result, correct);
    }

    #[test]
    fn test_display_with_capture_and_promotion() {
        let start = Square::new("d7");
        let dest = Square::new("e8");
        let captured_piece = PieceKind::Rook(Color::Black);
        let promoted_to = PieceKind::Queen(Color::White);
        let ply = Ply::builder(start, dest).captured(captured_piece).promoted_to(promoted_to).build();

        let result = ply.to_string();
        let correct = format!("{} -> {} (captured: ♖) (promoted to: ♛)", start, dest);

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
        let captured_piece = PieceKind::Rook(Color::Black);
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
        let promoted_to = PieceKind::Rook(Color::Black);
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
        let captured_piece = PieceKind::Queen(Color::White);
        let promoted_to = PieceKind::Rook(Color::Black);
        let ply = Ply::builder(start, dest).captured(captured_piece).promoted_to(promoted_to).build();

        assert_eq!(ply.start, start);
        assert_eq!(ply.dest, dest);
        assert_eq!(ply.captured_piece, Some(captured_piece));
        assert_eq!(ply.promoted_to, Some(promoted_to));
    }
}
