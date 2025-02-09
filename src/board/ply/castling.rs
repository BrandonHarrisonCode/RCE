#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(clippy::module_name_repetitions)]
pub enum CastlingStatus {
    #[default]
    Available,
    Unavailable,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(clippy::module_name_repetitions)]
pub enum CastlingKind {
    WhiteKingside,
    WhiteQueenside,
    BlackKingside,
    BlackQueenside,
}

impl From<CastlingKind> for usize {
    fn from(kind: CastlingKind) -> Self {
        match kind {
            CastlingKind::WhiteKingside => 0,
            CastlingKind::WhiteQueenside => 1,
            CastlingKind::BlackKingside => 2,
            CastlingKind::BlackQueenside => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(clippy::module_name_repetitions)]
pub struct CastlingRights {
    pub white_kingside: CastlingStatus,
    pub white_queenside: CastlingStatus,
    pub black_kingside: CastlingStatus,
    pub black_queenside: CastlingStatus,
}

impl Default for CastlingRights {
    fn default() -> Self {
        Self {
            white_kingside: CastlingStatus::Available,
            white_queenside: CastlingStatus::Available,
            black_kingside: CastlingStatus::Available,
            black_queenside: CastlingStatus::Available,
        }
    }
}

impl CastlingRights {
    pub const fn new() -> Self {
        Self {
            white_kingside: CastlingStatus::Available,
            white_queenside: CastlingStatus::Available,
            black_kingside: CastlingStatus::Available,
            black_queenside: CastlingStatus::Available,
        }
    }
}
