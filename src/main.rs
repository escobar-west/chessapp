mod view;

use chesslib::{errors::MoveError, prelude::*};
use errors::AppError;
use macroquad::input::{
    MouseButton, is_mouse_button_down, is_mouse_button_pressed, mouse_position,
};
use view::View;

#[macroquad::main("Chess")]
async fn main() -> Result<(), anyhow::Error> {
    let mut app = App::new(KNPR).await?;
    println!("{}", app.gs);
    loop {
        app.update_state();
        app.draw_state().await;
    }
}

struct App {
    gs: GameState,
    view: View,
    mouse: (f32, f32),
    app_state: AppState,
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
            app_state: AppState::Free,
            last_move: None,
        })
    }

    fn update_state(&mut self) {
        self.view.update_screen();
        self.mouse = mouse_position();
        match self.app_state {
            AppState::Free => self.update_free(),
            AppState::Clicked { from, piece } => self.update_clicked(from, piece),
            AppState::Promoting { from, to } => self.update_promoting(from, to),
        }
    }

    fn update_free(&mut self) {
        if !is_mouse_button_pressed(MouseButton::Left) {
            return;
        }
        let Some(square) = self.view.get_square_at_point(self.mouse) else {
            return;
        };
        let Some(piece) = self.gs.get_sq(square) else {
            return;
        };
        self.app_state = AppState::Clicked {
            from: square,
            piece,
        };
    }

    fn update_clicked(&mut self, from: Square, _piece: Piece) {
        if is_mouse_button_down(MouseButton::Left) {
            return;
        }
        self.app_state = AppState::Free;
        let Some(to) = self.view.get_square_at_point(self.mouse) else {
            return;
        };
        let res = self.gs.make_move(from, to);
        self.process_move_result(from, to, res);
    }

    fn update_promoting(&mut self, from: Square, to: Square) {
        if !is_mouse_button_pressed(MouseButton::Left) {
            return;
        }
        self.app_state = AppState::Free;
        let Some(clicked) = self.view.get_square_at_point(self.mouse) else {
            return;
        };
        if clicked.col() != to.col() {
            return;
        }
        let Some(piece) = self.get_promotion_piece(clicked.row()) else {
            return;
        };
        let res = self.gs.make_promotion(from, to, piece);
        self.process_move_result(from, to, res);
    }

    fn process_move_result(
        &mut self,
        from: Square,
        to: Square,
        res: Result<Option<Piece>, MoveError>,
    ) {
        println!("{:?}", res);
        println!("{}", self.gs);
        match res {
            Ok(Some(_)) => {
                self.last_move = Some((from, to));
                self.view.play_capture_sound();
            }
            Ok(None) => {
                self.last_move = Some((from, to));
                self.view.play_move_sound();
            }
            Err(MoveError::KingInCheck) => {
                self.view.play_in_check_sound();
            }
            Err(MoveError::Promoting) => {
                self.app_state = AppState::Promoting { from, to };
            }
            _ => {}
        }
    }

    fn get_promotion_piece(&self, rank: Row) -> Option<Piece> {
        let color = self.gs.get_turn();
        let figure = match color {
            Color::Black => match rank {
                Row::One => Some(Figure::Queen),
                Row::Two => Some(Figure::Rook),
                Row::Three => Some(Figure::Knight),
                Row::Four => Some(Figure::Bishop),
                _ => None,
            },
            Color::White => match rank {
                Row::Eight => Some(Figure::Queen),
                Row::Seven => Some(Figure::Rook),
                Row::Six => Some(Figure::Knight),
                Row::Five => Some(Figure::Bishop),
                _ => None,
            },
        };
        figure.map(|f| Piece { color, figure: f })
    }

    async fn draw_state(&self) {
        self.view.draw_board();
        if let Some(last_move) = self.last_move {
            self.view.draw_highlight(last_move.0);
            self.view.draw_highlight(last_move.1);
        }
        match self.app_state {
            AppState::Free => {
                for (square, piece) in self.gs.iter() {
                    self.view.draw_piece_at_square(piece, square);
                }
            }
            AppState::Clicked { from, piece } => {
                for (s, p) in self.gs.iter() {
                    if s != from {
                        self.view.draw_piece_at_square(p, s);
                    }
                }
                self.view.draw_piece_at_point(piece, self.mouse);
            }
            AppState::Promoting { from, to } => {
                for (s, p) in self.gs.iter() {
                    if s != from {
                        self.view.draw_piece_at_square(p, s);
                    }
                }
                self.view.draw_highlight(from);
                self.view
                    .draw_promotion_widget(to.col(), self.gs.get_turn());
            }
        }
        self.view.next_frame().await;
    }
}

enum AppState {
    Free,
    Clicked { from: Square, piece: Piece },
    Promoting { from: Square, to: Square },
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
