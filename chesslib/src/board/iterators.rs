use super::{Board, PieceSet, Square, bitboard::BitBoard};
use crate::pieces::{Figure, Piece};
use std::iter::repeat;

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

impl BitBoard {
    pub fn iter(self) -> impl Iterator<Item = Square> {
        BitBoardIter { rem_board: self }
    }
}

impl PieceSet {
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

impl Board {
    pub fn iter(&self) -> impl Iterator<Item = (Square, Piece)> {
        self.white_pieces.iter().chain(self.black_pieces.iter())
    }

    pub fn iter_piece(&self, piece: Piece) -> impl Iterator<Item = Square> {
        self.get_piece_board(piece).iter()
    }
}
