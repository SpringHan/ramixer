// UI

mod volume_bar;
mod sound_blocking;

use ratatui::{
    widgets::{Block, BorderType, Borders},
    layout::{Constraint, Layout, Rect},
    Frame,
};

use crate::{ui::volume_bar::VolumeBar, utils::AMixer};

pub fn ui(frame: &mut Frame, mixer: &mut AMixer) {
    let chunks = Layout::vertical([
        Constraint::Min(4),
        Constraint::Percentage(100)
    ])
        .split(frame.area());

    sound_blocking::render_sound_blocking(frame, mixer, chunks[0]);

    // Render volume bar
    let volume_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Volume Bar");

    let bar = VolumeBar::new(mixer.volume());
    let target_area = volume_block.inner(chunks[1]);
    frame.render_widget(volume_block, chunks[1]);
    frame.render_widget(bar, Rect::new(
        target_area.x + (target_area.width / 2) as u16 - 2,
        target_area.y,
        4,
        target_area.height
    ));
}
