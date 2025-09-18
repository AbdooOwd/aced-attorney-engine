use macroquad::prelude::*;
use crate::{assets::fs::get_texture, types::*};

pub fn draw_from_texture(texture: &Texture2D, x: f32, y: f32, scaler: f32) {
    draw_texture_ex(&texture, x, y, WHITE,
        DrawTextureParams { 
            dest_size: Some(Vec2 {x: texture.width() * scaler, y: texture.height() * scaler}), 
            source: None, 
            rotation: 0.0, 
            flip_x: false, 
            flip_y: false, 
            pivot: None 
        }
    );
}

/**
 * Draws a texture from `path`, and mutliplies its scale by `scaler`
 */
pub async fn draw_texture_from_path(path: &str, x: f32, y: f32, scaler: f32) {
    let tex = get_texture(path).await;
    draw_from_texture(&tex, x, y, scaler);
}

/**
 * TODO: what the heck is `<&'static str>`?
 * TODO: for now, pob is hardcoded as the only speaker texture 
*/
 pub fn get_emotion_texture_path(emotion: SpeakerEmotion) -> String {
    let emotion_texture_filename = match emotion {
        SpeakerEmotion::NaN => "hmm", // hmm, why is it NaN... (jokes aside, this handles "null" emotions)
        SpeakerEmotion::Normal => "normal",
        SpeakerEmotion::Exclamation => "huh",
        SpeakerEmotion::Thinking => "hmm",
        SpeakerEmotion::Objection => "objection",
    };

    format!("pob/{}.png", emotion_texture_filename)
}