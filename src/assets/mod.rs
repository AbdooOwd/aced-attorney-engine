pub mod fs;
pub mod sound_table;
pub mod texture_table;
pub mod acapella_table;

use macroquad::{
    audio::Sound, 
    prelude::{set_default_filter_mode, FilterMode, Texture2D},
};
use crate::assets::{fs::{get_audio, get_sound, get_texture}, sound_table::*, texture_table::*, acapella_table::*};




/** Global/General Game Assets */
pub struct GameAssets {
    pub speaker_tex: Texture2D,

    pub bgm: Sound,

    // sounds
    pub snd_ph_objection: Sound,
    pub snd_ph_hold_it: Sound
}


impl GameAssets {
    pub async fn load() -> GameAssets {
        // pixel not blurry!
        set_default_filter_mode(FilterMode::Nearest);

        GameAssets {
            speaker_tex: get_texture(TEX_POB_NORMAL).await,

            bgm: get_audio(ACPL_OBJECTION_2002).await,

            snd_ph_objection: get_sound(SND_PH_OBJECTION).await,
            snd_ph_hold_it: get_sound(SND_PH_HOLD_IT).await,
        }
    }
}