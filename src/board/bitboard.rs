pub use Square::*;

pub type Bitboard = u64;

#[inline(always)]
pub fn get_bit(bitboard: &Bitboard, square: impl Into<u64>) -> Bitboard {
    bitboard & (1 << square.into())
}

#[inline(always)]
pub fn set_bit(bitboard: &mut Bitboard, square: impl Into<u64>) {
    *bitboard |= 1 << square.into()
}

#[inline(always)]
pub fn pop_bit(bitboard: &mut Bitboard, sqaure: impl Into<u64>) {
    *bitboard &= !(1 << sqaure.into())
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Square {
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
}

impl From<Square> for u64 {
    fn from(square: Square) -> u64 {
        square as u64
    }
}

pub fn print_bitboard(bitboard: Bitboard) {
    println!();

    for rank in 0..8u64 {
        for file in 0..8 {
            if file == 0 {
                print!(" {} ", 8 - rank);
            }
            if get_bit(&bitboard, rank * 8 + file) != 0 {
                print!(" 1 ");
            } else {
                print!(" 0 ");
            }
        }
        println!();
    }

    println!("\n    a  b  c  d  e  f  g  h\n");
    println!("    Bitboard: {}\n", bitboard);
}
