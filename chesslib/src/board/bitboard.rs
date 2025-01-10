use super::{Column, Row, Square};
use crate::pieces::Color;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

macro_rules! gen_table {
    ($mask_fn:expr $(,$arg0:expr)*) => {{
        let mut array = [BitBoard::new(0); 64];
        let mut counter = 0;
        while counter < 64 {
            // Safety: counter < 64
            let square = unsafe { Square::from_u8_unchecked(counter) };
            array[counter as usize] = $mask_fn(square $(,$arg0)*);
            counter += 1;
        }
        array
    }};
}

const NOT_COL_A: u64 = 0xfefefefefefefefe;
const NOT_COL_H: u64 = 0x7f7f7f7f7f7f7f7f;

static COLUMNS: [BitBoard; 8] = [
    BitBoard::col_mask(Column::A),
    BitBoard::col_mask(Column::B),
    BitBoard::col_mask(Column::C),
    BitBoard::col_mask(Column::D),
    BitBoard::col_mask(Column::E),
    BitBoard::col_mask(Column::F),
    BitBoard::col_mask(Column::G),
    BitBoard::col_mask(Column::H),
];
static ROWS: [BitBoard; 8] = [
    BitBoard::row_mask(Row::One),
    BitBoard::row_mask(Row::Two),
    BitBoard::row_mask(Row::Three),
    BitBoard::row_mask(Row::Four),
    BitBoard::row_mask(Row::Five),
    BitBoard::row_mask(Row::Six),
    BitBoard::row_mask(Row::Seven),
    BitBoard::row_mask(Row::Eight),
];
static SQUARES: [BitBoard; 64] = gen_table!(BitBoard::square_mask);
static KING_MOVES: [BitBoard; 64] = gen_table!(BitBoard::king_move_mask);
static KNIGHT_MOVES: [BitBoard; 64] = gen_table!(BitBoard::king_move_mask);
static WHITE_PAWN_ATTACKS: [BitBoard; 64] = gen_table!(BitBoard::pawn_attack_mask, Color::White);
static BLACK_PAWN_ATTACKS: [BitBoard; 64] = gen_table!(BitBoard::pawn_attack_mask, Color::Black);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct BitBoard(u64);

impl BitBoard {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn king_moves(square: Square) -> Self {
        KING_MOVES[square]
    }

    pub fn knight_moves(square: Square) -> Self {
        KNIGHT_MOVES[square]
    }

    pub fn pawn_attacks(square: Square, color: Color) -> Self {
        match color {
            Color::White => WHITE_PAWN_ATTACKS[square],
            Color::Black => BLACK_PAWN_ATTACKS[square],
        }
    }

    pub const fn bitscan_forward(&self) -> Option<Square> {
        match self.0.trailing_zeros() {
            64 => None,
            // Safety: x < 64
            x => unsafe { Some(Square::from_u8_unchecked(x as u8)) },
        }
    }

    pub fn count_squares(&self) -> u8 {
        self.0.count_ones() as u8
    }

    pub fn empty(&self) -> bool {
        self.0 == 0
    }

    const fn col_mask(c: Column) -> Self {
        Self::new(0x0101010101010101 << c as u8)
    }

    const fn row_mask(r: Row) -> Self {
        Self::new(0xff << (8 * r as u8))
    }

    const fn square_mask(s: Square) -> Self {
        Self::new(1 << s as u8)
    }

    const fn king_move_mask(square: Square) -> Self {
        let square_mask = Self::square_mask(square).0;
        let lateral_mask = ((square_mask << 1) & NOT_COL_A) | ((square_mask >> 1) & NOT_COL_H);
        let screen_mask = lateral_mask | square_mask;
        Self(lateral_mask | (screen_mask << 8) | (screen_mask >> 8))
    }

    const fn pawn_attack_mask(square: Square, color: Color) -> Self {
        let square_mask = Self::square_mask(square).0;
        let (left, right) = match color {
            Color::White => (square_mask << 7, square_mask << 9),
            Color::Black => (square_mask >> 9, square_mask >> 7),
        };
        Self((left & NOT_COL_H) | (right & NOT_COL_A))
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
