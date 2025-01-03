mod view;
use chesslib::{GameState, constants::*};
use macroquad::{input::mouse_position, window::next_frame};

#[macroquad::main("Chess")]
async fn main() {
    let gs = GameState::default();
    let mut view = view::View::new().await;
    loop {
        view.update_screen();
        view.draw_board();
        for (square, piece) in gs.iter() {
            view.draw_piece_at_coords(square.col(), square.row(), piece);
        }
        let (x, y) = mouse_position();
        view.draw_piece_at_point(x, y, WHITE_KING);
        next_frame().await
    }
}
