use super::{Column, Row, Square};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct BitBoard(u64);

impl BitBoard {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

impl From<Square> for BitBoard {
    fn from(value: Square) -> Self {
        SQUARES[value]
    }
}

impl From<Row> for BitBoard {
    fn from(value: Row) -> Self {
        ROWS[value]
    }
}

impl From<Column> for BitBoard {
    fn from(value: Column) -> Self {
        COLUMNS[value]
    }
}

impl BitAnd for BitBoard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for BitBoard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitXor for BitBoard {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

impl Not for BitBoard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

pub static ROWS: [BitBoard; 8] = [
    Row::One.as_bitboard(),
    Row::Two.as_bitboard(),
    Row::Three.as_bitboard(),
    Row::Four.as_bitboard(),
    Row::Five.as_bitboard(),
    Row::Six.as_bitboard(),
    Row::Seven.as_bitboard(),
    Row::Eight.as_bitboard(),
];

pub static COLUMNS: [BitBoard; 8] = [
    Column::A.as_bitboard(),
    Column::B.as_bitboard(),
    Column::C.as_bitboard(),
    Column::D.as_bitboard(),
    Column::E.as_bitboard(),
    Column::F.as_bitboard(),
    Column::G.as_bitboard(),
    Column::H.as_bitboard(),
];

static SQUARES: [BitBoard; 64] = gen_sqs();

const fn gen_sqs() -> [BitBoard; 64] {
    let mut array = [BitBoard::new(0); 64];
    let mut counter = 0;
    while counter < 64 {
        array[counter as usize] = Square::from_u8(counter).as_bitboard();
        counter += 1;
    }
    array
}
