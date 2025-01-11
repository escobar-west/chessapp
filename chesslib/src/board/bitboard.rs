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

const fn shift_bb(bitboard: u64, cols: i8, rows: i8) -> u64 {
    if rows.abs() >= 8 {
        return 0;
    }
    let col_mask = if cols.is_positive() {
        0xffu8 >> cols
    } else {
        0xffu8 << -cols
    };
    let masked_bitboard = bitboard & (col_mask as u64 * 0x0101010101010101);
    let shift = 8 * rows + cols;
    if shift.is_positive() {
        masked_bitboard << shift
    } else {
        masked_bitboard >> -shift
    }
}

static COLUMNS: [BitBoard; 8] = [
    BitBoard::from_col(Column::A),
    BitBoard::from_col(Column::B),
    BitBoard::from_col(Column::C),
    BitBoard::from_col(Column::D),
    BitBoard::from_col(Column::E),
    BitBoard::from_col(Column::F),
    BitBoard::from_col(Column::G),
    BitBoard::from_col(Column::H),
];
static ROWS: [BitBoard; 8] = [
    BitBoard::from_row(Row::One),
    BitBoard::from_row(Row::Two),
    BitBoard::from_row(Row::Three),
    BitBoard::from_row(Row::Four),
    BitBoard::from_row(Row::Five),
    BitBoard::from_row(Row::Six),
    BitBoard::from_row(Row::Seven),
    BitBoard::from_row(Row::Eight),
];
static SQUARES: [BitBoard; 64] = gen_table!(BitBoard::from_square);
static KING_MOVES: [BitBoard; 64] = gen_table!(BitBoard::king_move_mask);
static KNIGHT_MOVES: [BitBoard; 64] = gen_table!(BitBoard::knight_move_mask);
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

    pub fn count_squares(&self) -> u8 {
        self.0.count_ones() as u8
    }

    pub fn empty(&self) -> bool {
        self.0 == 0
    }

    pub fn iter(self) -> impl Iterator<Item = Square> {
        BitBoardIter { rem_board: self }
    }

    pub fn print_board(self, c: char) {
        let mut char_board: [char; 64] = ['☐'; 64];
        for square in self.iter() {
            char_board[square] = c;
        }
        let mut out_str = String::new();
        for i in (0..8).rev() {
            let offset = 8 * i as usize;
            let row: String = char_board[offset..offset + 8].iter().collect();
            out_str.push_str(&row);
            out_str.push('\n')
        }
        println!("{}", out_str);
    }

    const fn from_col(c: Column) -> Self {
        Self(0x0101010101010101 << c as u8)
    }

    const fn from_row(r: Row) -> Self {
        Self(0xff << (8 * r as u8))
    }

    const fn from_square(s: Square) -> Self {
        Self(1 << s as u8)
    }

    const fn king_move_mask(square: Square) -> Self {
        let square_mask = Self::from_square(square).0;
        let lateral_mask = shift_bb(square_mask, -1, 0) | shift_bb(square_mask, 1, 0);
        let screen_mask = lateral_mask | square_mask;
        Self(lateral_mask | shift_bb(screen_mask, 0, 1) | shift_bb(screen_mask, 0, -1))
    }

    const fn knight_move_mask(square: Square) -> Self {
        let square_mask = Self::from_square(square).0;
        Self(
            shift_bb(square_mask, 2, 1)
                | shift_bb(square_mask, 2, -1)
                | shift_bb(square_mask, -2, 1)
                | shift_bb(square_mask, -2, -1)
                | shift_bb(square_mask, 1, 2)
                | shift_bb(square_mask, 1, -2)
                | shift_bb(square_mask, -1, 2)
                | shift_bb(square_mask, -1, -2),
        )
    }

    const fn pawn_attack_mask(square: Square, color: Color) -> Self {
        let square_mask = Self::from_square(square).0;
        let row_shift = match color {
            Color::White => 1,
            Color::Black => -1,
        };
        let left = shift_bb(square_mask, -1, row_shift);
        let right = shift_bb(square_mask, 1, row_shift);
        Self(left | right)
    }

    const fn bitscan_forward(&self) -> Option<Square> {
        match self.0.trailing_zeros() {
            64 => None,
            // Safety: x < 64
            x => unsafe { Some(Square::from_u8_unchecked(x as u8)) },
        }
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

struct BitBoardIter {
    rem_board: BitBoard,
}

impl Iterator for BitBoardIter {
    type Item = Square;
    fn next(&mut self) -> Option<Self::Item> {
        self.rem_board.bitscan_forward().inspect(|&lsb| {
            self.rem_board ^= BitBoard::from(lsb);
        })
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
    fn test_knight_moves() {
        let knight_moves = BitBoard::knight_moves(Square::A1);
        let expected = BitBoard::from(Square::B3) | Square::C2.into();
        assert_eq!(knight_moves, expected);

        let knight_moves = BitBoard::knight_moves(Square::F6);
        let expected = BitBoard::from(Square::D5)
            | Square::D7.into()
            | Square::E4.into()
            | Square::E8.into()
            | Square::G4.into()
            | Square::G8.into()
            | Square::H5.into()
            | Square::H7.into();
        assert_eq!(knight_moves, expected);
    }

    #[test]
    fn test_pawn_attacks() {
        let pawn_attacks = BitBoard::pawn_attacks(Square::A1, Color::White);
        let expected = BitBoard::from(Square::B2);
        assert_eq!(pawn_attacks, expected);

        let pawn_attacks = BitBoard::pawn_attacks(Square::H2, Color::White);
        let expected = BitBoard::from(Square::G3);
        assert_eq!(pawn_attacks, expected);

        let pawn_attacks = BitBoard::pawn_attacks(Square::B8, Color::Black);
        let expected = BitBoard::from(Square::A7) | Square::C7.into();
        assert_eq!(pawn_attacks, expected);
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
