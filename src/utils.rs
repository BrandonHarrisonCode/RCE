#[allow(dead_code)]
pub fn debug_bitboard(bitboard: &u64) {
    let mask = 0xFF;
    println!("Debug bitboard: {:0>64b}", bitboard);
    for i in (0..8).rev() {
        println!("{:0>8b}", (bitboard >> (8 * i)) & mask);
    }
}
