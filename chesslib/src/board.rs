mod bitboard;
mod mailbox;
use std::iter::repeat;

use crate::{
    errors::{InvalidValueError, ParseFenError},
    pieces::{Color, Figure, Piece, constants::*},
};
use bitboard::BitBoard;
use mailbox::MailBox;
use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Eq)]
pub struct Board {
    white_pieces: PieceSet,
    black_pieces: PieceSet,
    occupied: BitBoard,
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
            white_pieces: PieceSet::new(Color::White),
            black_pieces: PieceSet::new(Color::Black),
            occupied: BitBoard::default(),
            mailbox: MailBox::default(),
        }
    }

    pub fn get_sq(&self, square: Square) -> Option<Piece> {
        self.mailbox.get_sq(square)
    }

    pub fn clear_sq(&mut self, square: Square) -> Option<Piece> {
        self.mailbox
            .clear_sq(square)
            .inspect(|&p| self.clear_piece_board(p, square.into()))
    }

    pub fn set_sq(&mut self, square: Square, piece: Piece) -> Option<Piece> {
        let old_piece = self
            .mailbox
            .set_sq(square, piece)
            .inspect(|&p| self.clear_piece_board(p, square.into()));
        self.set_piece_board(piece, square.into());
        old_piece
    }

    pub fn move_piece(&mut self, from: Square, to: Square) -> Option<Piece> {
        self.clear_sq(from).and_then(|p| self.set_sq(to, p))
    }

    pub fn reverse_move_piece(&mut self, from: Square, to: Square, captured: Option<Piece>) {
        self.move_piece(to, from);
        if let Some(captured) = captured {
            self.set_sq(to, captured);
        }
    }

    pub fn is_pseudolegal(&self, piece: Piece, from: Square, to: Square) -> bool {
        let move_mask = self.get_move_mask(piece, from);
        move_mask & to.into() != BitBoard::default()
    }

    pub fn count_pieces(&self, piece: Piece) -> u8 {
        self.get_piece_board(piece).count_squares()
    }

    pub fn is_in_check(&self, turn: Color) -> bool {
        let king = Piece {
            color: turn,
            figure: Figure::King,
        };
        self.iter_piece(king)
            .any(|s| self.is_square_attacked(s, turn))
    }

    pub fn is_square_attacked(&self, square: Square, turn: Color) -> bool {
        let enemy_king_mask = BitBoard::king_moves(square);
        let enemy_king_location = self.get_piece_board(Piece {
            color: !turn,
            figure: Figure::King,
        });
        if !(enemy_king_mask & enemy_king_location).empty() {
            return true;
        }
        let enemy_knight_mask = BitBoard::knight_moves(square);
        let enemy_knight_location = self.get_piece_board(Piece {
            color: !turn,
            figure: Figure::Knight,
        });
        if !(enemy_knight_mask & enemy_knight_location).empty() {
            return true;
        }
        let enemy_pawn_mask = BitBoard::king_moves(square);
        let enemy_pawn_location = self.get_piece_board(Piece {
            color: !turn,
            figure: Figure::Pawn,
        });
        if !(enemy_pawn_mask & enemy_pawn_location).empty() {
            return true;
        }
        false
    }

    pub fn iter(&self) -> impl Iterator<Item = (Square, Piece)> {
        self.white_pieces.iter().chain(self.black_pieces.iter())
    }

    pub fn iter_piece(&self, piece: Piece) -> impl Iterator<Item = Square> {
        self.get_piece_board(piece).iter()
    }

    pub fn try_from_fen(fen: &str) -> Result<Self, ParseFenError> {
        let piece_data = fen.split(' ').next().ok_or(ParseFenError::EmptyFen)?;
        let row_data = piece_data.split('/');
        if row_data.clone().count() != 8 {
            return Err(ParseFenError::WrongRowCount);
        }
        let mut board = Self::new();
        for (row_idx, row) in (0..8).rev().zip(row_data) {
            let mut col_idx = 0;
            for c in row.chars() {
                if c.is_ascii_digit() {
                    col_idx += c.to_digit(10).unwrap() as u8;
                } else {
                    let piece = Piece::try_from(c)?;
                    let square = Square::from_coords(col_idx.try_into()?, row_idx.try_into()?);
                    board.set_sq(square, piece);
                    col_idx += 1;
                }
            }
        }
        Ok(board)
    }

    fn get_move_mask(&self, piece: Piece, from: Square) -> BitBoard {
        let move_mask = match piece.figure {
            Figure::King => BitBoard::king_moves(from),
            Figure::Knight => BitBoard::knight_moves(from),
            Figure::Pawn => BitBoard::king_moves(from),
            _ => BitBoard::default(),
        };
        move_mask & !self.occupied(piece.color)
    }

    fn clear_piece_board(&mut self, piece: Piece, mask: BitBoard) {
        let should_keep = !mask;
        *self.get_piece_board_mut(piece) &= should_keep;
        *self.occupied_mut(piece.color) &= should_keep;
        self.occupied &= should_keep;
    }

    fn set_piece_board(&mut self, piece: Piece, mask: BitBoard) {
        *self.get_piece_board_mut(piece) |= mask;
        *self.occupied_mut(piece.color) |= mask;
        self.occupied |= mask;
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

    fn occupied(&self, color: Color) -> BitBoard {
        match color {
            Color::White => self.white_pieces.occupied,
            Color::Black => self.black_pieces.occupied,
        }
    }

    fn occupied_mut(&mut self, color: Color) -> &mut BitBoard {
        match color {
            Color::White => &mut self.white_pieces.occupied,
            Color::Black => &mut self.black_pieces.occupied,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Column {
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
    const unsafe fn from_u8_unchecked(val: u8) -> Self {
        // Safety: val must be < 8
        unsafe { std::mem::transmute::<u8, Self>(val) }
    }
}

impl TryFrom<u8> for Column {
    type Error = InvalidValueError;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Column::A),
            1 => Ok(Column::B),
            2 => Ok(Column::C),
            3 => Ok(Column::D),
            4 => Ok(Column::E),
            5 => Ok(Column::F),
            6 => Ok(Column::G),
            7 => Ok(Column::H),
            v => Err(InvalidValueError(v)),
        }
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
#[repr(u8)]
pub enum Row {
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
    const unsafe fn from_u8_unchecked(val: u8) -> Self {
        // Safety: val must be < 8
        unsafe { std::mem::transmute::<u8, Self>(val) }
    }
}

impl TryFrom<u8> for Row {
    type Error = InvalidValueError;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Row::One),
            1 => Ok(Row::Two),
            2 => Ok(Row::Three),
            3 => Ok(Row::Four),
            4 => Ok(Row::Five),
            5 => Ok(Row::Six),
            6 => Ok(Row::Seven),
            7 => Ok(Row::Eight),
            v => Err(InvalidValueError(v)),
        }
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

#[rustfmt::skip]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    pub const fn from_coords(col: Column, row: Row) -> Self {
        // Safety: 8 * row + col < 64
        unsafe { Self::from_u8_unchecked(8 * row as u8 + col as u8) }
    }

    pub const fn col(self) -> Column {
        // Safety: self & 7 < 8
        unsafe { Column::from_u8_unchecked(self as u8 & 7) }
    }

    pub const fn row(self) -> Row {
        // Safety: self >> 3 < 8
        unsafe { Row::from_u8_unchecked(self as u8 >> 3) }
    }

    const unsafe fn from_u8_unchecked(val: u8) -> Self {
        // Safety: val must be < 64
        unsafe { std::mem::transmute::<u8, Self>(val) }
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

#[derive(Debug, PartialEq, Eq)]
struct PieceSet {
    color: Color,
    pawns: BitBoard,
    rooks: BitBoard,
    knights: BitBoard,
    bishops: BitBoard,
    queens: BitBoard,
    kings: BitBoard,
    occupied: BitBoard,
}

impl PieceSet {
    fn new(color: Color) -> Self {
        Self {
            color,
            pawns: BitBoard::default(),
            rooks: BitBoard::default(),
            knights: BitBoard::default(),
            bishops: BitBoard::default(),
            queens: BitBoard::default(),
            kings: BitBoard::default(),
            occupied: BitBoard::default(),
        }
    }

    fn iter(&self) -> impl Iterator<Item = (Square, Piece)> {
        let color = self.color;
        self.pawns
            .iter()
            .zip(repeat(Piece {
                color,
                figure: Figure::Pawn,
            }))
            .chain(self.rooks.iter().zip(repeat(Piece {
                color,
                figure: Figure::Rook,
            })))
            .chain(self.knights.iter().zip(repeat(Piece {
                color,
                figure: Figure::Knight,
            })))
            .chain(self.bishops.iter().zip(repeat(Piece {
                color,
                figure: Figure::Bishop,
            })))
            .chain(self.queens.iter().zip(repeat(Piece {
                color,
                figure: Figure::Queen,
            })))
            .chain(self.kings.iter().zip(repeat(Piece {
                color,
                figure: Figure::King,
            })))
    }
}

#[cfg(test)]
mod tests;
