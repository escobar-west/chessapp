mod bitboard;
mod mailbox;
use std::ops::{Index, IndexMut};

use crate::pieces::{Piece, constants::*};
use bitboard::BitBoard;
use errors::InvalidFen;
use mailbox::MailBox;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum Row {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Row {
    const fn from_u8(val: u8) -> Self {
        assert!(val < 8);
        // Safety: code will panic if val > 7
        unsafe { std::mem::transmute(val) }
    }

    const fn as_bitboard(self) -> BitBoard {
        BitBoard::new(0xff << (8 * self as u8))
    }
}

impl<T> Index<Row> for [T] {
    type Output = T;

    fn index(&self, index: Row) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> IndexMut<Row> for [T] {
    fn index_mut(&mut self, index: Row) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum Column {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl Column {
    const fn from_u8(val: u8) -> Self {
        assert!(val < 8);
        // Safety: code will panic if val > 7
        unsafe { std::mem::transmute(val) }
    }

    const fn as_bitboard(self) -> BitBoard {
        BitBoard::new(0x0101010101010101 << self as u8)
    }
}

impl<T> Index<Column> for [T] {
    type Output = T;

    fn index(&self, index: Column) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> IndexMut<Column> for [T] {
    fn index_mut(&mut self, index: Column) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[rustfmt::skip]
#[repr(u8)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    const fn from_u8(val: u8) -> Self {
        assert!(val < 64);
        // Safety: code will panic if val > 63
        unsafe { std::mem::transmute(val) }
    }

    const fn from_coords(row: Row, col: Column) -> Self {
        // Safety: Row and Col both only range from 0 to 7
        unsafe { std::mem::transmute(8 * row as u8 + col as u8) }
    }

    const fn as_bitboard(self) -> BitBoard {
        BitBoard::new(1 << self as u8)
    }
}

impl<T> Index<Square> for [T] {
    type Output = T;

    fn index(&self, index: Square) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> IndexMut<Square> for [T] {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
struct PieceSet {
    pawns: BitBoard,
    rooks: BitBoard,
    knights: BitBoard,
    bishops: BitBoard,
    queens: BitBoard,
    kings: BitBoard,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Board {
    white_pieces: PieceSet,
    black_pieces: PieceSet,
    mailbox: MailBox,
}

impl Default for Board {
    fn default() -> Self {
        Self::try_from_fen(crate::constants::DEFAULT_FEN).unwrap()
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            white_pieces: PieceSet::default(),
            black_pieces: PieceSet::default(),
            mailbox: MailBox::default(),
        }
    }

    pub fn try_from_fen(fen: &str) -> Result<Self, InvalidFen> {
        let piece_data = fen.split(' ').next().ok_or(InvalidFen::EmptyFen)?;
        let row_data = piece_data.split('/');
        if row_data.clone().count() != 8 {
            return Err(InvalidFen::WrongRowCount);
        }
        let mut board = Self::new();
        for (row_idx, row) in (0..8).rev().zip(row_data) {
            let mut col_idx = 0;
            for c in row.chars() {
                if c.is_ascii_digit() {
                    col_idx += c.to_digit(10).unwrap() as u8;
                } else {
                    let piece = Piece::try_from(c)?;
                    let square =
                        Square::from_coords(Row::from_u8(row_idx), Column::from_u8(col_idx));
                    board.set_sq(square, piece);
                    col_idx += 1;
                }
            }
        }
        Ok(board)
    }

    pub fn set_sq(&mut self, square: Square, piece: Piece) -> Option<Piece> {
        let old_piece = self.mailbox.set_sq(square, piece);
        if let Some(old_piece) = old_piece {
            self.clear_piece_board(old_piece, square.into())
        }
        self.set_piece_board(piece, square.into());
        old_piece
    }

    fn clear_piece_board(&mut self, piece: Piece, mask: BitBoard) {
        *self.get_piece_board_mut(piece) &= !mask;
    }

    fn set_piece_board(&mut self, piece: Piece, mask: BitBoard) {
        *self.get_piece_board_mut(piece) |= mask;
    }

    fn get_piece_board(&self, piece: Piece) -> BitBoard {
        match piece {
            WHITE_PAWN => self.white_pieces.pawns,
            WHITE_ROOK => self.white_pieces.rooks,
            WHITE_KNIGHT => self.white_pieces.knights,
            WHITE_BISHOP => self.white_pieces.bishops,
            WHITE_QUEEN => self.white_pieces.queens,
            WHITE_KING => self.white_pieces.kings,
            BLACK_PAWN => self.black_pieces.pawns,
            BLACK_ROOK => self.black_pieces.rooks,
            BLACK_KNIGHT => self.black_pieces.knights,
            BLACK_BISHOP => self.black_pieces.bishops,
            BLACK_QUEEN => self.black_pieces.queens,
            BLACK_KING => self.black_pieces.kings,
        }
    }

    fn get_piece_board_mut(&mut self, piece: Piece) -> &mut BitBoard {
        match piece {
            WHITE_PAWN => &mut self.white_pieces.pawns,
            WHITE_ROOK => &mut self.white_pieces.rooks,
            WHITE_KNIGHT => &mut self.white_pieces.knights,
            WHITE_BISHOP => &mut self.white_pieces.bishops,
            WHITE_QUEEN => &mut self.white_pieces.queens,
            WHITE_KING => &mut self.white_pieces.kings,
            BLACK_PAWN => &mut self.black_pieces.pawns,
            BLACK_ROOK => &mut self.black_pieces.rooks,
            BLACK_KNIGHT => &mut self.black_pieces.knights,
            BLACK_BISHOP => &mut self.black_pieces.bishops,
            BLACK_QUEEN => &mut self.black_pieces.queens,
            BLACK_KING => &mut self.black_pieces.kings,
        }
    }
}

pub mod errors {
    use crate::pieces::errors::InvalidChar;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum InvalidFen {
        #[error("Empty FEN")]
        EmptyFen,
        #[error("Wrong number of rows")]
        WrongRowCount,
        #[error(transparent)]
        InvalidChar(#[from] InvalidChar),
    }
}

#[cfg(test)]
mod tests;
