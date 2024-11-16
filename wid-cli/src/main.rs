use std::io::{self};

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Terminal,
};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> crossterm::Result<()> {
    // Enable raw mode and hide the cursor
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;

    // Setup the terminal
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    // Main event loop
    loop {
        // Check for user input
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(key_event) = event::read()? {
                // Exit on Ctrl+C
                if key_event.code == KeyCode::Char('c') && key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    break;
                }
            }
        }

        // Draw the UI
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Max(220),
                        Constraint::Length(8),
                    ]
                    .as_ref(),
                )
                .split(size);

            let menu = Paragraph::new("Hello, World!")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title("Menu")
                        .border_type(BorderType::Plain),
                );

            rect.render_widget(menu.clone(), chunks[0]);
            rect.render_widget(menu.clone(), chunks[1]);
            rect.render_widget(menu.clone(), chunks[2]);
        })?;
    }

    // Cleanup terminal before exiting
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, cursor::Show)?;
    terminal.clear()?;

    Ok(())
}
