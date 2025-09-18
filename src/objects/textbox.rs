use macroquad::{prelude::*, experimental::collections::storage};
use crate::{
    assets::GameAssets, 
    config::{PRIMARY_COLOR, SECONDARY_COLOR}, 
    objects::shout::{do_shout, get_shout_type_from_str, HAS_SHOUTED}, 
    types::*
};

const FONT_SIZE: f32 = 36.0;
const PADDING_FONT_RELATIVE_VALUE: f32 = 0.4;
const PADDING: Vec2 = Vec2 {
    x: FONT_SIZE * PADDING_FONT_RELATIVE_VALUE,
    y: FONT_SIZE * PADDING_FONT_RELATIVE_VALUE
};

/**
 * Handles drawing the textbox's background + text, including positioning
 */
pub fn draw_textbox(text: &str, x: f32, y: f32) {
    draw_rectangle(x, y, screen_width() * 0.99, screen_height() * 0.2, SECONDARY_COLOR);
    draw_text(text, x + PADDING.x, y + FONT_SIZE / 2.0 + PADDING.y, FONT_SIZE, PRIMARY_COLOR);
}

/**
 * Updates the Textbox based on `text_id`.
 * Also handles input for textbox advancing.
 * TODO: use u8 instead of usize
 */
pub async fn textbox_loop(text_id: &mut usize, textbox_data: &TextboxData) {
    if !textbox_data.can_accept_text_id(*text_id) {
        // reached end of text data
        println!("Reached end of textbox data!");
        return;
    }

    let _game_assets = storage::get::<GameAssets>();
    let current_textbox_data: &TextboxDataEntry = textbox_data.get_entry(*text_id);
    let current_text: &String = &current_textbox_data.text;

    if current_text.starts_with("shout:") {
        let shout_type_str = current_text.split(':').last().unwrap();
        let shout_type: ShoutType = get_shout_type_from_str(shout_type_str.to_string());
        do_shout(shout_type).await;
    } else {
        draw_textbox(&current_textbox_data.text, screen_width() * 0.005, screen_height() * 0.79);
    }

    // handle input
    if is_key_pressed(KeyCode::Space) {
        *text_id += 1;
        unsafe { HAS_SHOUTED = false; }
    }
    if is_key_pressed(KeyCode::Left) && *text_id > 0 {
        *text_id -= 1;
        unsafe { HAS_SHOUTED = false; }
    }
}