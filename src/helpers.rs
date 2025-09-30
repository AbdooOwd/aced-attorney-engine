use macroquad::{miniquad::conf::Icon, prelude::*};
use image::ImageReader;
use crate::{assets::fs::get_texture, types::*};

const ICON_PATH_PREFIX: &str = "assets/logo";

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
        SpeakerEmotion::Reading => "read"
    };

    format!("pob/{}.png", emotion_texture_filename)
}


#[allow(non_snake_case)]
pub fn get_Icon() -> Icon {
    // TODO: I must find a better way to do this... This is painful...
    // Painful to code, to look at and see, to work with, for the computer, on preformance...

    //! WARNING! THIS IS HARDCODED! (Well, actually: no, it would work if you have the correct sizes for the logos...)

    // TODO: I'd like this to be independent from the "image" crate

    let img16 = ImageReader::open(format!("{}16.png", ICON_PATH_PREFIX)).unwrap().decode().unwrap().into_bytes();
    let img32 = ImageReader::open(format!("{}32.png", ICON_PATH_PREFIX)).unwrap().decode().unwrap().into_bytes();
    let img64 = ImageReader::open(format!("{}.png", ICON_PATH_PREFIX)).unwrap().decode().unwrap().into_bytes();

    let data16: [u8; 1024] = img16.try_into().expect("Couldn't get icon16 as u8 arr!");
    let data32: [u8; 4096] = img32.try_into().expect("Couldn't get icon32 as u8 arr!");
    let data64: [u8; 16384] = img64.try_into().expect("Couldn't get icon64 as u8 arr!");

    Icon {
        big: data64,
        medium: data32,
        small: data16
    }
}