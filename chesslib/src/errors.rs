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
    #[error("King in check")]
    KingInCheck,
}

#[derive(Error, Debug)]
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

#[derive(Error, Debug)]
#[error("Invalid char {0}")]
pub struct ParsePieceError(pub char);

#[derive(Error, Debug)]
#[error("Invalid input: {0}")]
pub struct InvalidValueError(pub u8);