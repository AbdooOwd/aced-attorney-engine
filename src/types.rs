use macroquad::prelude::error;

#[derive(Clone, Copy)]
pub enum SpeakerEmotion {
    NaN, // used for error handling or null values or anything else
    Normal,
    Exclamation,
    Thinking,
    Objection
}

// TODO: could also be called ViewType, or CamView, or idk
#[derive(Clone, Copy)]
pub enum SpeakerType {
    Defence,
    DefenceAssistant,
    Prosecution,
    Judge,
    Witness,
}


/**
 * TODO: Migrate this to `objects/textbox`
 */
#[derive(Clone)]
pub struct TextboxDataEntry {
    pub text: String,
    pub speaker_emotion: SpeakerEmotion,
    pub speaker_type: SpeakerType
}

pub struct TextboxData {
    pub data: Vec<TextboxDataEntry>,
}

impl TextboxData {
    pub fn can_accept_text_id(&self, textbox_id: usize) -> bool {
        self.get_entries_count() >= textbox_id + 1
    }

    pub fn get_entry(&self, textbox_id: usize) -> &TextboxDataEntry {
        if self.get_entries_count() < 1 {
            error!("No Textbox Data Entry found! (meaning this case or story is empty!)");
        }

        if self.can_accept_text_id(textbox_id) {
            &self.data[textbox_id]
        } else {
            println!("Error! Couldn't find textbox data for id {}", textbox_id);
            &self.data[self.get_entries_count() - 1] // get last entry
        }
    }

    pub fn get_entries_count(&self) -> usize {
        self.data.len()
    }

    pub fn get_emotion(&self, textbox_id: usize) -> SpeakerEmotion {
        if self.can_accept_text_id(textbox_id) {
            self.data[textbox_id].speaker_emotion
        } else {
            SpeakerEmotion::NaN
        }
    }
}