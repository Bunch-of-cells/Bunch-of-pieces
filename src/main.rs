mod board;
use board::*;

fn main() {
    print_bitboard(mask_king_attacks(D4));
}
