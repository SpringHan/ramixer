// Desktop notification utils, based on `notify-send`

use std::process::Command;

use crate::errors::{AppError, AppResult};

/// Application name shown by the notification daemon.
const APP_NAME: &str = "RAmixer";

/// Arguments passed to `notify-send`.
///
/// The body is empty so that the notification stays a single line, and the
/// `value` hint is what mako reads to draw its native progress bar.
fn command_args(content: &str, progress: u16) -> Vec<String> {
    vec![
        String::from("--app-name"),
        String::from(APP_NAME),
        String::from("--hint"),
        String::from("string:x-canonical-private-synchronous:ramixer-volume"),
        String::from("--hint"),
        format!("int:value:{progress}"),
        String::from(content),
        String::new()
    ]
}

/// Send a desktop notification via `notify-send`.
pub fn send(content: &str, progress: u16) -> AppResult<()> {
    let output = Command::new("notify-send")
        .args(command_args(content, progress))
        .output()?;

    if !output.status.success() {
        return Err(AppError::Custom(format!(
            "notify-send failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }

    Ok(())
}

/// Notify the user about the volume after the change.
///
/// The content is the volume itself; the progress bar is drawn by mako from
/// the `value` hint.
pub fn volume_changed(volume: u16) -> AppResult<()> {
    let volume = volume.min(100);

    send(&format!("{volume}%"), volume)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_is_passed_as_the_mako_value_hint() {
        let args = command_args("52%", 52);

        assert_eq!(args[2], "--hint");
        assert_eq!(args[3], "int:value:52");
    }

    #[test]
    fn body_is_empty_so_that_only_the_title_line_is_shown() {
        let args = command_args("52%", 52);

        assert_eq!(args[4], "52%");
        assert_eq!(args[5], "");
    }
}
