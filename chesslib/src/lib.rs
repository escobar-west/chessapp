#![allow(dead_code)]
#![feature(let_chains)]
pub mod board;
pub mod errors;
pub mod pieces;

use board::Board;
use board::Row;
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

    pub fn get_turn(&self) -> Color {
        self.turn
    }

    pub fn get_sq(&self, square: Square) -> Option<Piece> {
        self.board.get_sq(square)
    }

    pub fn make_promotion(
        &mut self,
        from: Square,
        to: Square,
        promotion_piece: Piece,
    ) -> Result<Option<Piece>, MoveError> {
        let Some(piece) = self.board.get_sq(from) else {
            return Err(MoveError::EmptySquare);
        };
        if piece.color != self.turn {
            return Err(MoveError::WrongTurn);
        }
        if !self.board.is_pseudolegal(piece, from, to, self.turn) {
            return Err(MoveError::IllegalMove);
        }
        let captured = self.board.move_piece(from, to);
        if self.board.is_in_check(self.turn) {
            self.board.reverse_move_piece(from, to, captured);
            return Err(MoveError::KingInCheck);
        }
        self.board.set_sq(to, promotion_piece);
        if self.turn == Color::Black {
            self.full_move += 1;
        }
        self.turn = !self.turn;
        Ok(captured)
    }

    pub fn make_move(&mut self, from: Square, to: Square) -> Result<Option<Piece>, MoveError> {
        let Some(piece) = self.board.get_sq(from) else {
            return Err(MoveError::EmptySquare);
        };
        if piece.color != self.turn {
            return Err(MoveError::WrongTurn);
        }
        if !self.board.is_pseudolegal(piece, from, to, self.turn) {
            return Err(MoveError::IllegalMove);
        }
        let captured = self.test_move(self.turn, from, to)?;
        if self.turn == Color::Black {
            self.full_move += 1;
        }
        self.turn = !self.turn;
        Ok(captured)
    }

    pub fn test_move(
        &mut self,
        turn: Color,
        from: Square,
        to: Square,
    ) -> Result<Option<Piece>, MoveError> {
        let captured = self.board.move_piece(from, to);
        if self.board.is_in_check(turn) {
            self.board.reverse_move_piece(from, to, captured);
            return Err(MoveError::KingInCheck);
        }
        let last_row = match turn {
            Color::White => Row::Eight,
            Color::Black => Row::One,
        };
        if to.row() == last_row
            && let Some(Piece {
                color: _,
                figure: pieces::Figure::Pawn,
            }) = self.board.get_sq(to)
        {
            self.board.reverse_move_piece(from, to, captured);
            return Err(MoveError::Promoting);
        }
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

pub mod constants {
    pub const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    pub const KINGS_ONLY: &str = "4k3/8/8/8/8/8/8/4K3 w - - 0 1";
    pub const KN: &str = "8/8/4k3/3N1n2/4K3/8/8/8 w - - 0 1";
    pub const KNP: &str = "nnnNNNk1/1P2P1P1/8/8/3p2p1/1Pp1p1p1/P1PP1P1P/2K5 w - - 0 1";
}
