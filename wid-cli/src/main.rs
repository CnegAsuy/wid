use std::io::{self};

use tui::{
    backend::CrosstermBackend,
    Terminal,
};
use crossterm::{
    cursor,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

mod draw_tui;
mod widgets;

use draw_tui::draw_tui;

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
    
    let _ = draw_tui(&mut terminal);
    // Cleanup terminal before exiting
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, cursor::Show)?;
    terminal.clear()?;

    Ok(())
}