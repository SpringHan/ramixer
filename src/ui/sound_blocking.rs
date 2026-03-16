// Sound blocking manager

use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders},
};

use crate::utils::{AMixer, ControlPart};

pub fn render_sound_blocking(frame: &mut Frame, mixer: &AMixer, area: Rect) {
    let sound_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Sound Blocking Manager");

    let text = Text::from(vec![
        generate_line(mixer, ControlPart::Speaker),
        generate_line(mixer, ControlPart::Headphone)
    ]);

    frame.render_widget(text, sound_block.inner(area));
    frame.render_widget(sound_block, area);
}

fn generate_line(mixer: &AMixer, part: ControlPart) -> Line<'_> {
    let status_span = Span::raw(format!(
        "[{}] ",
        if part == ControlPart::Speaker {
            blocking_span(mixer.speaker_mute)
        } else {
            blocking_span(mixer.headphone_mute)
        }
    ))
        .style(Style::default().green());

    let name_span = Span::raw(if part == ControlPart::Speaker {
        "Speaker"
    } else {
        "Headphone"
    }).bold();

    Line::from(vec![status_span, name_span])
}

fn blocking_span(blocking: bool) -> &'static str {
    if blocking {
        "M"
    } else {
        " "
    }
}
