use ratatui::{
    prelude::*,
    widgets::{canvas::*, *},
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.size());

    let right_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout[1]);

    frame.render_widget(app.map_canvas(), main_layout[0]);
    frame.render_widget(app.pong_canvas(), right_layout[0]);
    frame.render_widget(app.boxes_canvas(right_layout[1]), right_layout[1]);
    // This is where you add new widgets.
}
