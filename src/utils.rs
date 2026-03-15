// Amixer command utils

use std::process::Command;

use crate::errors::{AppError, AppResult};

#[derive(Clone, Copy)]
pub enum ControlPart {
    Speaker,
    Headphone
}

#[derive(Debug, Clone, Copy)]
pub struct AMixer {
    current_volume: i32,
}

impl AMixer {
    pub fn new() -> AppResult<Self> {
        Ok(Self {
            current_volume: Self::get_master_volume()?
        })
    }

    pub fn get_master_volume() -> AppResult<i32> {
        let volume_value = Command::new("sh")
            .arg("-c")
            .arg("amixer get Master | grep -oP '\\d+(?=%)' | head -1")
            .output()?;

        if !volume_value.status.success() {
            return Err(AppError::Custom(String::from(
                "Failed to get current volume!"
            )));
        }

        Ok(String::from_utf8(volume_value.stdout)?.parse()?)
    }

    pub fn change_volume(steps: i32) -> AppResult<()> {
        Ok(())
    }

    pub fn mute_or_unmute(part: ControlPart) -> AppResult<()> {
        Ok(())
    }
}
