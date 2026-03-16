// Event handle

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{errors::AppResult, utils::{AMixer, ControlPart}};

pub fn handle(mixer: &mut AMixer, key: KeyEvent) -> AppResult<()> {
    match key.code {
        KeyCode::Up | KeyCode::Char('=') => {
            mixer.change_volume(
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    5
                } else {
                    10    
                },
                true
            )?;
        },

        KeyCode::Down | KeyCode::Char('-') => {
            mixer.change_volume(
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    5
                } else {
                    10    
                },
                false
            )?;
        },

        KeyCode::Char(c) => {
            match c {
                '+' => mixer.change_volume(5, true)?,
                '_' => mixer.change_volume(5, false)?,
                'm' => mixer.mute_or_unmute(ControlPart::Headphone)?,
                'M' => mixer.mute_or_unmute(ControlPart::Speaker)?,
                
                _ => ()
            }
        },

        _ => ()
    }

    Ok(())
}
