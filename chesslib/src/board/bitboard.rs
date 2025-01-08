use super::{Column, Row, Square};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

const NOT_COL_A: u64 = 0xfefefefefefefefe;
const NOT_COL_H: u64 = 0x7f7f7f7f7f7f7f7f;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct BitBoard(u64);

impl BitBoard {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn bitscan_forward(&self) -> Option<Square> {
        match self.0.trailing_zeros() {
            64 => None,
            // Safety: x < 64
            x => unsafe { Some(Square::from_u8(x as u8)) },
        }
    }

    pub fn count_squares(&self) -> u8 {
        self.0.count_ones() as u8
    }

    pub fn king_moves(square: Square) -> Self {
        KING_MOVES[square]
    }

    const fn king_attack_mask(square: Square) -> Self {
        let square_mask = square.bitboard().0;
        let lateral_mask = ((square_mask << 1) & NOT_COL_A) | ((square_mask >> 1) & NOT_COL_H);
        let screen_mask = lateral_mask | square_mask;
        Self(lateral_mask | (screen_mask << 8) | (screen_mask >> 8))
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

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitOr for BitBoard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl BitXor for BitBoard {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
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

const fn gen_sqs() -> [BitBoard; 64] {
    let mut array = [BitBoard::new(0); 64];
    let mut counter = 0;
    while counter < 64 {
        // Safety: counter < 64
        array[counter as usize] = unsafe { Square::from_u8(counter).bitboard() };
        counter += 1;
    }
    array
}

const fn gen_king_moves() -> [BitBoard; 64] {
    let mut array = [BitBoard::new(0); 64];
    let mut counter = 0;
    while counter < 64 {
        // Safety: counter < 64
        let square = unsafe { Square::from_u8(counter) };
        array[counter as usize] = BitBoard::king_attack_mask(square);
        counter += 1;
    }
    array
}

static COLUMNS: [BitBoard; 8] = [
    Column::A.bitboard(),
    Column::B.bitboard(),
    Column::C.bitboard(),
    Column::D.bitboard(),
    Column::E.bitboard(),
    Column::F.bitboard(),
    Column::G.bitboard(),
    Column::H.bitboard(),
];
static ROWS: [BitBoard; 8] = [
    Row::One.bitboard(),
    Row::Two.bitboard(),
    Row::Three.bitboard(),
    Row::Four.bitboard(),
    Row::Five.bitboard(),
    Row::Six.bitboard(),
    Row::Seven.bitboard(),
    Row::Eight.bitboard(),
];
static SQUARES: [BitBoard; 64] = gen_sqs();
static KING_MOVES: [BitBoard; 64] = gen_king_moves();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_king_moves() {
        let king_moves = BitBoard::king_moves(Square::A1);
        let expected = BitBoard::from(Square::B1) | Square::A2.into() | Square::B2.into();
        assert_eq!(king_moves, expected);

        let king_moves = BitBoard::king_moves(Square::G7);
        let expected = BitBoard::from(Square::F6)
            | Square::F7.into()
            | Square::F8.into()
            | Square::G6.into()
            | Square::G8.into()
            | Square::H6.into()
            | Square::H7.into()
            | Square::H8.into();
        assert_eq!(king_moves, expected);
    }

    #[test]
    fn test_bitscan_forward() {
        let bitboard = BitBoard::from(Row::One);
        let lsb = bitboard.bitscan_forward();
        assert_eq!(lsb, Some(Square::A1));

        let bitboard = BitBoard::from(Row::Eight);
        let lsb = bitboard.bitscan_forward();
        assert_eq!(lsb, Some(Square::A8));

        let bitboard = BitBoard::from(Column::A);
        let lsb = bitboard.bitscan_forward();
        assert_eq!(lsb, Some(Square::A1));

        let bitboard = BitBoard::from(Column::H);
        let lsb = bitboard.bitscan_forward();
        assert_eq!(lsb, Some(Square::H1));

        let bitboard = BitBoard::from(Square::A1);
        let lsb = bitboard.bitscan_forward();
        assert_eq!(lsb, Some(Square::A1));

        let bitboard = BitBoard::from(Square::H8);
        let lsb = bitboard.bitscan_forward();
        assert_eq!(lsb, Some(Square::H8));

        let bitboard = BitBoard::new(std::u64::MAX);
        let lsb = bitboard.bitscan_forward();
        assert_eq!(lsb, Some(Square::A1));

        let bitboard = BitBoard::new(0);
        let lsb = bitboard.bitscan_forward();
        assert_eq!(lsb, None);
    }
}
