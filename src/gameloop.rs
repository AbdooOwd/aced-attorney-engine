use macroquad::prelude::*;
use crate::{helpers::{draw_from_texture, get_emotion_texture_path}, types::*};

// TODO: should this "speaker" stuff be moved to its separate file in "objects/"?

const SPEAKER_TEXTURE_SCALER: f32 = 4.0;

pub async fn update_speaker_emotion_texture(emotion: SpeakerEmotion, speaker_tex: &Texture2D) {
    // this emotion_texture_path variable seems... "unoptimized" and "non-performant"...
    let emotion_texture_path = get_emotion_texture_path(emotion);
    let new_image = load_image(emotion_texture_path.as_str()).await.unwrap();
    speaker_tex.update(&new_image);
    draw_from_texture(speaker_tex, 50.0, screen_height() - f32::from(new_image.height) * SPEAKER_TEXTURE_SCALER, SPEAKER_TEXTURE_SCALER);
}