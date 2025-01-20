#![feature(let_chains)]
#![feature(adt_const_params)]
pub mod board;
mod castle;
pub mod errors;
pub mod pieces;

use core::panic;
use std::fmt::Display;

use board::Column;
use board::Row;
use board::Square;
use board::{Board, bitboard::BitBoard};
use castle::Castle;
use errors::{MoveError, ParseFenError};
use pieces::{
    Color, Figure, Piece,
    constants::{BLACK_KING, WHITE_KING},
};

type MoveResult = Result<Option<Piece>, MoveError>;

#[derive(Debug, PartialEq, Eq)]
pub struct GameState {
    board: Board,
    turn: Color,
    castle: Castle,
    ep_square: Option<Square>,
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
        let castle = fen_iter.next().ok_or(ParseFenError::EmptyFen)?.parse()?;
        let ep_square = match fen_iter.next().ok_or(ParseFenError::EmptyFen)? {
            "-" => Result::<_, ParseFenError>::Ok(None),
            ep => Ok(Some(ep.parse()?)),
        }?;
        let half_move = fen_iter.next().ok_or(ParseFenError::EmptyFen)?.parse()?;
        let full_move = fen_iter.next().ok_or(ParseFenError::EmptyFen)?.parse()?;
        if board.count_pieces(WHITE_KING) != 1 || board.count_pieces(BLACK_KING) != 1 {
            return Err(ParseFenError::IllegalState);
        }
        Ok(GameState {
            board,
            turn,
            castle,
            ep_square,
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
    ) -> MoveResult {
        let Some(Piece {
            color,
            figure: Figure::Pawn,
        }) = self.board.get_sq(from)
        else {
            return Err(MoveError::IllegalMove);
        };
        if color != self.turn {
            return Err(MoveError::WrongTurn);
        }
        let moves = self.board.pawn_moves(from, self.turn);
        let captured = if moves.contains(to) {
            self.test_move_for_check(from, to)
        } else {
            Err(MoveError::IllegalMove)
        }?;
        self.board.set_sq(to, promotion_piece);
        // half move
        self.half_move = 0;
        // ep square
        self.ep_square = None;
        self.end_move(to);
        Ok(captured)
    }

    pub fn make_move(&mut self, from: Square, to: Square) -> MoveResult {
        let Some(Piece { color, figure }) = self.board.get_sq(from) else {
            return Err(MoveError::EmptySquare);
        };
        if color != self.turn {
            return Err(MoveError::WrongTurn);
        }
        let captured = match figure {
            Figure::Pawn => self.make_pawn_move(from, to)?,
            Figure::King => self.make_king_move(from, to)?,
            Figure::Knight => self.make_generic_move::<{ Figure::Knight }>(from, to)?,
            Figure::Rook => self.make_generic_move::<{ Figure::Rook }>(from, to)?,
            Figure::Bishop => self.make_generic_move::<{ Figure::Bishop }>(from, to)?,
            Figure::Queen => self.make_generic_move::<{ Figure::Queen }>(from, to)?,
        };
        self.end_move(to);
        Ok(captured)
    }

    fn end_move(&mut self, to_square: Square) {
        // opp castle
        let (opp_q_rook, opp_k_rook) = match self.turn {
            Color::White => (Square::A8, Square::H8),
            Color::Black => (Square::A1, Square::H1),
        };
        match to_square {
            sq if sq == opp_k_rook => self.castle.remove_king_castle(!self.turn),
            sq if sq == opp_q_rook => self.castle.remove_queen_castle(!self.turn),
            _ => {}
        }
        // full move
        if self.turn == Color::Black {
            self.full_move += 1;
        }
        // turn
        self.turn = !self.turn;
    }

    fn make_pawn_move(&mut self, from: Square, to: Square) -> MoveResult {
        let non_ep_moves = self.board.pawn_moves(from, self.turn);
        let captured = if non_ep_moves.contains(to) {
            let captured = self.test_move_for_check(from, to)?;
            let last_row = match self.turn {
                Color::White => Row::Eight,
                Color::Black => Row::One,
            };
            if to.row() == last_row {
                self.board.unmove_piece(from, to, captured);
                return Err(MoveError::Promoting);
            }
            Ok(captured)
        } else if let Some(ep) = self.ep_square
            && to == ep
            && BitBoard::pawn_attacks(from, self.turn).contains(to)
        {
            // make ep move
            let capture_sq = Square::from_coords(to.col(), from.row());
            self.board.move_piece(from, to);
            let capture_pawn = self.board.clear_sq(capture_sq);
            if self.board.is_in_check(self.turn) {
                self.board.move_piece(to, from);
                capture_pawn.map(|p| self.board.set_sq(capture_sq, p));
                return Err(MoveError::KingInCheck);
            }
            Ok(capture_pawn)
        } else {
            Err(MoveError::IllegalMove)
        }?;
        // ep
        let (start_row, ep_row, end_row) = match self.turn {
            Color::White => (Row::Two, Row::Three, Row::Four),
            Color::Black => (Row::Seven, Row::Six, Row::Five),
        };
        if (from.row(), to.row()) == (start_row, end_row) {
            self.ep_square = Some(Square::from_coords(to.col(), ep_row));
        } else {
            self.ep_square = None;
        }
        // half move
        self.half_move = 0;
        Ok(captured)
    }

    fn make_king_move(&mut self, from: Square, to: Square) -> MoveResult {
        let non_castle_moves = BitBoard::king_moves(from) & !self.board.occupied_color(self.turn);
        let captured = if non_castle_moves.contains(to) {
            self.test_move_for_check(from, to)
        } else {
            // make castle move
            let castle_row = match self.turn {
                Color::White => Row::One,
                Color::Black => Row::Eight,
            };
            match to.col() {
                Column::C if to.row() == castle_row => {
                    if self.castle.can_queen_castle(self.turn) {
                        let rook_from = Square::from_coords(Column::A, castle_row);
                        let rook_to = Square::from_coords(Column::D, castle_row);
                        for square in BitBoard::straight_ray(from, rook_from).iter() {
                            if self.board.is_square_attacked(square, self.turn) {
                                return Err(MoveError::KingInCheck);
                            }
                        }
                        self.board.move_piece(from, to);
                        self.board.move_piece(rook_from, rook_to);
                        Ok(None)
                    } else {
                        Err(MoveError::IllegalMove)
                    }
                }
                Column::G if to.row() == castle_row => {
                    if self.castle.can_king_castle(self.turn) {
                        let rook_from = Square::from_coords(Column::H, castle_row);
                        let rook_to = Square::from_coords(Column::F, castle_row);
                        for square in BitBoard::straight_ray(from, rook_from).iter() {
                            if self.board.is_square_attacked(square, self.turn) {
                                return Err(MoveError::KingInCheck);
                            }
                        }
                        self.board.move_piece(from, to);
                        self.board.move_piece(rook_from, rook_to);
                        Ok(None)
                    } else {
                        Err(MoveError::IllegalMove)
                    }
                }
                _ => Err(MoveError::IllegalMove),
            }
        }?;
        // own castle
        self.castle.remove_castle(self.turn);
        // ep
        self.ep_square = None;
        // half move
        match captured {
            Some(_piece) => self.half_move = 0,
            None => self.half_move += 1,
        }
        Ok(captured)
    }

    fn make_generic_move<const FIGURE: Figure>(&mut self, from: Square, to: Square) -> MoveResult {
        use Figure::*;
        let is_pseudo = match FIGURE {
            Knight => self.board.is_pseudo::<{ Knight }>(from, to, self.turn),
            Rook => self.board.is_pseudo::<{ Rook }>(from, to, self.turn),
            Bishop => self.board.is_pseudo::<{ Bishop }>(from, to, self.turn),
            Queen => self.board.is_pseudo::<{ Queen }>(from, to, self.turn),
            _ => panic!("Cannot make a generic move with a King or Pawn"),
        };
        let captured = if is_pseudo {
            self.test_move_for_check(from, to)
        } else {
            Err(MoveError::IllegalMove)
        }?;
        // own castle
        if FIGURE == Rook {
            let (q_rook_sq, k_rook_sq) = match self.turn {
                Color::White => (Square::A1, Square::H1),
                Color::Black => (Square::A8, Square::H8),
            };
            match from {
                sq if sq == k_rook_sq => self.castle.remove_king_castle(self.turn),
                sq if sq == q_rook_sq => self.castle.remove_queen_castle(self.turn),
                _ => {}
            }
        }
        // ep
        self.ep_square = None;
        // half move
        match captured {
            Some(_piece) => self.half_move = 0,
            None => self.half_move += 1,
        }
        Ok(captured)
    }

    fn test_move_for_check(&mut self, from: Square, to: Square) -> MoveResult {
        let captured = self.board.move_piece(from, to);
        if self.board.is_in_check(self.turn) {
            self.board.unmove_piece(from, to, captured);
            return Err(MoveError::KingInCheck);
        }
        Ok(captured)
    }

    pub fn iter(&self) -> impl Iterator<Item = (Square, Piece)> {
        self.board.iter()
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.board)?;
        writeln!(f, "turn: {:?}", self.turn)?;
        writeln!(f, "castle: {:?}", self.castle)?;
        writeln!(f, "ep: {:?}", self.ep_square)?;
        writeln!(f, "half: {:?}", self.half_move)?;
        writeln!(f, "full: {:?}", self.full_move)
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
    pub const KNPR: &str = "rn1nk2r/2P5/8/3pP3/6p1/8/4p1P1/R3K2R w KQq d6 0 1";
    pub const EPCHECK: &str = "4k3/8/8/r2pP2K/8/8/8/8 w - d6 0 1";
    pub const CASTLECHECK: &str = "r3k2r/1p6/2B5/8/8/5q2/6P1/R3K2R w KQkq - 0 1";
}

#[cfg(test)]
mod tests;
