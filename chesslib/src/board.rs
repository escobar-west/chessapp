pub mod bitboard;
mod mailbox;
use std::{fmt::Display, iter::repeat, str::FromStr};

use crate::{
    errors::{InvalidCharError, InvalidValueError, ParseFenError},
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

    pub fn unmove_piece(&mut self, from: Square, to: Square, captured: Option<Piece>) {
        self.move_piece(to, from);
        if let Some(captured) = captured {
            self.set_sq(to, captured);
        }
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

        let enemy_pawn_mask = self.pawn_moves(square, turn);
        let enemy_pawn_location = self.get_piece_board(Piece {
            color: !turn,
            figure: Figure::Pawn,
        });
        if !(enemy_pawn_mask & enemy_pawn_location).empty() {
            return true;
        }

        for rook_sq in self.iter_piece(Piece {
            color: !turn,
            figure: Figure::Rook,
        }) {
            if self.is_pseudo::<{ Figure::Rook }>(rook_sq, square, !turn) {
                return true;
            }
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

    pub fn is_pseudo<const FIGURE: Figure>(&self, from: Square, to: Square, turn: Color) -> bool {
        use Figure::*;
        match FIGURE {
            Knight => (BitBoard::knight_moves(from) & !self.occupied_color(turn)).contains(to),
            Rook => {
                let is_cleared = BitBoard::straight_ray(from, to) & self.occupied == from.into();
                is_cleared && !self.occupied_color(turn).contains(to)
            }
            Bishop => {
                let is_cleared = BitBoard::diag_ray(from, to) & self.occupied == from.into();
                is_cleared && !self.occupied_color(turn).contains(to)
            }
            Queen => {
                let is_cleared = (BitBoard::straight_ray(from, to) | BitBoard::diag_ray(from, to))
                    & self.occupied
                    == from.into();
                is_cleared && !self.occupied_color(turn).contains(to)
            }
            _ => todo!(),
        }
    }

    pub fn pawn_moves(&self, from: Square, turn: Color) -> BitBoard {
        let attacks = BitBoard::pawn_attacks(from, turn) & self.occupied_color(!turn);
        let moves = match turn {
            Color::White => {
                let mut moves = BitBoard::from(from).shift::<0, 1>() & !self.occupied;
                moves |= moves.shift::<0, 1>() & Row::Four.into() & !self.occupied;
                moves
            }
            Color::Black => {
                let mut moves = BitBoard::from(from).shift::<0, -1>() & !self.occupied;
                moves |= moves.shift::<0, -1>() & Row::Five.into() & !self.occupied;
                moves
            }
        };
        attacks | moves
    }

    fn clear_piece_board(&mut self, piece: Piece, mask: BitBoard) {
        let should_keep = !mask;
        *self.get_piece_board_mut(piece) &= should_keep;
        *self.occupied_color_mut(piece.color) &= should_keep;
        self.occupied &= should_keep;
    }

    fn set_piece_board(&mut self, piece: Piece, mask: BitBoard) {
        *self.get_piece_board_mut(piece) |= mask;
        *self.occupied_color_mut(piece.color) |= mask;
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

    pub fn occupied_color(&self, color: Color) -> BitBoard {
        match color {
            Color::White => self.white_pieces.occupied,
            Color::Black => self.black_pieces.occupied,
        }
    }

    fn occupied_color_mut(&mut self, color: Color) -> &mut BitBoard {
        match color {
            Color::White => &mut self.white_pieces.occupied,
            Color::Black => &mut self.black_pieces.occupied,
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut char_board: [char; 64] = ['☐'; 64];
        for (square, piece) in self.iter() {
            let c: char = piece.into();
            char_board[square] = c;
        }
        let mut out_str = String::new();
        for i in (0..8).rev() {
            let offset = 8 * i as usize;
            let row: String = char_board[offset..offset + 8].iter().collect();
            out_str.push_str(&row);
            out_str.push('\n')
        }
        write!(f, "{out_str}")
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

impl TryFrom<char> for Column {
    type Error = InvalidCharError;

    fn try_from(val: char) -> Result<Self, Self::Error> {
        let int_repr = u8::try_from(u32::from(val) - 97).map_err(|_| InvalidCharError(val))?;
        Self::try_from(int_repr).map_err(|_| InvalidCharError(val))
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

impl TryFrom<char> for Row {
    type Error = InvalidCharError;

    fn try_from(val: char) -> Result<Self, Self::Error> {
        let int_repr = u8::try_from(u32::from(val) - 49).map_err(|_| InvalidCharError(val))?;
        Self::try_from(int_repr).map_err(|_| InvalidCharError(val))
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

    pub const fn shift<const COLS: i8, const ROWS: i8>(self) -> Option<Self> {
        let (new_col, new_row) = (self.col() as i8 + COLS, self.row() as i8 + ROWS);
        if 0 <= new_col && new_col < 8 && 0 <= new_row && new_row < 8 {
            unsafe { Some(Self::from_u8_unchecked(8 * new_row as u8 + new_col as u8)) }
        } else {
            None
        }
    }

    const unsafe fn from_u8_unchecked(val: u8) -> Self {
        // Safety: val must be < 64
        unsafe { std::mem::transmute::<u8, Self>(val) }
    }
}

impl FromStr for Square {
    type Err = ParseFenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() != 2 {
            return Err(ParseFenError::InvalidString(s.into()));
        }
        let col = chars[0]
            .try_into()
            .map_err(|_| ParseFenError::InvalidString(s.into()))?;
        let row = chars[1]
            .try_into()
            .map_err(|_| ParseFenError::InvalidString(s.into()))?;
        Ok(Self::from_coords(col, row))
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
