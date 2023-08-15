use indoc::indoc;

#[allow(dead_code)]
pub fn debug_bitboard(bitboard: &u64) {
    let output = debug_bitboard_helper(bitboard);
    println!("{}", output);
}

pub fn debug_bitboard_helper(bitboard: &u64) -> String {
    let mut builder = String::new();
    let mask = 0xFF;

    builder.push_str(&format!("Debug bitboard: {:0>64b}\n", bitboard).to_string());
    for i in (0..8).rev() {
        builder.push_str(&format!("{:0>8b}\n", (bitboard >> (8 * i)) & mask));
    }

    builder
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_bitboard1() {
        let bb = 0b0000000000000000000000000000000000000000000000001111111100000000;

        let msg = debug_bitboard_helper(&bb);
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
        let bb = 0b0100001000000000000000000000000000000000000000000000000000000000;

        let msg = debug_bitboard_helper(&bb);
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
