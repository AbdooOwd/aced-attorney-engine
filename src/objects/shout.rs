// Objection, Hold It, Yes... Those popups
use macroquad::{
    audio::{play_sound_once, Sound}, 
    prelude::{collections::storage, Texture2D}, window::{screen_height, screen_width}
};
use crate::{
    assets::{fs::get_texture, GameAssets}, 
    config::SHOUT_TEX_SCALE, 
    helpers::draw_from_texture, 
    types::ShoutType
};

// pub static shout_is_visible: bool = false;

pub async fn do_shout(shout_type: ShoutType) {
    let shout_tex = get_shout_tex(shout_type).await;

    draw_from_texture(&shout_tex, 
        0.5 * (screen_width() - shout_tex.width() * SHOUT_TEX_SCALE),
        0.5 * (screen_height() - shout_tex.height() * SHOUT_TEX_SCALE),
    SHOUT_TEX_SCALE);

    yell(shout_type);
}

pub async fn get_shout_tex(shout_type: ShoutType) -> Texture2D {
    let shout_filename: String = get_shout_sprite_filename(shout_type);
    let new_shout_tex: Texture2D = get_texture(shout_filename.as_str()).await;

    new_shout_tex
}

pub static mut HAS_SHOUTED: bool = false;

pub fn yell(shout_type: ShoutType) {
    // TODO: gotta get rid of the unsafe
    if unsafe { HAS_SHOUTED } {
        return;
    }
    let yelling: Sound = get_shout_audio(shout_type);
    play_sound_once(&yelling);
    unsafe { HAS_SHOUTED = true; }
}


pub fn get_shout_sprite_filename(shout_type: ShoutType) -> String {
    let shout_sprite_filename: String = match shout_type {
        ShoutType::Objection => {
            "objection.png"
        },
        ShoutType::HoldIt => {
            "hold_it.png"
        },
        ShoutType::NaN => {
            // add an "ERROR!" shout sprite
            panic!("Shout Type is NaN!");
        }
    }.to_string();

    shout_sprite_filename
}

pub fn get_shout_audio(shout_type: ShoutType) -> Sound {
    let game_assets = storage::get::<GameAssets>();
    let shout_audio = match shout_type {
        ShoutType::Objection => &game_assets.snd_ph_objection,
        ShoutType::HoldIt => &game_assets.snd_ph_hold_it,
        ShoutType::NaN => panic!("Shout Type is NaN!")
    };

    shout_audio.clone()
}

pub fn get_shout_type_from_str(shout_type: String) -> ShoutType {
    // TODO: comparing strings is SLOW! WE DON'T WANT SLOW!
    match shout_type {
        val if val == "objection".to_string() => {
            ShoutType::Objection
        },
        val if val == "hold_it".to_string() => {
            ShoutType::HoldIt
        },
        _ => {
            ShoutType::NaN
        }
    }
}