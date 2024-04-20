use super::{Board, BoardBuilder, Castling, Color, Ply, Square};

pub enum FENInstruction<'a> {
    Bitboard(&'a mut u64),
    NewRow(),
    Skip(u64),
}

fn piece_placement(builder: BoardBuilder, str: &str) -> BoardBuilder {
    let mut white_pawns: u64 = 0;
    let mut white_king: u64 = 0;
    let mut white_queens: u64 = 0;
    let mut white_rooks: u64 = 0;
    let mut white_bishops: u64 = 0;
    let mut white_knights: u64 = 0;
    let mut black_pawns: u64 = 0;
    let mut black_king: u64 = 0;
    let mut black_queens: u64 = 0;
    let mut black_rooks: u64 = 0;
    let mut black_bishops: u64 = 0;
    let mut black_knights: u64 = 0;

    let mut idx: u64 = 0;
    for chr in str.chars() {
        let instruction = match chr {
            'P' => FENInstruction::Bitboard(&mut white_pawns),
            'K' => FENInstruction::Bitboard(&mut white_king),
            'Q' => FENInstruction::Bitboard(&mut white_queens),
            'R' => FENInstruction::Bitboard(&mut white_rooks),
            'B' => FENInstruction::Bitboard(&mut white_bishops),
            'N' => FENInstruction::Bitboard(&mut white_knights),
            'p' => FENInstruction::Bitboard(&mut black_pawns),
            'k' => FENInstruction::Bitboard(&mut black_king),
            'q' => FENInstruction::Bitboard(&mut black_queens),
            'r' => FENInstruction::Bitboard(&mut black_rooks),
            'b' => FENInstruction::Bitboard(&mut black_bishops),
            'n' => FENInstruction::Bitboard(&mut black_knights),
            '1'..='8' => FENInstruction::Skip(chr.to_string().parse().ok().unwrap()),
            '/' => FENInstruction::NewRow(),
            _ => panic!("Unknown FEN instruction: {chr}"),
        };

        let mask: u64 = 1 << (63 - idx);
        match instruction {
            FENInstruction::Bitboard(bb) => *bb |= mask,
            FENInstruction::Skip(num) => idx += num - 1,
            FENInstruction::NewRow() => idx -= 1,
        }
        idx += 1;
    }

    builder
        .pawns(Color::White, white_pawns)
        .king(Color::White, white_king)
        .queens(Color::White, white_queens)
        .rooks(Color::White, white_rooks)
        .bishops(Color::White, white_bishops)
        .knights(Color::White, white_knights)
        .pawns(Color::Black, black_pawns)
        .king(Color::Black, black_king)
        .queens(Color::Black, black_queens)
        .rooks(Color::Black, black_rooks)
        .bishops(Color::Black, black_bishops)
        .knights(Color::Black, black_knights)
}

fn current_turn(builder: BoardBuilder, str: &str) -> BoardBuilder {
    match str.chars().next().unwrap_or('w') {
        'w' => builder.turn(Color::White),
        'b' => builder.turn(Color::Black),
        _ => panic!("Not given a valid FEN. The second field must either be a 'b' or a 'w'"),
    }
}

fn castling_rights(mut builder: BoardBuilder, str: &str) -> BoardBuilder {
    builder = builder
        .kingside_castling(Color::White, Castling::Unavailiable)
        .kingside_castling(Color::Black, Castling::Unavailiable)
        .queenside_castling(Color::White, Castling::Unavailiable)
        .queenside_castling(Color::Black, Castling::Unavailiable);

    for chr in str.chars() {
        builder = match chr {
            'K' => builder.kingside_castling(Color::White, Castling::Availiable),
            'k' => builder.kingside_castling(Color::Black, Castling::Availiable),
            'Q' => builder.queenside_castling(Color::White, Castling::Availiable),
            'q' => builder.queenside_castling(Color::Black, Castling::Availiable),
            '-' => builder,
            _ => panic!("Unknown FEN castling notation: {chr}"),
        };
    }

    builder
}

fn en_passant_file(builder: BoardBuilder, str: &str) -> BoardBuilder {
    #[allow(clippy::cast_possible_truncation)]
    builder.en_passant_file(match str.chars().next().unwrap_or('-') {
        '-' => None,
        'a'..='h' => Some((str.chars().next().unwrap() as u128 - 'a' as u128) as u8),
        _ => panic!("Unknown FEN en passant notation: {str}"),
    })
}

fn history(builder: BoardBuilder) -> BoardBuilder {
    // If the first move is en passant, add the previous pawn push to the
    // history so we can restore en passant rights
    let mut history = Vec::new();
    if let Some(file) = builder.en_passant_file {
        let (start, dest) = match builder.current_turn {
            Color::White => (Square { rank: 1, file }, Square { rank: 3, file }),
            Color::Black => (Square { rank: 6, file }, Square { rank: 4, file }),
        };

        history.push(Ply::builder(start, dest).en_passant(true).build());
    }

    builder.history(&history)
}

fn halfmove_clock(builder: BoardBuilder, str: &str) -> BoardBuilder {
    builder.halfmove_clock(str.parse().ok().unwrap())
}

fn fullmove_counter(builder: BoardBuilder, str: &str) -> BoardBuilder {
    builder.fullmove_counter(str.parse().ok().unwrap())
}

impl Board {
    /// Returns a new board given a FEN string
    ///
    /// # Examples
    /// ```
    /// let board = Board::from_fen("8/8/8/8/8/8/8/8 w - - 0 1");
    /// ```
    #[allow(dead_code)]
    pub fn from_fen(fen: &str) -> Self {
        let mut builder = Self::builder();
        let fields: Vec<&str> = fen.split_ascii_whitespace().collect();

        builder = piece_placement(builder, fields[0]);
        builder = current_turn(builder, fields[1]);
        builder = castling_rights(builder, fields[2]);
        builder = en_passant_file(builder, fields[3]);
        builder = halfmove_clock(builder, fields[4]);
        builder = fullmove_counter(builder, fields[5]);
        builder = history(builder);

        builder.build()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::super::bitboards::BitBoards;
    use super::*;

    #[test]
    fn from_fen_starting_position() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(Board::construct_starting_board(), Board::from_fen(fen));
    }

    #[test]
    fn from_fen_white_position1() {
        let fen = "1k1r3r/p6p/1pp1pp2/2Np1qp1/1Q1P4/2P1PP2/PP4PP/R4nK1 w - - 0 21";
        let correct = Board {
            current_turn: Color::White,
            halfmove_clock: 0,
            fullmove_counter: 21,

            white_kingside_castling: Castling::Unavailiable,
            white_queenside_castling: Castling::Unavailiable,
            black_kingside_castling: Castling::Unavailiable,
            black_queenside_castling: Castling::Unavailiable,

            en_passant_file: None,

            bitboards: BitBoards::builder()
                .pawns(Color::White, 271_368_960)
                .pawns(Color::Black, 36_429_096_560_885_760)
                .king(Color::White, 2)
                .king(Color::Black, 4_611_686_018_427_387_904)
                .queens(Color::White, 1_073_741_824)
                .queens(Color::Black, 17_179_869_184)
                .rooks(Color::White, 128)
                .rooks(Color::Black, 1_224_979_098_644_774_912)
                .bishops(Color::White, 0)
                .bishops(Color::Black, 0)
                .knights(Color::White, 137_438_953_472)
                .knights(Color::Black, 4)
                .build(),
            history: Vec::new(),
        };

        assert_eq!(Board::from_fen(fen), correct);
    }

    #[test]
    fn from_fen_black_position1() {
        let fen = "5b2/pp1N2pk/2pn1q1p/3n1p1Q/3P1P2/2PB3R/PP3KPP/R1B1r3 b - - 12 31";
        let correct = Board {
            current_turn: Color::Black,
            halfmove_clock: 12,
            fullmove_counter: 31,

            white_kingside_castling: Castling::Unavailiable,
            white_queenside_castling: Castling::Unavailiable,
            black_kingside_castling: Castling::Unavailiable,
            black_queenside_castling: Castling::Unavailiable,

            en_passant_file: None,

            bitboards: BitBoards::builder()
                .pawns(Color::White, 337_691_392)
                .pawns(Color::Black, 54_642_446_545_453_056)
                .king(Color::White, 1024)
                .king(Color::Black, 281_474_976_710_656)
                .queens(Color::White, 4_294_967_296)
                .queens(Color::Black, 4_398_046_511_104)
                .rooks(Color::White, 65664)
                .rooks(Color::Black, 8)
                .bishops(Color::White, 1_048_608)
                .bishops(Color::Black, 288_230_376_151_711_744)
                .knights(Color::White, 4_503_599_627_370_496)
                .knights(Color::Black, 17_660_905_521_152)
                .build(),

            history: Vec::new(),
        };

        assert_eq!(Board::from_fen(fen), correct);
    }
}
