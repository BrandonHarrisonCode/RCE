#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(clippy::module_name_repetitions)]
pub enum CastlingStatus {
    #[default]
    Availiable,
    Unavailiable,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(clippy::module_name_repetitions)]
pub enum CastlingKind {
    WhiteKingside,
    WhiteQueenside,
    BlackKingside,
    BlackQueenside,
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
            white_kingside: CastlingStatus::Availiable,
            white_queenside: CastlingStatus::Availiable,
            black_kingside: CastlingStatus::Availiable,
            black_queenside: CastlingStatus::Availiable,
        }
    }
}

impl CastlingRights {
    pub const fn new() -> Self {
        Self {
            white_kingside: CastlingStatus::Availiable,
            white_queenside: CastlingStatus::Availiable,
            black_kingside: CastlingStatus::Availiable,
            black_queenside: CastlingStatus::Availiable,
        }
    }
}
