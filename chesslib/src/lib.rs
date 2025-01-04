#![allow(dead_code)]
mod board;
mod pieces;

use board::Board;
pub use board::{Column, Row, Square};
use errors::InvalidFen;
pub use pieces::{Color, Piece};

pub struct GameState {
    board: Board,
    turn: Color,
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
        let turn = match fen_iter.next() {
            Some("w") => Color::White,
            Some("b") => Color::Black,
            s => return Err(InvalidFen::InvalidColor(s.map(String::from))),
        };
        Ok(GameState { board, turn })
    }

    pub fn get_sq(&self, square: Square) -> Option<Piece> {
        self.board.get_sq(square)
    }

    pub fn make_illegal_move(&mut self, from: Square, to: Square) -> Option<Piece> {
        self.board.move_piece(from, to)
    }

    pub fn iter(&self) -> impl Iterator<Item = (Square, Piece)> {
        self.board.iter()
    }
}

pub mod constants {
    pub use crate::pieces::constants::*;
    pub const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
}

mod errors {
    use crate::board::errors::InvalidFen as InvalidBoardFen;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum InvalidFen {
        #[error("Empty FEN")]
        EmptyFen,
        #[error("Invalid color: {0:#?}")]
        InvalidColor(Option<String>),
        #[error(transparent)]
        InvalidBoardFen(#[from] InvalidBoardFen),
    }
}
