mod view;
use chesslib::{Column, GameState, Row, Square, constants::*};
use macroquad::prelude::*;

#[macroquad::main("Chess")]
async fn main() {
    let _gs = GameState::default();
    let mut view = view::View::new().await;
    loop {
        view.update_screen();
        view.draw_board();
        view.draw_piece_at_coords(Column::E, Row::One, WHITE_KING);
        next_frame().await
    }
}
