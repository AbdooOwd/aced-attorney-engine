use macroquad::{
    audio::{load_sound, Sound}, prelude::{load_image, load_texture, Image, Texture2D}
};

const _ASSETS_DIR: &str = "assets";
const DATA_DIR: &str = "assets/data";
const TEXTURES_DIR: &str = "assets/textures";
const AUDIO_DIR: &str = "assets/audio";

pub async fn get_texture(path: &str) -> Texture2D {
    let tex = load_texture(format!("{}/{}", TEXTURES_DIR, path).as_str()).await.unwrap();
    
    tex
}

pub async fn get_image(path: &str) -> Image {
    let image = load_image(format!("{}/{}", TEXTURES_DIR, path).as_str()).await.unwrap();
    image
}

pub fn get_data(path: &str) -> String {
    format!("{}/{}", DATA_DIR, path)
}

pub async fn get_audio(path: &str) -> Sound {
    let audio = load_sound(format!("{}/{}", AUDIO_DIR, path).as_str()).await.unwrap();
    audio
}