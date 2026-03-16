// Amixer command utils

use std::process::Command;

use crate::errors::{AppError, AppResult};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlPart {
    Speaker,
    Headphone
}

#[derive(Debug, Clone, Copy)]
pub struct AMixer {
    current_volume: u16,

    pub speaker_mute: bool,
    pub headphone_mute: bool,
}

impl AMixer {
    pub fn new() -> AppResult<Self> {
        Ok(Self {
            current_volume: Self::get_volume_after_exec(None)?,
            speaker_mute: Self::sound_blocking_status(ControlPart::Speaker)?,
            headphone_mute: Self::sound_blocking_status(ControlPart::Headphone)?,
        })
    }

    pub fn volume(&self) -> u16 {
        self.current_volume
    }

    pub fn change_volume(&mut self, steps: u16, increase: bool) -> AppResult<()> {
        let cmd = format!(
            "amixer -M set Master {}%{}",
            steps,
            if increase {
                "+"
            } else {
                "-"
            }
        );

        self.current_volume = Self::get_volume_after_exec(Some(cmd))?;
        Ok(())
    }

    pub fn mute_or_unmute(&mut self, part: ControlPart) -> AppResult<()> {
        let cmd = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "amixer set {} toggle | grep -oP \"(?<=\\[)(on|off)(?=\\])\" | head -1",
                if part == ControlPart::Speaker {
                    "Speaker"
                } else {
                    "Headphone"
                }
            ))
            .output()?;

        if !cmd.status.success() {
            return Err(AppError::Custom(String::from(
                "Failed to mute/unmute!"
            )));
        }

        if part == ControlPart::Speaker {
            self.speaker_mute = !self.speaker_mute;
        } else {
            self.headphone_mute = !self.headphone_mute;
        }

        Ok(())
    }

    /// Execute a `cmd`, then get current volume.
    pub fn get_volume_after_exec(cmd: Option<String>) -> AppResult<u16> {
        let volume_value = Command::new("sh")
            .arg("-c")
            .arg(if let Some(cmd) = cmd {
                format!("{} | grep -oP '\\d+(?=%)' | head -1", cmd)
            } else {
                String::from("amixer get Master | grep -oP '\\d+(?=%)' | head -1")
            })
            .output()?;

        if !volume_value.status.success() {
            return Err(AppError::Custom(String::from(
                "Failed to get/change volume!"
            )));
        }
        
        Ok(
            String::from_utf8(volume_value.stdout)?
                .trim()
                .parse()?
        )
    }

    /// Get sound blocking status, if it's muted, return true.
    pub fn sound_blocking_status(part: ControlPart) -> AppResult<bool> {
        let status = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "amixer get {} | grep -oP \"(?<=\\[)(on|off)(?=\\])\" | head -1",
                if part == ControlPart::Speaker {
                    "Speaker"
                } else {
                    "Headphone"
                }
            ))
            .output()?;

        if !status.status.success() {
            return Err(AppError::Custom(format!(
                "Failed to get blocking status of {:?}",
                part
            )));
        }

        Ok(String::from_utf8(status.stdout)?.trim() == "off")
    }

}
