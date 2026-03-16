mod ui;
mod utils;
mod errors;
mod handle_event;

use std::time::Duration;

use ratatui::crossterm::event;

use crate::utils::AMixer;

fn main() -> errors::AppResult<()> {
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
