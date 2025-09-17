use macroquad::prelude::*;
use crate::{helpers::{draw_from_texture, get_emotion_texture_path}, objects::textbox::*, types::*};

const SPEAKER_TEXTURE_SCALER: f32 = 4.0;

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

pub async fn update_speaker_emotion_texture(emotion: SpeakerEmotion, speaker_tex: &Texture2D) {
    // this emotion_texture_path variable seems... "unoptimized" and "non-performant"...
    let emotion_texture_path = get_emotion_texture_path(emotion);
    let new_image = load_image(emotion_texture_path.as_str()).await.unwrap();
    speaker_tex.update(&new_image);
    draw_from_texture(speaker_tex, 50.0, 60.0, SPEAKER_TEXTURE_SCALER);
}