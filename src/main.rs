pub mod config;
pub mod types;
pub mod helpers;
pub mod objects;
pub mod gameloop;
pub mod global_keybinds;
pub mod debug;
pub mod importers;
pub mod assets;

use macroquad::{audio::{play_sound, PlaySoundParams}, experimental::collections::storage, prelude::*};
use types::*;
use config::*;
use gameloop::*;
use objects::textbox::*;
use importers::data_importer::import_data;
use assets::fs::*;

use crate::{assets::GameAssets, helpers::get_Icon};


const STORY_DATA_PATH: &str = "playground.json"; // should I really use a json?

fn window_conf() -> Conf {
    let icon = get_Icon();

    Conf { 
        window_title: "Aced Attorney Engine".to_owned(),
        icon: Some(icon),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    
    let mut text_id: usize = 0;
    let textbox_data: TextboxData = import_data(get_data(STORY_DATA_PATH).as_str());

    storage::store(GameAssets::load().await);
    let game_assets= storage::get::<GameAssets>();
    
    play_sound(&game_assets.bgm, PlaySoundParams { looped: true, volume: 0.7 });

    loop {
        clear_background(PRIMARY_COLOR);

        // initialize a texture
        update_speaker_emotion_texture(textbox_data.get_entry(text_id).speaker_emotion, &game_assets.speaker_tex).await;
        textbox_loop(&mut text_id, &textbox_data).await;

        if text_id + 1 > textbox_data.get_entries_count() {
            /* TODO
            We shouldn't always just "Stop" the game when we're out of textbox data.
            Sometimes in the future, the game should be able to start new textbox data
            when done with previous.
            */
            println!("Out of textbox data entries. breaking out of game loop");
            text_id = 0; // reset cuz why not
            // break;
        }

        global_keybinds::handle_global_keybinds();

        next_frame().await
    }
}