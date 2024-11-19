use crate::widgets::centered_rect;
use tui::{
    backend::CrosstermBackend, 
        layout::{Alignment, Constraint, Direction, Layout}, 
        style::{Color, Style}, widgets::{Block, BorderType, Borders, Paragraph},
        Terminal,
};

use crossterm::event::{self, Event, KeyCode, KeyModifiers};

pub fn draw_tui(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> std::io::Result<()> {
    let mut show_menu = false;
    let mut x: u16 = 0;
    // Main event loop
    Ok(loop {
        // Check for user input
        if event::poll(std::time::Duration::from_millis(500)).expect("Can't wait the app") {
            if let Event::Key(key_event) = event::read().expect("can't read keys") {
                // Exit on Ctrl+C
                if (key_event.code == KeyCode::Char('c') && key_event.modifiers.contains(KeyModifiers::CONTROL)) || key_event.code == KeyCode::Char('q') {
                    break;
                }
                if key_event.code == KeyCode::Char('h') {
                    if x == 30 {x = 0} else {x = 30}
                }
                if key_event.code == KeyCode::Esc {
                    show_menu = !show_menu;
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
                        Constraint::Min(2),
                        Constraint::Length(6),
                    ]
                    .as_ref(),
                )
                .split(size);

            let title = Paragraph::new("wid-cli")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .border_type(BorderType::Plain),
            );


            let main = Layout::default()
                .direction(Direction::Horizontal)
                .margin(0)
                .constraints(
                    [
                        Constraint::Percentage(100 - x),
                        Constraint::Percentage(x),
                    ]
                    .as_ref()
                )
                .split(chunks[1]);

            let p = Paragraph::new("Hello")
                .style(Style::default().fg(Color::Green))
                .alignment(Alignment::Left)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .border_type(BorderType::Plain),
                );

            let h = Paragraph::new("Help Menu\n\n<q>: Exit Program\n<h>: Close help menu")
                .style(Style::default().fg(Color::Green))
                .alignment(Alignment::Left)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .border_type(BorderType::Plain),
                );

            let copyright = Paragraph::new("All Right Reserved © CnegAsuy\n\n Press <h> to help.")
            .style(Style::default().fg(Color::LightCyan))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .border_type(BorderType::Plain),
        );

            rect.render_widget(title, chunks[0]);
            rect.render_widget(copyright, chunks[2]);
            rect.render_widget(p, main[0]);
            rect.render_widget(h, main[1]);

            if show_menu {
                // Create a centered rect for the menu
                let menu_rect = centered_rect(50, 45, size);
    
                // Menu widget
                let menu = Paragraph::new("Menu\n[1] Option 1\n[2] Option 2\n[Esc] Close")
                    .block(
                        Block::default()
                            .title("Menu")
                            .borders(Borders::ALL)
                            .style(Style::default().fg(Color::Rgb(132, 172, 224))),
                    )
                    .style(Style::default().fg(Color::White))
                    .alignment(Alignment::Center);
    
                rect.render_widget(menu, menu_rect);
            }
        })?;
    })
}