mod view;
use chesslib::{GameState, Piece, Square};
use macroquad::input::{
    MouseButton, is_mouse_button_pressed, is_mouse_button_released, mouse_position,
};

#[macroquad::main("Chess")]
async fn main() {
    let mut gs = GameState::default();
    let mut view = view::View::new().await;
    let mut last_pressed: Option<LastPressed> = None;
    loop {
        view.update_screen();
        let (mouse_x, mouse_y) = mouse_position();
        if is_mouse_button_pressed(MouseButton::Left) {
            let square = view.get_square_at_point(mouse_x, mouse_y);
            let piece = square.and_then(|s| gs.get_sq(s));
            last_pressed = square
                .zip(piece)
                .map(|(square, piece)| LastPressed { square, piece })
        }
        if is_mouse_button_released(MouseButton::Left) {
            if let Some(last_pressed) = last_pressed {
                view.get_square_at_point(mouse_x, mouse_y)
                    .and_then(|to| gs.make_illegal_move(last_pressed.square, to));
            }
            last_pressed = None;
        }
        view.draw_board();
        match last_pressed {
            Some(LastPressed { square, piece }) => {
                for (s, p) in gs.iter() {
                    if s != square {
                        view.draw_piece_at_square(p, s);
                    }
                }
                view.draw_piece_at_point(piece, mouse_x, mouse_y);
            }
            None => {
                for (square, piece) in gs.iter() {
                    view.draw_piece_at_square(piece, square);
                }
            }
        }
        view.next_frame().await;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct LastPressed {
    square: Square,
    piece: Piece,
}
