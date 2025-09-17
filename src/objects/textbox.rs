use macroquad::prelude::*;

const PADDING: Vec2 = Vec2 { x: 10.0, y: 10.0 };

pub fn draw_textbox(text: &str, x: f32, y: f32) {

    draw_rectangle(x, y, screen_width() * 0.99, screen_height() * 0.2, BLACK);
    draw_text(text, x + PADDING.x, y + 12.0 + PADDING.y, 24.0, WHITE);
}