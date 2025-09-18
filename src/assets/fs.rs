use macroquad::{
    audio::{load_sound, Sound}, prelude::{load_image, load_texture, Image, Texture2D}
};

const ASSETS_DIR: &str = "assets";
const DATA_DIR: &str = "data";
const TEXTURES_DIR: &str = "textures";
const AUDIO_DIR: &str = "audio";
const SOUNDS_DIR: &str = "audio/sounds";

pub async fn get_texture(path: &str) -> Texture2D {
    let tex = load_texture(format!("{}/{}/{}", ASSETS_DIR, TEXTURES_DIR, path).as_str()).await.unwrap();
    
    tex
}

pub async fn get_image(path: &str) -> Image {
    let image = load_image(format!("{}/{}/{}", ASSETS_DIR, TEXTURES_DIR, path).as_str()).await.unwrap();
    image
}

pub fn get_data(path: &str) -> String {
    format!("{}/{}/{}", ASSETS_DIR, DATA_DIR, path)
}

pub async fn get_audio(path: &str) -> Sound {
    let audio = load_sound(format!("{}/{}/{}", ASSETS_DIR, AUDIO_DIR, path).as_str()).await.unwrap();
    audio
}

pub async fn get_sound(path: &str) -> Sound {
    let sound = load_sound(format!("{}/{}/{}", ASSETS_DIR, SOUNDS_DIR, path).as_str()).await.unwrap();
    sound
}