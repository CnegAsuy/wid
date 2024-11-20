use crate::widgets::centered_rect;
use tui::{
    backend::CrosstermBackend, 
        layout::{Alignment, Constraint, Direction, Layout}, 
        style::{Color, Style}, widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
        Terminal,
};

use crossterm::event::{self, Event, KeyCode, KeyModifiers};

pub fn draw_tui(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> std::io::Result<()> {
    let mut show_menu = false;
    let mut x: u16 = 0;
    let mut choice_at_menu: u8 = 0;
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

                if show_menu {
                    if key_event.code == KeyCode::Up {
                        if  choice_at_menu != 0 {
                            choice_at_menu -= 1;
                        } else {
                            choice_at_menu = 4
                        }
                    }

                    if key_event.code == KeyCode::Down {
                        if choice_at_menu == 4 {
                            choice_at_menu = 0;
                        } else {
                            choice_at_menu += 1;
                        }
                    }

                    if key_event.code == KeyCode::Enter {
                        match choice_at_menu {
                            4_u8 => {
                                break;
                            },
                            _ => {}
                        }
                    }
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
                        .border_type(BorderType::Rounded),
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
                        .border_type(BorderType::Rounded),
                );

            let h = Paragraph::new("Help Menu\n\n<q>: Exit Program\n<h>: Close help menu")
                .style(Style::default().fg(Color::Green))
                .alignment(Alignment::Left)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .border_type(BorderType::Rounded),
                );

            let copyright = Paragraph::new("MIT Licence\n\nPress <h> to help.")
            .style(Style::default().fg(Color::LightCyan))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .border_type(BorderType::Rounded),
        );

            rect.render_widget(title, chunks[0]);
            rect.render_widget(copyright, chunks[2]);
            rect.render_widget(p, main[0]);
            rect.render_widget(h, main[1]);

            if show_menu {
                // Create a centered rect for the menu
                let menu_rect = centered_rect(50, 45, size);
                let menu_items = Vec::from(vec![
                    ListItem::new("
                |   |   |  o     |  
                |   |   |      __|  
                |   |   |  |  /  |  
                 \\_/ \\_/   |_/\\_/|_/
                ")
                    .style(Style::default().fg(Color::White).bg(Color::Rgb(64, 64, 64))),
                    ListItem::new("    Add an app that track")
                    .style(if choice_at_menu == 0 {Style::default().fg(Color::Black).bg(Color::White)} else {Style::default().fg(Color::White).bg(Color::Rgb(64, 64, 64))}),
                    ListItem::new("    My Usage")
                    .style(if choice_at_menu == 1 {Style::default().fg(Color::Black).bg(Color::White)} else {Style::default().fg(Color::White).bg(Color::Rgb(64, 64, 64))}),
                    ListItem::new("    Add an app that track")
                    .style(if choice_at_menu == 2 {Style::default().fg(Color::Black).bg(Color::White)} else {Style::default().fg(Color::White).bg(Color::Rgb(64, 64, 64))}),
                    ListItem::new("    Settings")
                    .style(if choice_at_menu == 3 {Style::default().fg(Color::Black).bg(Color::White)} else {Style::default().fg(Color::White).bg(Color::Rgb(64, 64, 64))}),
                    ListItem::new("    Exit")
                    .style(if choice_at_menu == 4 {Style::default().fg(Color::Black).bg(Color::White)} else {Style::default().fg(Color::White).bg(Color::Rgb(64, 64, 64))}),
                ]);
                // Menu widget
                let menu = List::new(menu_items)
                    .block(
                        Block::default()
                            .style(Style::default().fg(Color::Rgb(132, 172, 254))),
                    )
                    .style(Style::default().fg(Color::White));
    
                rect.render_widget(menu, menu_rect);
            }
        })?;
    })
}