use crate::types::{TextboxData, TextboxDataEntry, SpeakerEmotion, SpeakerType};

pub fn normal_texbox_from_strings(text_entries: &Vec<&'static str>) -> TextboxData {
    let mut new_textbox_data = TextboxData {
        data: vec![]
    };
    for text_entry in text_entries {
        new_textbox_data.data.push(
            TextboxDataEntry {
                text: text_entry.to_string(),
                speaker_emotion: SpeakerEmotion::Normal,
                speaker_type: SpeakerType::Defence
            }
        );
    }

    new_textbox_data
}