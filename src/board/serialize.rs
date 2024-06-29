use super::{Board, BoardBuilder, CastlingKind, CastlingStatus, Color, Ply, Square};

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

        let mask: u64 = 1 << (8 * (7 - idx / 8) + idx % 8);
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
        .castling(CastlingKind::WhiteKingside, CastlingStatus::Unavailiable)
        .castling(CastlingKind::BlackKingside, CastlingStatus::Unavailiable)
        .castling(CastlingKind::WhiteQueenside, CastlingStatus::Unavailiable)
        .castling(CastlingKind::BlackQueenside, CastlingStatus::Unavailiable);

    for chr in str.chars() {
        builder = match chr {
            'K' => builder.castling(CastlingKind::WhiteKingside, CastlingStatus::Availiable),
            'k' => builder.castling(CastlingKind::BlackKingside, CastlingStatus::Availiable),
            'Q' => builder.castling(CastlingKind::WhiteQueenside, CastlingStatus::Availiable),
            'q' => builder.castling(CastlingKind::BlackQueenside, CastlingStatus::Availiable),
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

fn history(mut builder: BoardBuilder) -> BoardBuilder {
    let rights = builder.get_last_history().castling_rights;
    let mut history = Vec::new();

    if let Some(file) = builder.en_passant_file {
        let (start, dest) = match builder.current_turn {
            Color::White => (Square { rank: 1, file }, Square { rank: 3, file }),
            Color::Black => (Square { rank: 6, file }, Square { rank: 4, file }),
        };

        let ply = Ply::builder(start, dest)
            .double_pawn_push(true)
            .castling_rights(rights)
            .halfmove_clock(builder.halfmove_clock)
            .build();
        history.push(ply);
    } else {
        let ply = Ply::builder(Square::from("a1"), Square::from("a1"))
            .castling_rights(rights)
            .halfmove_clock(builder.halfmove_clock)
            .build();
        history.push(ply);
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
        builder = halfmove_clock(builder, fields.get(4).unwrap_or(&"0"));
        builder = fullmove_counter(builder, fields.get(5).unwrap_or(&"1"));
        builder = history(builder);

        builder.build()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn from_fen_starting_position() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(
            BoardBuilder::construct_starting_board(),
            Board::from_fen(fen)
        );
    }

    #[test]
    fn from_fen_white_position1() {
        let fen = "1k1r3r/p6p/1pp1pp2/2Np1qp1/1Q1P4/2P1PP2/PP4PP/R4nK1 w - - 0 21";
        let correct = BoardBuilder::new()
            .halfmove_clock(0)
            .fullmove_counter(21)
            .pawns(Color::White, 137675520)
            .pawns(Color::Black, 36369954861219840)
            .king(Color::White, 64)
            .king(Color::Black, 144115188075855872)
            .queens(Color::White, 33554432)
            .queens(Color::Black, 137438953472)
            .rooks(Color::White, 1)
            .rooks(Color::Black, 9799832789158199296)
            .bishops(Color::White, 0)
            .bishops(Color::Black, 0)
            .knights(Color::White, 17179869184)
            .knights(Color::Black, 32)
            .castling(CastlingKind::WhiteKingside, CastlingStatus::Unavailiable)
            .castling(CastlingKind::BlackKingside, CastlingStatus::Unavailiable)
            .castling(CastlingKind::WhiteQueenside, CastlingStatus::Unavailiable)
            .castling(CastlingKind::BlackQueenside, CastlingStatus::Unavailiable)
            .build();

        assert_eq!(Board::from_fen(fen), correct);
    }

    #[test]
    fn from_fen_black_position1() {
        let fen = "5b2/pp1N2pk/2pn1q1p/3n1p1Q/3P1P2/2PB3R/PP3KPP/R1B1r3 b - - 12 31";
        let correct = BoardBuilder::new()
            .turn(Color::Black)
            .halfmove_clock(12)
            .fullmove_counter(31)
            .pawns(Color::White, 671400704)
            .pawns(Color::Black, 19004096413433856)
            .king(Color::White, 8192)
            .king(Color::Black, 36028797018963968)
            .queens(Color::White, 549755813888)
            .queens(Color::Black, 35184372088832)
            .rooks(Color::White, 8388609)
            .rooks(Color::Black, 16)
            .bishops(Color::White, 524292)
            .bishops(Color::Black, 2305843009213693952)
            .knights(Color::White, 2251799813685248)
            .knights(Color::Black, 8830452760576)
            .castling(CastlingKind::WhiteKingside, CastlingStatus::Unavailiable)
            .castling(CastlingKind::BlackKingside, CastlingStatus::Unavailiable)
            .castling(CastlingKind::WhiteQueenside, CastlingStatus::Unavailiable)
            .castling(CastlingKind::BlackQueenside, CastlingStatus::Unavailiable)
            .build();

        let from_fen = Board::from_fen(fen);
        assert_eq!(from_fen, correct);
    }
}
