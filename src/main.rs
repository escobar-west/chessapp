use chesslib::GameState;
use macroquad::prelude::*;

#[macroquad::main("Chess")]
async fn main() {
    let gs = GameState::default();
    let texture: Texture2D = load_texture("assets/boards/default.png").await.unwrap();
    loop {
        let board_size = screen_width().min(screen_height());
        clear_background(WHITE);
        draw_texture_ex(&texture, 0.0, 0.0, WHITE, DrawTextureParams {
            dest_size: Some(Vec2::splat(board_size)),
            ..Default::default()
        });
        next_frame().await
    }
}

struct View {
    board_texture: Texture2D,
}

struct App {
    state: GameState,
    view: View,
}
