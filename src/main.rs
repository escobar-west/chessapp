mod view;

use chesslib::{errors::MoveError, prelude::*};
use errors::AppError;
use macroquad::{
    input::{MouseButton, is_mouse_button_pressed, is_mouse_button_released, mouse_position},
    logging::debug,
};
use view::View;

#[macroquad::main("Chess")]
async fn main() -> Result<(), anyhow::Error> {
    let mut app = App::new(KNP).await?;
    loop {
        app.update_state();
        app.draw_state().await;
    }
}

struct App {
    gs: GameState,
    view: View,
    mouse: (f32, f32),
    last_pressed: Option<LastPressed>,
    last_move: Option<(Square, Square)>,
}

impl App {
    async fn new(fen: &str) -> Result<Self, AppError> {
        let gs = GameState::try_from_fen(fen)?;
        let view = View::new().await;
        Ok(Self {
            gs,
            view,
            mouse: mouse_position(),
            last_pressed: None,
            last_move: None,
        })
    }

    fn update_state(&mut self) {
        self.view.update_screen();
        self.mouse = mouse_position();
        if is_mouse_button_pressed(MouseButton::Left) {
            let square = self.view.get_square_at_point(self.mouse);
            let piece = square.and_then(|s| self.gs.get_sq(s));
            self.last_pressed = square
                .zip(piece)
                .map(|(square, piece)| LastPressed { square, piece })
        }
        if is_mouse_button_released(MouseButton::Left) {
            if let Some(last_pressed) = self.last_pressed {
                if let Some(to) = self.view.get_square_at_point(self.mouse) {
                    let res = self.gs.make_move(last_pressed.square, to);
                    #[cfg(debug_assertions)]
                    debug!("{:#?}", res);
                    match res {
                        Ok(Some(_)) => {
                            self.last_move = Some((last_pressed.square, to));
                            self.view.play_capture_sound();
                        }
                        Ok(None) => {
                            self.last_move = Some((last_pressed.square, to));
                            self.view.play_move_sound();
                        }
                        Err(MoveError::KingInCheck) => {
                            self.view.play_in_check_sound();
                        }
                        Err(MoveError::PawnPromotion) => {}
                        _ => {}
                    }
                };
            }
            self.last_pressed = None;
        }
    }

    async fn draw_state(&self) {
        self.view.draw_board();
        if let Some(last_move) = self.last_move {
            self.view.draw_highlight(last_move.0);
            self.view.draw_highlight(last_move.1);
        }
        match self.last_pressed {
            Some(LastPressed { square, piece }) => {
                for (s, p) in self.gs.iter() {
                    if s != square {
                        self.view.draw_piece_at_square(p, s);
                    }
                }
                self.view.draw_piece_at_point(piece, self.mouse);
            }
            None => {
                for (square, piece) in self.gs.iter() {
                    self.view.draw_piece_at_square(piece, square);
                }
            }
        }
        self.view.next_frame().await;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct LastPressed {
    square: Square,
    piece: Piece,
}

pub mod errors {
    use chesslib::errors::ParseFenError;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum AppError {
        #[error(transparent)]
        ParseFenError(#[from] ParseFenError),
    }
}
