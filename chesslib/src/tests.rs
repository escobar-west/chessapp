use super::*;
use Square::*;
use prelude::{BLACK_PAWN, WHITE_PAWN};

#[test]
fn test_pawn_moves() {
    const WHITE_FEN: &str = "4k3/6P1/6P1/8/1p6/p1p5/PP4P1/4K3 w - - 0 1";
    let mut gs = GameState::try_from_fen(WHITE_FEN).unwrap();

    let res = gs.make_move(A2, A3);
    assert!(res.is_err());

    let res = gs.make_move(B2, B4);
    assert!(res.is_err());

    let res = gs.make_move(G6, G7);
    assert!(res.is_err());

    let res = gs.make_move(G7, G8);
    assert!(res.is_err());

    let res = gs.make_move(G2, F3);
    assert!(res.is_err());

    let res = gs.make_move(G2, F2);
    assert!(res.is_err());

    let res = gs.make_move(G2, F1);
    assert!(res.is_err());

    let res = gs.make_move(G2, G1);
    assert!(res.is_err());

    let res = gs.make_move(G2, H1);
    assert!(res.is_err());

    let res = gs.make_move(G2, H2);
    assert!(res.is_err());

    let res = gs.make_move(G2, H3);
    assert!(res.is_err());

    let res = gs.make_move(B2, A3).unwrap();
    assert_eq!(res, Some(BLACK_PAWN));

    let mut gs = GameState::try_from_fen(WHITE_FEN).unwrap();
    let res = gs.make_move(B2, B3).unwrap();
    assert_eq!(res, None);

    let mut gs = GameState::try_from_fen(WHITE_FEN).unwrap();
    let res = gs.make_move(B2, C3).unwrap();
    assert_eq!(res, Some(BLACK_PAWN));

    let mut gs = GameState::try_from_fen(WHITE_FEN).unwrap();
    let res = gs.make_move(G2, G3).unwrap();
    assert_eq!(res, None);

    let mut gs = GameState::try_from_fen(WHITE_FEN).unwrap();
    let res = gs.make_move(G2, G4).unwrap();
    assert_eq!(res, None);

    const BLACK_FEN: &str = "4k3/2p5/1P1P4/8/8/8/8/4K3 b - - 0 1";

    let mut gs = GameState::try_from_fen(BLACK_FEN).unwrap();
    let res = gs.make_move(C7, C6).unwrap();
    assert_eq!(res, None);

    let mut gs = GameState::try_from_fen(BLACK_FEN).unwrap();
    let res = gs.make_move(C7, C5).unwrap();
    assert_eq!(res, None);

    let mut gs = GameState::try_from_fen(BLACK_FEN).unwrap();
    let res = gs.make_move(C7, B6).unwrap();
    assert_eq!(res, Some(WHITE_PAWN));

    let mut gs = GameState::try_from_fen(BLACK_FEN).unwrap();
    let res = gs.make_move(C7, D6).unwrap();
    assert_eq!(res, Some(WHITE_PAWN));
}

#[test]
fn test_ep_pawn_moves() {
    const WHITE_FEN: &str = "4k3/8/8/3pP3/8/8/8/4K3 w - d6 0 1";

    let mut gs = GameState::try_from_fen(WHITE_FEN).unwrap();

    let res = gs.make_move(E5, F6);
    assert!(res.is_err());

    let res = gs.make_move(E5, D6).unwrap();
    assert_eq!(res, Some(BLACK_PAWN));

    const BLACK_FEN: &str = "4k3/8/8/8/1Pp5/8/8/4K3 b - b3 0 1";

    let mut gs = GameState::try_from_fen(BLACK_FEN).unwrap();

    let res = gs.make_move(C4, D3);
    assert!(res.is_err());

    let res = gs.make_move(C4, B3).unwrap();
    assert_eq!(res, Some(WHITE_PAWN));
}
