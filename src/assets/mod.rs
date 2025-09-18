pub mod fs;

use macroquad::{
    audio::Sound, 
    prelude::{set_default_filter_mode, FilterMode, Texture2D},
};
use crate::assets::fs::{get_texture, get_audio};

const DEFAULT_SPEAKER_SPRITE_FILENAME: &str = "pob/normal.png";
const DEFAULT_BGM_FILENAME: &str = "objection_2002.ogg";
const DEFAULT_SHOUT_SPRITE_FILENAME: &str = "objection.png";

/** Global/General Game Assets */
pub struct GameAssets {
    pub speaker_tex: Texture2D,
    pub shouting_tex: Texture2D,
    pub bgm: Sound,
}


impl GameAssets {
    pub async fn initialize() -> GameAssets {
        // pixel not blurry!
        set_default_filter_mode(FilterMode::Nearest);

        GameAssets {
            speaker_tex: get_texture(DEFAULT_SPEAKER_SPRITE_FILENAME).await,
            shouting_tex: get_texture(DEFAULT_SHOUT_SPRITE_FILENAME).await,
            bgm: get_audio(DEFAULT_BGM_FILENAME).await,
        }
    }
}