mod utils;
mod errors;

use std::time::Duration;

use ratatui::crossterm::event;

fn main() -> errors::AppResult<()> {
    let mut terminal = ratatui::init();

    loop {
        if event::poll(Duration::from_millis(400))? {
            if let event::Event::Key(key) = event::read()? {
                if let event::KeyEventKind::Press = key.kind {
                    if key.code == event::KeyCode::Char('q') {
                        break;
                    }
                }
            }
        }
    }

    ratatui::restore();
    Ok(())
}
