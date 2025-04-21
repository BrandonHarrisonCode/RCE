use super::castling::{CastlingKind, CastlingRights, CastlingStatus};
use super::Ply;
use crate::board::piece::Kind;
use crate::board::square::Square;

#[derive(Default)]
pub struct Builder {
    start: Square,
    dest: Square,
    piece: Kind,
    captured_piece: Option<Kind>,
    promoted_to: Option<Kind>,

    castles: bool,
    en_passant: bool,
    double_pawn_push: bool,

    halfmove_clock: u16,
    castling_rights: CastlingRights,
}

impl Builder {
    pub const fn new(start: Square, dest: Square, piece: Kind) -> Self {
        Self {
            start,
            dest,
            piece,
            captured_piece: None,
            promoted_to: None,

            castles: false,
            en_passant: false,
            double_pawn_push: false,

            halfmove_clock: 0,
            castling_rights: CastlingRights::new(),
        }
    }

    #[allow(dead_code)]
    pub const fn start(&mut self, start: Square) -> &mut Self {
        self.start = start;
        self
    }

    #[allow(dead_code)]
    pub const fn dest(&mut self, dest: Square) -> &mut Self {
        self.dest = dest;
        self
    }

    #[allow(dead_code)]
    pub const fn captured(&mut self, captured_piece: Option<Kind>) -> &mut Self {
        self.captured_piece = captured_piece;
        self
    }

    #[allow(dead_code)]
    pub const fn promoted_to(&mut self, promoted_to: Kind) -> &mut Self {
        self.promoted_to = Some(promoted_to);
        self
    }

    #[allow(dead_code)]
    pub const fn castles(&mut self, is_castles: bool) -> &mut Self {
        self.castles = is_castles;
        self
    }

    #[allow(dead_code)]
    pub const fn en_passant(&mut self, is_en_passant: bool) -> &mut Self {
        self.en_passant = is_en_passant;
        self
    }

    #[allow(dead_code)]
    pub const fn double_pawn_push(&mut self, is_double_pawn_push: bool) -> &mut Self {
        self.double_pawn_push = is_double_pawn_push;
        self
    }

    #[allow(dead_code)]
    pub const fn halfmove_clock(&mut self, halfmove_clock: u16) -> &mut Self {
        self.halfmove_clock = halfmove_clock;
        self
    }

    #[allow(dead_code)]
    pub const fn castling_rights(&mut self, rights: CastlingRights) -> &mut Self {
        self.castling_rights = rights;
        self
    }

    #[allow(dead_code)]
    pub const fn castling_status(
        &mut self,
        kind: CastlingKind,
        status: CastlingStatus,
    ) -> &mut Self {
        match kind {
            CastlingKind::WhiteKingside => self.castling_rights.white_kingside = status,
            CastlingKind::WhiteQueenside => self.castling_rights.white_queenside = status,
            CastlingKind::BlackKingside => self.castling_rights.black_kingside = status,
            CastlingKind::BlackQueenside => self.castling_rights.black_queenside = status,
        }
        self
    }

    pub const fn build(&self) -> Ply {
        Ply {
            start: self.start,
            dest: self.dest,
            piece: self.piece,
            captured_piece: self.captured_piece,
            promoted_to: self.promoted_to,

            is_castles: self.castles,
            en_passant: self.en_passant,
            is_double_pawn_push: self.double_pawn_push,

            halfmove_clock: self.halfmove_clock,
            castling_rights: self.castling_rights,
        }
    }
}
