mod ui;
mod cli;
mod utils;
mod errors;
mod notify;
mod handle_event;

use std::time::Duration;

use ratatui::crossterm::event;

use crate::utils::AMixer;

fn main() -> errors::AppResult<()> {
    match cli::parse() {
        cli::Action::Tui => run_tui(),
        cli::Action::Change { steps, increase } => change_volume(steps, increase),
    }
}

/// Change the volume once and notify the user, without opening the TUI.
fn change_volume(steps: u16, increase: bool) -> errors::AppResult<()> {
    let mut mixer = AMixer::volume_only()?;
    mixer.change_volume(steps, increase)?;
    let volume = mixer.volume();

    println!(
        "Volume {} by {}%, now at {}%",
        if increase { "increased" } else { "decreased" },
        steps,
        volume
    );

    // The volume change itself already succeeded, so a missing or broken
    // notification daemon should only warn instead of failing the command.
    if let Err(err) = notify::volume_changed(volume) {
        eprintln!("ramixer: failed to send notification: {err}");
    }

    Ok(())
}

/// Open the interactive TUI.
fn run_tui() -> errors::AppResult<()> {
    let mut mixer = AMixer::new()?;
    let mut terminal = ratatui::init();

    loop {
        if let Err(err) = terminal.draw(|frame| ui::ui(frame, &mut mixer)) {
            ratatui::restore();
            return Err(err.into());
        }

        if event::poll(Duration::from_millis(400))? {
            if let event::Event::Key(key) = event::read()? {
                if let event::KeyEventKind::Press = key.kind {
                    if key.code == event::KeyCode::Char('q') {
                        break;
                    }

                    if let Err(err) = handle_event::handle(&mut mixer, key) {
                        ratatui::restore();
                        return Err(err)
                    }
                }
            }
        }
    }

    ratatui::restore();
    Ok(())
}
