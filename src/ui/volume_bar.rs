// Volume bar

use ratatui::{
    widgets::{Block, Borders, Widget},
    layout::{Constraint, Layout},
    style::{Color, Stylize},
    text::Line,
};

#[derive(Clone)]
pub struct VolumeBar {
    volume: u16
}

impl VolumeBar {
    pub fn new(volume: u16) -> Self {
        Self { volume }
    }
}

impl Widget for VolumeBar {
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer
    ) {
        let layouts = Layout::vertical([
            Constraint::Percentage(100),
            Constraint::Min(1)
        ])
            .split(area);

        // Draw column bar border
        let block = Block::new()
            .borders(Borders::ALL);
        let inner_area = block.inner(layouts[0]);

        block.render(layouts[0], buf);
        
        // Draw percentage text
        let volume_line = Line::raw(if self.volume == 100 {
            String::from("100")
        } else if self.volume >= 10 {
            format!(" {}", self.volume)
        } else {
            format!("  {}", self.volume)
        }).bold();

        volume_line.render(layouts[1], buf);

        // Draw column bar
        if self.volume == 0 {
            return;
        }

        let volume_to_height = (inner_area.height * self.volume / 100) as usize;
        for (index, pos) in (inner_area.y..(inner_area.y + inner_area.height))
            .rev()
            .enumerate()
        {
            if index > volume_to_height {
                break;
            }

            buf[(inner_area.x, pos)].set_bg(Color::LightGreen);
            buf[(inner_area.x + 1, pos)].set_bg(Color::LightGreen);
        }
    }
}
