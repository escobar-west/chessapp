use super::{Column, Row, Square};
use crate::pieces::Color;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

macro_rules! gen_table {
    ($mask_fn:expr $(,$arg0:expr)*) => {{
        let mut array = [BitBoard(0); 64];
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

static KING_MOVES: [BitBoard; 64] = gen_table!(BitBoard::king_move_mask);
static KNIGHT_MOVES: [BitBoard; 64] = gen_table!(BitBoard::knight_move_mask);
static WHITE_PAWN_ATTACKS: [BitBoard; 64] = gen_table!(BitBoard::pawn_attack_mask, Color::White);
static BLACK_PAWN_ATTACKS: [BitBoard; 64] = gen_table!(BitBoard::pawn_attack_mask, Color::Black);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct BitBoard(u64);

impl BitBoard {
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

    const fn shift<const COLS: i8, const ROWS: i8>(self) -> Self {
        if ROWS.abs() >= 8 || COLS.abs() >= 8 {
            return Self(0);
        }
        let col_mask = if COLS.is_positive() {
            0xffu16 >> COLS
        } else {
            0xffu16 << -COLS
        } as u8;
        let masked_bitboard = self.0 & (col_mask as u64 * 0x0101010101010101);
        let shift = 8 * ROWS + COLS;
        if shift.is_positive() {
            Self(masked_bitboard << shift)
        } else {
            Self(masked_bitboard >> -shift)
        }
    }

    const fn king_move_mask(square: Square) -> Self {
        let square = Self::from_square(square);
        let lateral_mask = square.shift::<-1, 0>().or(square.shift::<1, 0>());
        let screen_mask = lateral_mask.or(square);
        lateral_mask
            .or(screen_mask.shift::<0, 1>())
            .or(screen_mask.shift::<0, -1>())
    }

    const fn knight_move_mask(square: Square) -> Self {
        let square = Self::from_square(square);
        square
            .shift::<-2, 1>()
            .or(square.shift::<-2, -1>())
            .or(square.shift::<-1, 2>())
            .or(square.shift::<-1, -2>())
            .or(square.shift::<1, 2>())
            .or(square.shift::<1, -2>())
            .or(square.shift::<2, 1>())
            .or(square.shift::<2, -1>())
    }

    const fn pawn_attack_mask(square: Square, color: Color) -> Self {
        let square = Self::from_square(square);
        let (left, right) = match color {
            Color::White => (square.shift::<-1, 1>(), square.shift::<1, 1>()),
            Color::Black => (square.shift::<-1, -1>(), square.shift::<1, -1>()),
        };
        left.or(right)
    }

    const fn bitscan_forward(&self) -> Option<Square> {
        match self.0.trailing_zeros() {
            64 => None,
            // Safety: x < 64
            x => unsafe { Some(Square::from_u8_unchecked(x as u8)) },
        }
    }

    const fn and(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }

    const fn or(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }

    const fn xor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }

    const fn not(self) -> Self {
        Self(!self.0)
    }

    const fn and_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }

    const fn or_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }

    const fn xor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl From<Square> for BitBoard {
    fn from(s: Square) -> Self {
        Self::from_square(s)
    }
}

impl From<Row> for BitBoard {
    fn from(r: Row) -> Self {
        Self::from_row(r)
    }
}

impl From<Column> for BitBoard {
    fn from(c: Column) -> Self {
        Self::from_col(c)
    }
}

impl BitAnd for BitBoard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

impl BitOr for BitBoard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}

impl BitXor for BitBoard {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.xor(rhs)
    }
}

impl Not for BitBoard {
    type Output = Self;
    fn not(self) -> Self::Output {
        self.not()
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.and_assign(rhs);
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.or_assign(rhs);
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.xor_assign(rhs);
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
    fn test_shift_a1() {
        let input = BitBoard::from(Square::A1);
        let output = input.shift::<1, 0>();
        assert_eq!(output, BitBoard::from(Square::B1));
        let output = input.shift::<0, 1>();
        assert_eq!(output, BitBoard::from(Square::A2));
        let output = input.shift::<1, 1>();
        assert_eq!(output, BitBoard::from(Square::B2));

        let output = input.shift::<-1, 0>();
        assert_eq!(output, BitBoard(0));
        let output = input.shift::<0, -1>();
        assert_eq!(output, BitBoard(0));
        let output = input.shift::<-1, -1>();
        assert_eq!(output, BitBoard(0));

        let output = input.shift::<7, 0>();
        assert_eq!(output, BitBoard::from(Square::H1));
        let output = input.shift::<0, 7>();
        assert_eq!(output, BitBoard::from(Square::A8));
        let output = input.shift::<7, 7>();
        assert_eq!(output, BitBoard::from(Square::H8));
    }

    #[test]
    fn test_shift_h8() {
        let input = BitBoard::from(Square::H8);
        let output = input.shift::<1, 0>();
        assert_eq!(output, BitBoard(0));
        let output = input.shift::<0, 1>();
        assert_eq!(output, BitBoard(0));
        let output = input.shift::<1, 1>();
        assert_eq!(output, BitBoard(0));

        let output = input.shift::<-1, 0>();
        assert_eq!(output, BitBoard::from(Square::G8));
        let output = input.shift::<0, -1>();
        assert_eq!(output, BitBoard::from(Square::H7));
        let output = input.shift::<-1, -1>();
        assert_eq!(output, BitBoard::from(Square::G7));

        let output = input.shift::<-7, 0>();
        assert_eq!(output, BitBoard::from(Square::A8));
        let output = input.shift::<0, -7>();
        assert_eq!(output, BitBoard::from(Square::H1));
        let output = input.shift::<-7, -7>();
        assert_eq!(output, BitBoard::from(Square::A1));
    }

    #[test]
    fn test_shift_e4() {
        let input = BitBoard::from(Square::E4);
        let output = input.shift::<-4, -3>();
        assert_eq!(output, BitBoard::from(Square::A1));
        let output = input.shift::<-4, 0>();
        assert_eq!(output, BitBoard::from(Square::A4));
        let output = input.shift::<-4, 4>();
        assert_eq!(output, BitBoard::from(Square::A8));
        let output = input.shift::<0, 4>();
        assert_eq!(output, BitBoard::from(Square::E8));
        let output = input.shift::<3, 4>();
        assert_eq!(output, BitBoard::from(Square::H8));
        let output = input.shift::<3, 0>();
        assert_eq!(output, BitBoard::from(Square::H4));
        let output = input.shift::<3, -3>();
        assert_eq!(output, BitBoard::from(Square::H1));
        let output = input.shift::<0, -3>();
        assert_eq!(output, BitBoard::from(Square::E1));

        let output = input.shift::<4, 0>();
        assert_eq!(output, BitBoard(0));
        let output = input.shift::<-5, 0>();
        assert_eq!(output, BitBoard(0));
        let output = input.shift::<0, 5>();
        assert_eq!(output, BitBoard(0));
        let output = input.shift::<0, -4>();
        assert_eq!(output, BitBoard(0));
    }

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

        let bitboard = BitBoard(std::u64::MAX);
        let lsb = bitboard.bitscan_forward();
        assert_eq!(lsb, Some(Square::A1));

        let bitboard = BitBoard(0);
        let lsb = bitboard.bitscan_forward();
        assert_eq!(lsb, None);
    }
}
