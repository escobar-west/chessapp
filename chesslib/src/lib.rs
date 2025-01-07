#![allow(dead_code)]
mod board;
mod pieces;

use board::Board;
pub use board::{Column, Row, Square};
pub use errors::{InvalidFen, MoveError};
pub use pieces::{Color, Piece};

#[derive(Debug, PartialEq, Eq)]
pub struct GameState {
    board: Board,
    turn: Color,
    half_move: u16,
    full_move: u16,
}

impl Default for GameState {
    fn default() -> Self {
        Self::try_from_fen(constants::DEFAULT_FEN).unwrap()
    }
}

impl GameState {
    pub fn try_from_fen(fen: &str) -> Result<Self, InvalidFen> {
        let mut fen_iter = fen.split(' ');
        let position_fen = fen_iter.next().ok_or(InvalidFen::EmptyFen)?;
        let board = Board::try_from_fen(position_fen)?;
        let turn = match fen_iter.next().ok_or(InvalidFen::EmptyFen)? {
            "w" => Color::White,
            "b" => Color::Black,
            s => return Err(InvalidFen::InvalidColor(s.to_owned())),
        };
        let _castle_fen = fen_iter.next().ok_or(InvalidFen::EmptyFen)?;
        let _ep_fen = fen_iter.next().ok_or(InvalidFen::EmptyFen)?;
        let half_move = fen_iter.next().ok_or(InvalidFen::EmptyFen)?.parse()?;
        let full_move = fen_iter.next().ok_or(InvalidFen::EmptyFen)?.parse()?;
        Ok(GameState {
            board,
            turn,
            half_move,
            full_move,
        })
    }

    pub fn get_sq(&self, square: Square) -> Option<Piece> {
        self.board.get_sq(square)
    }

    pub fn make_move(&mut self, from: Square, to: Square) -> Result<Option<Piece>, MoveError> {
        let Some(piece) = self.board.get_sq(from) else {
            return Err(MoveError::EmptySquare);
        };
        if piece.color != self.turn {
            return Err(MoveError::WrongTurn);
        }
        if !self.board.is_pseudolegal(piece, from, to) {
            return Err(MoveError::IllegalMove);
        }
        Ok(self.make_illegal_move(from, to))
    }

    fn make_illegal_move(&mut self, from: Square, to: Square) -> Option<Piece> {
        let piece = self.board.move_piece(from, to);
        if self.turn == Color::Black {
            self.full_move += 1;
        }
        self.turn = !self.turn;
        #[cfg(debug_assertions)]
        //println!("{self:#?}");
        println!("{:#?}", self.board.iter().collect::<Vec<_>>());
        piece
    }

    pub fn iter(&self) -> impl Iterator<Item = (Square, Piece)> {
        self.board.iter()
    }
}

pub mod constants {
    pub use crate::pieces::constants::*;
    pub const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    pub const KINGS_ONLY: &str = "4k3/8/8/8/8/8/8/4K3 w - - 0 1";
}

mod errors {
    use crate::board::errors::InvalidFen as InvalidBoardFen;
    use std::num::ParseIntError;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum MoveError {
        #[error("Empty square")]
        EmptySquare,
        #[error("Wrong turn")]
        WrongTurn,
        #[error("Illegal move")]
        IllegalMove,
    }

    #[derive(Error, Debug)]
    pub enum InvalidFen {
        #[error("Empty FEN entry")]
        EmptyFen,
        #[error("Invalid color: {0:#?}")]
        InvalidColor(String),
        #[error(transparent)]
        InvalidBoardFen(#[from] InvalidBoardFen),
        #[error(transparent)]
        ParseIntError(#[from] ParseIntError),
    }
}
