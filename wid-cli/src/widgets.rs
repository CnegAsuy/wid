use tui::layout::{Constraint, Direction, Layout, Rect};

pub(crate) fn centered_rect(percent_x: u16, percent_y: u16, rect: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2), // Top padding
                Constraint::Percentage(percent_y),            // Menu height
                Constraint::Percentage((100 - percent_y) / 2), // Bottom padding
            ]
            .as_ref(),
        )
        .split(rect);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2), // Left padding
                Constraint::Percentage(percent_x),            // Menu width
                Constraint::Percentage((100 - percent_x) / 2), // Right padding
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
