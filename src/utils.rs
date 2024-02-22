#[allow(dead_code)]
pub fn debug_bitboard(bitboard: u64) -> String {
    debug_bitboard_helper(bitboard)
}

pub fn debug_bitboard_helper(bitboard: u64) -> String {
    let mut builder = String::new();
    let mask = 0xFF;

    builder.push_str(&format!("Debug bitboard: {bitboard:0>64b}\n"));
    for i in (0..8).rev() {
        builder.push_str(&format!("{:0>8b}\n", (bitboard >> (8 * i)) & mask));
    }

    builder
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_debug_bitboard_no_panic() {
        let bb = 0b_00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000;

        debug_bitboard(bb);
    }

    #[test]
    fn test_debug_bitboard1() {
        let bb = 0b_00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000;

        let msg = debug_bitboard_helper(bb);
        let correct = indoc! {"
            Debug bitboard: 0000000000000000000000000000000000000000000000001111111100000000
            00000000
            00000000
            00000000
            00000000
            00000000
            00000000
            11111111
            00000000
        "};

        assert_eq!(msg, correct);
    }

    #[test]
    fn test_debug_bitboard2() {
        let bb = 0b_01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000;

        let msg = debug_bitboard_helper(bb);
        let correct = indoc! {"
            Debug bitboard: 0100001000000000000000000000000000000000000000000000000000000000
            01000010
            00000000
            00000000
            00000000
            00000000
            00000000
            00000000
            00000000
        "};

        assert_eq!(msg, correct);
    }
}
