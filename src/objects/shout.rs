// Objection, Hold It, Yes... Those popups
use macroquad::{prelude::Image, experimental::collections::storage};
use crate::{assets::{fs::get_image, GameAssets}, types::ShoutType};

// pub static shout_is_visible: bool = false;

pub async fn update_shout_tex(shout_type: ShoutType) {
    let shout_filename: String = get_shout_sprite_filename(shout_type);
    let new_shout_tex: Image = get_image(shout_filename.as_str()).await;
    let shout_tex = &storage::get::<GameAssets>().shouting_tex;
    shout_tex.update(&new_shout_tex);
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