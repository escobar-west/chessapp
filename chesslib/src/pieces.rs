use constants::*;
use errors::InvalidChar;
use std::ops::Not;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Piece {
    pub color: Color,
    pub figure: Figure,
}

impl From<Piece> for char {
    fn from(piece: Piece) -> Self {
        let c = match piece.figure {
            Figure::Pawn => 'P',
            Figure::Rook => 'R',
            Figure::Knight => 'N',
            Figure::Bishop => 'B',
            Figure::Queen => 'Q',
            Figure::King => 'K',
        };
        match piece.color {
            Color::White => c,
            Color::Black => c.to_lowercase().next().unwrap(),
        }
    }
}

impl TryFrom<char> for Piece {
    type Error = InvalidChar;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'P' => Ok(WHITE_PAWN),
            'R' => Ok(WHITE_ROOK),
            'N' => Ok(WHITE_KNIGHT),
            'B' => Ok(WHITE_BISHOP),
            'Q' => Ok(WHITE_QUEEN),
            'K' => Ok(WHITE_KING),
            'p' => Ok(BLACK_PAWN),
            'r' => Ok(BLACK_ROOK),
            'n' => Ok(BLACK_KNIGHT),
            'b' => Ok(BLACK_BISHOP),
            'q' => Ok(BLACK_QUEEN),
            'k' => Ok(BLACK_KING),
            v => Err(errors::InvalidChar(v)),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    White,
    Black,
}

impl Not for Color {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Figure {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl Figure {
    pub fn iter() -> impl Iterator<Item = &'static Self> {
        use Figure::*;
        static FIGURES: [Figure; 6] = [Pawn, Rook, Knight, Bishop, Queen, King];
        FIGURES.iter()
    }
}

pub mod errors {
    use thiserror::Error;

    #[derive(Error, Debug)]
    #[error("Invalid char {0}")]
    pub struct InvalidChar(pub char);
}

pub mod constants {
    use super::*;
    pub const WHITE_PAWN: Piece = Piece {
        color: Color::White,
        figure: Figure::Pawn,
    };

    pub const WHITE_ROOK: Piece = Piece {
        color: Color::White,
        figure: Figure::Rook,
    };

    pub const WHITE_KNIGHT: Piece = Piece {
        color: Color::White,
        figure: Figure::Knight,
    };

    pub const WHITE_BISHOP: Piece = Piece {
        color: Color::White,
        figure: Figure::Bishop,
    };

    pub const WHITE_QUEEN: Piece = Piece {
        color: Color::White,
        figure: Figure::Queen,
    };

    pub const WHITE_KING: Piece = Piece {
        color: Color::White,
        figure: Figure::King,
    };

    pub const BLACK_PAWN: Piece = Piece {
        color: Color::Black,
        figure: Figure::Pawn,
    };

    pub const BLACK_ROOK: Piece = Piece {
        color: Color::Black,
        figure: Figure::Rook,
    };

    pub const BLACK_KNIGHT: Piece = Piece {
        color: Color::Black,
        figure: Figure::Knight,
    };

    pub const BLACK_BISHOP: Piece = Piece {
        color: Color::Black,
        figure: Figure::Bishop,
    };

    pub const BLACK_QUEEN: Piece = Piece {
        color: Color::Black,
        figure: Figure::Queen,
    };

    pub const BLACK_KING: Piece = Piece {
        color: Color::Black,
        figure: Figure::King,
    };
}
