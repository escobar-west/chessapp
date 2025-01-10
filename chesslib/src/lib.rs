#![allow(dead_code)]
pub mod board;
pub mod errors;
pub mod pieces;

use board::Board;
use board::Square;
use errors::{MoveError, ParseFenError};
use pieces::{
    Color, Piece,
    constants::{BLACK_KING, WHITE_KING},
};

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
    pub fn try_from_fen(fen: &str) -> Result<Self, ParseFenError> {
        let mut fen_iter = fen.split(' ');
        let position_fen = fen_iter.next().ok_or(ParseFenError::EmptyFen)?;
        let board = Board::try_from_fen(position_fen)?;
        let turn = match fen_iter.next().ok_or(ParseFenError::EmptyFen)? {
            "w" => Color::White,
            "b" => Color::Black,
            s => return Err(ParseFenError::InvalidColor(s.to_owned())),
        };
        let _castle_fen = fen_iter.next().ok_or(ParseFenError::EmptyFen)?;
        let _ep_fen = fen_iter.next().ok_or(ParseFenError::EmptyFen)?;
        let half_move = fen_iter.next().ok_or(ParseFenError::EmptyFen)?.parse()?;
        let full_move = fen_iter.next().ok_or(ParseFenError::EmptyFen)?.parse()?;
        if board.count_pieces(WHITE_KING) != 1 || board.count_pieces(BLACK_KING) != 1 {
            return Err(ParseFenError::IllegalState);
        }
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
        let captured = self.board.move_piece(from, to);
        if self.board.is_in_check(piece.color) {
            self.board.reverse_move_piece(from, to, captured);
            return Err(MoveError::KingInCheck);
        }
        if self.turn == Color::Black {
            self.full_move += 1;
        }
        self.turn = !self.turn;
        Ok(captured)
    }

    pub fn iter(&self) -> impl Iterator<Item = (Square, Piece)> {
        self.board.iter()
    }
}

pub mod prelude {
    pub use crate::{
        GameState,
        board::{Column, Row, Square},
        constants::*,
        pieces::{Color, Figure, Piece, constants::*},
    };
}

mod constants {
    pub const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    pub const KINGS_ONLY: &str = "4k3/8/8/8/8/8/8/4K3 w - - 0 1";
    pub const KINGS_PAWNS: &str = "4k3/ppppP3/8/8/8/8/PPPPp3/4K3 w - - 0 1";
}
