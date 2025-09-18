use std::process::exit;
use macroquad::input::{is_key_pressed, KeyCode};

pub fn handle_global_keybinds() {
    if is_key_pressed(KeyCode::Escape) {
        exit(1);
    }
}