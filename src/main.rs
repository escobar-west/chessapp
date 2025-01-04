mod view;
use chesslib::{GameState, constants::*};
use macroquad::{
    input::{MouseButton, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released},
    window::next_frame,
};

#[macroquad::main("Chess")]
async fn main() {
    let gs = GameState::default();
    let mut view = view::View::new().await;
    loop {
        view.update_screen();
        view.draw_board();
        for (square, piece) in gs.iter() {
            view.draw_piece_at_square(piece, square);
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            println!("start press")
        }
        if is_mouse_button_down(MouseButton::Left) {
            println!("pressing button")
        }
        if is_mouse_button_released(MouseButton::Left) {
            println!("releasing button")
        }
        if let Some(s) = view.get_square_at_mouse() {
            view.draw_piece_at_square(BLACK_KING, s);
        };
        view.draw_piece_at_mouse(WHITE_KING);
        next_frame().await
    }
}
