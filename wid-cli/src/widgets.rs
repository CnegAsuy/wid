use tui::layout::{Constraint, Direction, Layout, Rect};

pub(crate) fn centered_rect(percent_width: u16, parent_rect: Rect) -> Rect {
    // Calculate padding dynamically based on available space
    let top_padding = (parent_rect.height / 8).max(3); // Minimum 3, but scales with height
    let bottom_padding = (parent_rect.height / 8).max(3); // Minimum 3, scales as well

    // Ensure that the menu height fits within the available space
    let menu_height = 12.min(parent_rect.height - top_padding - bottom_padding);

    // Split into vertical sections (top, menu, bottom)
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(top_padding),  // Top padding
                Constraint::Length(menu_height), // Menu height
                Constraint::Length(bottom_padding), // Bottom padding
            ]
            .as_ref(),
        )
        .split(parent_rect);

    // Split the menu horizontally with the specified percentage width
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_width) / 2), // Left padding
                Constraint::Percentage(percent_width),            // Menu width
                Constraint::Percentage((100 - percent_width) / 2), // Right padding
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
