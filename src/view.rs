use chesslib::prelude::*;
use macroquad::{
    color::WHITE,
    math::{Rect, Vec2},
    texture::{DrawTextureParams, Texture2D, draw_texture_ex, load_texture},
    window::{next_frame, screen_height, screen_width},
};

type MousePos = (f32, f32);

pub struct View {
    width: f32,
    height: f32,
    board_size: f32,
    square_size: f32,
    board_texture: Texture2D,
    piece_texture: Texture2D,
}

impl View {
    pub async fn new() -> Self {
        let board_texture: Texture2D = load_texture("assets/boards/default.png").await.unwrap();
        let piece_texture: Texture2D = load_texture("assets/pieces/wiki_chess.png").await.unwrap();
        let width = screen_width();
        let height = screen_height();
        let board_size = width.min(height);
        let square_size = board_size / 8.0;
        Self {
            width,
            height,
            board_size,
            square_size,
            board_texture,
            piece_texture,
        }
    }

    pub async fn next_frame(&self) {
        next_frame().await;
    }

    pub fn update_screen(&mut self) {
        self.width = screen_width();
        self.height = screen_height();
        self.board_size = self.width.min(self.height);
        self.square_size = self.board_size / 8.0;
    }

    pub fn draw_board(&self) {
        draw_texture_ex(&self.board_texture, 0.0, 0.0, WHITE, DrawTextureParams {
            dest_size: Some(Vec2::splat(self.board_size)),
            ..Default::default()
        });
    }

    pub fn get_square_at_point(&self, (x, y): MousePos) -> Option<Square> {
        if x <= 0.0 || y <= 0.0 || self.board_size <= x || self.board_size <= y {
            return None;
        }
        let (col, row) = (
            (x / self.square_size).floor() as u8,
            ((self.board_size - y) / self.square_size).floor() as u8,
        );
        let col = col.try_into().ok()?;
        let row = row.try_into().ok()?;
        Some(Square::from_coords(col, row))
    }

    pub fn draw_piece_at_point(&self, piece: Piece, (x, y): MousePos) {
        let top_left_x = x - self.square_size / 2.0;
        let top_left_y = y - self.square_size / 2.0;
        self.draw_piece_at(piece, top_left_x, top_left_y);
    }

    pub fn draw_piece_at_square(&self, piece: Piece, square: Square) {
        let top_left_x = square.col() as u8 as f32 * self.square_size;
        let top_left_y = (7 - square.row() as u8) as f32 * self.square_size;
        self.draw_piece_at(piece, top_left_x, top_left_y);
    }

    fn draw_piece_at(&self, piece: Piece, x: f32, y: f32) {
        let rectangle = match piece {
            WHITE_KING => WK_RECTANGLE,
            WHITE_QUEEN => WQ_RECTANGLE,
            WHITE_BISHOP => WB_RECTANGLE,
            WHITE_KNIGHT => WN_RECTANGLE,
            WHITE_ROOK => WR_RECTANGLE,
            WHITE_PAWN => WP_RECTANGLE,
            BLACK_KING => BK_RECTANGLE,
            BLACK_QUEEN => BQ_RECTANGLE,
            BLACK_BISHOP => BB_RECTANGLE,
            BLACK_KNIGHT => BN_RECTANGLE,
            BLACK_ROOK => BR_RECTANGLE,
            BLACK_PAWN => BP_RECTANGLE,
        };
        draw_texture_ex(&self.piece_texture, x, y, WHITE, DrawTextureParams {
            dest_size: Some(Vec2::splat(self.square_size)),
            source: Some(rectangle),
            ..Default::default()
        });
    }
}

const WK_RECTANGLE: Rect = Rect {
    x: 0.0,
    y: 0.0,
    w: 170.0,
    h: 170.0,
};
const WQ_RECTANGLE: Rect = Rect {
    x: 171.0,
    y: 0.0,
    w: 170.0,
    h: 170.0,
};
const WB_RECTANGLE: Rect = Rect {
    x: 342.0,
    y: 0.0,
    w: 170.0,
    h: 170.0,
};
const WN_RECTANGLE: Rect = Rect {
    x: 513.0,
    y: 0.0,
    w: 170.0,
    h: 170.0,
};
const WR_RECTANGLE: Rect = Rect {
    x: 684.0,
    y: 0.0,
    w: 170.0,
    h: 170.0,
};
const WP_RECTANGLE: Rect = Rect {
    x: 855.0,
    y: 0.0,
    w: 170.0,
    h: 170.0,
};
const BK_RECTANGLE: Rect = Rect {
    x: 0.0,
    y: 171.0,
    w: 170.0,
    h: 170.0,
};
const BQ_RECTANGLE: Rect = Rect {
    x: 171.0,
    y: 171.0,
    w: 170.0,
    h: 170.0,
};
const BB_RECTANGLE: Rect = Rect {
    x: 342.0,
    y: 171.0,
    w: 170.0,
    h: 170.0,
};
const BN_RECTANGLE: Rect = Rect {
    x: 513.0,
    y: 171.0,
    w: 170.0,
    h: 170.0,
};
const BR_RECTANGLE: Rect = Rect {
    x: 684.0,
    y: 171.0,
    w: 170.0,
    h: 170.0,
};
const BP_RECTANGLE: Rect = Rect {
    x: 855.0,
    y: 171.0,
    w: 170.0,
    h: 170.0,
};
