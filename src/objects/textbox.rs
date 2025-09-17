use macroquad::prelude::*;
use crate::types::*;

const PADDING: Vec2 = Vec2 { x: 10.0, y: 10.0 };

pub fn draw_textbox(text: &str, x: f32, y: f32) {

    draw_rectangle(x, y, screen_width() * 0.99, screen_height() * 0.2, BLACK);
    draw_text(text, x + PADDING.x, y + 12.0 + PADDING.y, 24.0, WHITE);
}

/**
 * Updates the Textbox based on `text_id`.
 * Also handles input for textbox advancing.
 * TODO: use u8 instead of usize
 */
pub fn textbox_loop(text_id: &mut usize, textbox_data: &TextboxData) {
    if !textbox_data.can_accept_text_id(*text_id) {
        // reached end of text data
        println!("Reached end of textbox data!");
        return;
    }

    let current_textbox_data: &TextboxDataEntry = textbox_data.get_entry(*text_id);

    draw_textbox(&current_textbox_data.text, screen_width() * 0.005, screen_height() * 0.79);

    // handle input
    if is_key_pressed(KeyCode::Space) {
        *text_id += 1;
    }
    if is_key_pressed(KeyCode::Left) && *text_id > 0 {
        *text_id -= 1;
    }
}