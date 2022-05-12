use lazy_static::lazy_static;
use std::ops::{Index, IndexMut};

use super::{
    bitboard::{set_bit, Bitboard},
    piece::Color,
};

lazy_static! {
    pub static ref PAWN_ATTACKS: PawnAttackTable = pawn_attacks();
    pub static ref KNIGHT_ATTACKS: AttackTable =
        AttackTable::generate_attack_table(mask_knight_attacks);
    pub static ref KING_ATTACKS: AttackTable =
        AttackTable::generate_attack_table(mask_king_attacks);
}

const NOT_A_FILE: u64 = 18374403900871474942;
const NOT_H_FILE: u64 = 9187201950435737471;
const NOT_HG_FILE: u64 = 4557430888798830399;
const NOT_AB_FILE: u64 = 18229723555195321596;

pub struct AttackTable {
    attacks: [Bitboard; 64],
}

impl AttackTable {
    fn generate_attack_table(mask_attack_function: fn(u64) -> Bitboard) -> Self {
        let mut attacks = [0; 64];
        for square in 0..64u64 {
            attacks[square as usize] = mask_attack_function(square);
        }

        AttackTable { attacks }
    }
}

impl<T: Into<u64>> Index<T> for AttackTable {
    type Output = Bitboard;

    fn index(&self, index: T) -> &Bitboard {
        &self.attacks[index.into() as usize]
    }
}

impl<T: Into<u64>> IndexMut<T> for AttackTable {
    fn index_mut(&mut self, index: T) -> &mut Bitboard {
        &mut self.attacks[index.into() as usize]
    }
}

pub struct PawnAttackTable {
    attacks: [AttackTable; 2],
}

impl PawnAttackTable {
    fn new(white: [Bitboard; 64], black: [Bitboard; 64]) -> Self {
        PawnAttackTable {
            attacks: [
                AttackTable { attacks: white },
                AttackTable { attacks: black },
            ],
        }
    }
}

impl Index<Color> for PawnAttackTable {
    type Output = AttackTable;

    fn index(&self, index: Color) -> &AttackTable {
        &self.attacks[index as usize]
    }
}

impl IndexMut<Color> for PawnAttackTable {
    fn index_mut(&mut self, index: Color) -> &mut AttackTable {
        &mut self.attacks[index as usize]
    }
}

pub fn mask_pawn_attacks(side: Color, square: impl Into<u64>) -> Bitboard {
    let mut attacks = 0;
    let mut bitboard = 0;
    set_bit(&mut bitboard, square);

    if side.into() {
        if ((bitboard >> 7) & NOT_A_FILE) != 0 {
            attacks |= bitboard >> 7;
        }
        if ((bitboard >> 9) & NOT_H_FILE) != 0 {
            attacks |= bitboard >> 9;
        }
    } else {
        if ((bitboard << 7) & NOT_H_FILE) != 0 {
            attacks |= bitboard << 7;
        }
        if ((bitboard << 9) & NOT_A_FILE) != 0 {
            attacks |= bitboard << 9;
        }
    }

    attacks
}

pub fn pawn_attacks() -> PawnAttackTable {
    let mut white_attacks = [0; 64];
    let mut black_attacks = [0; 64];
    for square in 0..64u64 {
        white_attacks[square as usize] = mask_pawn_attacks(Color::White, square);
        black_attacks[square as usize] = mask_pawn_attacks(Color::Black, square);
    }

    PawnAttackTable::new(white_attacks, black_attacks)
}

pub fn mask_knight_attacks(square: impl Into<u64>) -> Bitboard {
    let mut attacks = 0;
    let mut bitboard = 0;
    set_bit(&mut bitboard, square);

    if ((bitboard >> 17) & NOT_H_FILE) != 0 {
        attacks |= bitboard >> 17;
    }
    if ((bitboard >> 15) & NOT_A_FILE) != 0 {
        attacks |= bitboard >> 15;
    }
    if ((bitboard >> 10) & NOT_HG_FILE) != 0 {
        attacks |= bitboard >> 10;
    }
    if ((bitboard >> 6) & NOT_AB_FILE) != 0 {
        attacks |= bitboard >> 6;
    }

    if ((bitboard << 17) & NOT_A_FILE) != 0 {
        attacks |= bitboard << 17;
    }
    if ((bitboard << 15) & NOT_H_FILE) != 0 {
        attacks |= bitboard << 15;
    }
    if ((bitboard << 10) & NOT_AB_FILE) != 0 {
        attacks |= bitboard << 10;
    }
    if ((bitboard << 6) & NOT_HG_FILE) != 0 {
        attacks |= bitboard << 6;
    }

    attacks
}

pub fn mask_king_attacks(square: impl Into<u64>) -> Bitboard {
    let mut attacks = 0;
    let mut bitboard = 0;
    set_bit(&mut bitboard, square);

    if (bitboard >> 8) != 0 {
        attacks |= bitboard >> 8;
    }
    if ((bitboard >> 9) & NOT_H_FILE) != 0 {
        attacks |= bitboard >> 9;
    }
    if ((bitboard >> 7) & NOT_A_FILE) != 0 {
        attacks |= bitboard >> 7;
    }
    if ((bitboard >> 1) & NOT_H_FILE) != 0 {
        attacks |= bitboard >> 1;
    }

    if (bitboard << 8) != 0 {
        attacks |= bitboard << 8;
    }
    if ((bitboard << 9) & NOT_A_FILE) != 0 {
        attacks |= bitboard << 9;
    }
    if ((bitboard << 7) & NOT_H_FILE) != 0 {
        attacks |= bitboard << 7;
    }
    if ((bitboard << 1) & NOT_A_FILE) != 0 {
        attacks |= bitboard << 1;
    }

    attacks
}
