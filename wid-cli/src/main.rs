use std::{io, thread, time};

use tui::{backend::CrosstermBackend, layout::{Alignment, Constraint, Direction, Layout}, style::{Color, Style}, widgets::{Block, BorderType, Borders, Paragraph}, Terminal};

fn main() {
    loop {
        thread::sleep(time::Duration::from_millis(500));
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).expect("Something went wrong while defining terminal.");
        terminal.clear().expect("Something went wrong while creating tui");
        let _ = terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

        let copyright = Paragraph::new( "Hello, World!")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Copyright")
                .border_type(BorderType::Plain),
        );

        rect.render_widget(copyright, chunks[2]);
        });
    }
}
