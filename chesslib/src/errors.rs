use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum MoveError {
    #[error("Empty square")]
    EmptySquare,
    #[error("Wrong turn")]
    WrongTurn,
    #[error("Illegal move")]
    IllegalMove,
    #[error("King in check")]
    KingInCheck,
    #[error("Pawn Promotion")]
    PawnPromotion,
}

#[derive(Error, Debug, Clone)]
pub enum ParseFenError {
    #[error("Empty FEN entry")]
    EmptyFen,
    #[error("Wrong number of rows")]
    WrongRowCount,
    #[error("Illegal state")]
    IllegalState,
    #[error("Invalid color: {0:#?}")]
    InvalidColor(String),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error(transparent)]
    ParsePieceError(#[from] ParsePieceError),
    #[error(transparent)]
    InvalidValueError(#[from] InvalidValueError),
}

#[derive(Error, Debug, Copy, Clone)]
#[error("Invalid char {0}")]
pub struct ParsePieceError(pub char);

#[derive(Error, Debug, Copy, Clone)]
#[error("Invalid input: {0}")]
pub struct InvalidValueError(pub u8);
