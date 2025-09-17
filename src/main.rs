pub mod helpers;
pub mod objects;
pub mod gameloop;
pub mod types;

use macroquad::prelude::*;
use helpers::*;
use types::*;
use gameloop::*;
use objects::textbox::*;


#[macroquad::main("Aced Attorney Engine")]
async fn main() {
    let mut text_id: usize = 0;
    
    let textbox_data = TextboxData {
        data: vec![
            TextboxDataEntry { text: "Your honor,".to_string(), speaker_emotion: SpeakerEmotion::Normal },
            TextboxDataEntry { text: "cry about it.".to_string(), speaker_emotion: SpeakerEmotion::Objection }
        ]
    };

    // let bgm = load_sound("assets/objection_2002.ogg").await.unwrap();
    // play_sound(&bgm, PlaySoundParams { looped: true, volume: 1.0 });

    loop { 
        clear_background(WHITE);

        // initialize a texture
        let speaker_texture = get_texture("assets/pob/normal.png").await;
        update_speaker_emotion_texture(textbox_data.get_entry(text_id).speaker_emotion, &speaker_texture).await;
        textbox_loop(&mut text_id, &textbox_data);

        if text_id + 1 > textbox_data.get_entries_count() {
            /* TODO
            We shouldn't always just "Stop" the game when we're out of textbox data.
            Sometimes in the future, the game should be able to start new textbox data
            when done with previous.
            */
            println!("Out of textbox data entries. breaking out of game loop");
            break;
        }

        next_frame().await
    }
}