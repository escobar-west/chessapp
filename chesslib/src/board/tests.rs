use super::{Board, Row, Square, bitboard::BitBoard};
use crate::{
    constants::DEFAULT_FEN,
    pieces::{Color, constants::*},
};

#[test]
fn test_default_fen() {
    let fen = DEFAULT_FEN;
    let board = Board::try_from_fen(fen).unwrap();

    let pawn_mask = BitBoard::from(Row::Two) | BitBoard::from(Row::Seven);
    assert_eq!(
        board.white_pieces.pawns | board.black_pieces.pawns,
        pawn_mask
    );

    let rook_mask = BitBoard::from(Square::A1)
        | BitBoard::from(Square::H1)
        | BitBoard::from(Square::A8)
        | BitBoard::from(Square::H8);
    assert_eq!(
        board.white_pieces.rooks | board.black_pieces.rooks,
        rook_mask
    );

    let knight_mask = BitBoard::from(Square::B1)
        | BitBoard::from(Square::G1)
        | BitBoard::from(Square::B8)
        | BitBoard::from(Square::G8);
    assert_eq!(
        board.white_pieces.knights | board.black_pieces.knights,
        knight_mask
    );

    let bishop_mask = BitBoard::from(Square::C1)
        | BitBoard::from(Square::F1)
        | BitBoard::from(Square::C8)
        | BitBoard::from(Square::F8);
    assert_eq!(
        board.white_pieces.bishops | board.black_pieces.bishops,
        bishop_mask
    );

    let queen_mask = BitBoard::from(Square::D1) | BitBoard::from(Square::D8);
    assert_eq!(
        board.white_pieces.queens | board.black_pieces.queens,
        queen_mask
    );

    let king_mask = BitBoard::from(Square::E1) | BitBoard::from(Square::E8);
    assert_eq!(
        board.white_pieces.kings | board.black_pieces.kings,
        king_mask
    );

    let white_mask = BitBoard::from(Row::One) | BitBoard::from(Row::Two);
    assert_eq!(board.occupied_color(Color::White), white_mask);

    let black_mask = BitBoard::from(Row::Seven) | BitBoard::from(Row::Eight);
    assert_eq!(board.occupied_color(Color::Black), black_mask);

    let occ_mask = white_mask | black_mask;
    assert_eq!(board.occupied, occ_mask);

    //let to_fen = board.to_fen();
    //assert_eq!(to_fen, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
}

#[test]
fn test_clear_and_set_sq() {
    let fen = DEFAULT_FEN;
    let mut board = Board::try_from_fen(fen).unwrap();
    board.clear_sq(Square::A8);

    let rook_mask = BitBoard::from(Square::A1) | Square::H1.into() | Square::H8.into();
    assert_eq!(
        board.white_pieces.rooks | board.black_pieces.rooks,
        rook_mask
    );

    let piece = board.set_sq(Square::H1, BLACK_QUEEN);
    assert_eq!(piece, Some(WHITE_ROOK));

    let rook_mask = BitBoard::from(Square::A1) | Square::H8.into();
    assert_eq!(
        board.white_pieces.rooks | board.black_pieces.rooks,
        rook_mask
    );

    let queen_mask = BitBoard::from(Square::D1) | Square::H1.into() | Square::D8.into();
    assert_eq!(
        board.white_pieces.queens | board.black_pieces.queens,
        queen_mask
    );

    let _white_mask =
        BitBoard::from(Row::One) | BitBoard::from(Row::Two) ^ BitBoard::from(Square::H1);
    //assert_eq!(board.white_occupied, white_mask);

    let _black_mask =
        (BitBoard::from(Row::Seven) | BitBoard::from(Row::Eight) | BitBoard::from(Square::H1))
            ^ BitBoard::from(Square::A8);
    //assert_eq!(board.black_occupied, black_mask);
}

#[test]
fn test_shift() {
    let s = Square::A1;
    let s = s.shift::<0, 1>().unwrap();
    println!("{s:?}");
    let s = s.shift::<0, 1>().unwrap();
    println!("{s:?}");
    let s = s.shift::<0, 1>().unwrap();
    println!("{s:?}");
    let s = s.shift::<0, 1>().unwrap();
    println!("{s:?}");
    let s = s.shift::<0, 1>().unwrap();
    println!("{s:?}");
    let s = s.shift::<0, 1>().unwrap();
    println!("{s:?}");
    let s = s.shift::<0, 1>().unwrap();
    println!("{s:?}");
    let s = s.shift::<0, 1>().unwrap();
    println!("{s:?}");
}
