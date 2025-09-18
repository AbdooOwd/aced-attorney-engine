use macroquad::prelude::{Texture2D, load_texture, FilterMode, Image, load_image};

const ASSETS_DIR: &str = "assets";
const DATA_DIR: &str = "assets/data";
const TEXTURES_DIR: &str = "assets/textures";

pub async fn get_texture(path: &str) -> Texture2D {
    let tex = load_texture(format!("{}/{}", TEXTURES_DIR, path).as_str()).await.unwrap();
    tex.set_filter(FilterMode::Nearest);
    
    tex
}

pub async fn get_image(path: &str) -> Image {
    let image = load_image(format!("{}/{}", TEXTURES_DIR, path).as_str()).await.unwrap();
    image
}

pub fn get_data(path: &str) -> String {
    format!("{}/{}", DATA_DIR, path)
}