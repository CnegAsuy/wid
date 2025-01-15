use std::{cell::RefCell, rc::Rc};

use crate::{controller::*, widgets::centered_rect};
use chrono::Local;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Terminal,
};

enum MODE {
    DAILY,
    WEEKLY,
}

use crossterm::event::{self, Event, KeyCode, KeyModifiers};

pub fn draw_tui(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> std::io::Result<()> {
    let mut show_menu = false;
    let mut x: u16 = 0;
    let mut choice_at_menu: u8 = 0;
    let time = Local::now();
    let mut hztl = true;
    let mut mode: MODE = MODE::DAILY;
    // Main event loop
    Ok(loop {
        // Check for user input
        if event::poll(std::time::Duration::from_millis(500)).expect("Can't wait the app") {
            if let Event::Key(key_event) = event::read().expect("can't read keys") {
                // Exit on Ctrl+C
                if (key_event.code == KeyCode::Char('c')
                && key_event.modifiers.contains(KeyModifiers::CONTROL))
                || key_event.code == KeyCode::Char('q')
                {
                    break;
                }
                if key_event.code == KeyCode::Char('h') {
                    if x == 30 {
                        x = 0
                    } else {
                        x = 30
                    }
                }
                if key_event.code == KeyCode::Esc {
                    show_menu = !show_menu;
                }

                //if key_event.code == KeyCode::Right {
                //    increase_day(choosen_day.clone());
                //}
                
                //if key_event.code == KeyCode::Right {
                //    decrease_day(choosen_day.clone());
                //}
                
                if show_menu {
                    match key_event.code {
                        KeyCode::Up => {
                            choice_at_menu = (choice_at_menu + 4) % 5;
                        }
                        KeyCode::Down => {
                            choice_at_menu = (choice_at_menu + 1) % 5;
                        }
                        KeyCode::Enter => {
                            match choice_at_menu {
                                4 => break, // Exit program
                                _ => {}
                            }
                        }
                        _ => {}
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
                        Constraint::Length(5),
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
                .constraints([Constraint::Percentage(100 - x), Constraint::Percentage(x)].as_ref())
                .split(chunks[1]);
            
            let choosen_day = Rc::new(RefCell::new(time.format("%Y-%m-%d").to_string()));
            let app = String::from("kitty");
            let day = &(choosen_day.clone().take());
            let main_list = daily_gui(&day, &app, hztl);

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
                    rect.render_widget(main_list, main[0]);
                    rect.render_widget(h, main[1]);
                    
                    if show_menu {
                        // Create a centered rect for the menu
                        let menu_rect = centered_rect(50, size);
                        let menu_items = menu(choice_at_menu);
                        // Menu widget
                        let menu = List::new(menu_items)
                        .block(Block::default().style(Style::default().fg(Color::Rgb(132, 172, 254))))
                        .style(Style::default().fg(Color::White).bg(Color::Rgb(64, 64, 64)));
                    
                    rect.render_widget(menu, menu_rect);
                }
            })?;
        })
    }
    
    fn daily_gui<'a>(day: &String, app: &String, hztl: bool) -> List<'a> {
        let main_list = vec![
        ListItem::new(app_graph(app, day, hztl)),
        ListItem::new(format!("Application Usage: {}", app_usage(app, day))),
        ListItem::new(power_graph(day, hztl)),
        ListItem::new(format!("Power Usage: {}", power_usage(day))),
        ];
        
        let x = List::new(main_list)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White));
    x
}

fn choosen(num: u8, choice_at_menu: u8) -> Style {
    if choice_at_menu == num {
        Style::default().fg(Color::Black).bg(Color::White)
    } else {
        Style::default().fg(Color::White).bg(Color::Rgb(64, 64, 64))
    }
}

fn menu<'a>(choice_at_menu: u8) -> Vec<ListItem<'a>> {
    vec![
        ListItem::new(
            "
            |   |   |  o     |
            |   |   |      __|
            |   |   |  |  /  |
            \\_/ \\_/   |_/\\_/|_/
            ",
        )
        .style(Style::default().fg(Color::White).bg(Color::Rgb(64, 64, 64))),
        ListItem::new("    Add an app that track                          ")
            .style(choosen(1, choice_at_menu)),
        ListItem::new("    My Usage                                       ")
            .style(choosen(2, choice_at_menu)),
            ListItem::new("                                                   ")
            .style(choosen(3, choice_at_menu)),
            ListItem::new("    Settings                                       ")
            .style(choosen(4, choice_at_menu)),
            ListItem::new("    Exit                                           ")
            .style(choosen(5, choice_at_menu)),
            ListItem::new("                                                   ")
            .style(Style::default().fg(Color::White).bg(Color::Rgb(64, 64, 64))),
    ]
}
