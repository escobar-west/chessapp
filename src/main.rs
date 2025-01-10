mod view;

use chesslib::prelude::*;
use errors::AppError;
use macroquad::input::{
    MouseButton, is_mouse_button_pressed, is_mouse_button_released, mouse_position,
};
use view::View;

#[macroquad::main("Chess")]
async fn main() -> Result<(), anyhow::Error> {
    let mut app = App::new(KINGS_PAWNS).await?;
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
                    if let Err(res) = self.gs.make_move(last_pressed.square, to) {
                        println!("{res:?}");
                    }
                };
            }
            self.last_pressed = None;
        }
    }

    async fn draw_state(&self) {
        self.view.draw_board();
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
