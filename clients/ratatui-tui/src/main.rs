use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

mod state;
mod ui;

use state::AppState;
use ui::render_ui;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = AppState::new();

    // Main loop
    let result = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut AppState,
) -> Result<()> {
    loop {
        terminal.draw(|f| render_ui(f, app))?;

        // Only wait for events, don't poll continuously
        if let Event::Key(key) = event::read()? {
            // Only process KeyCode::Char on Press events (not Release or Repeat)
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char(c) => app.input.push(c),
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Enter => {
                        let cmd = app.input.clone();
                        app.messages.push(format!("You typed: {}", cmd));
                        app.input.clear();

                        // TODO: Send command to SpacetimeDB
                    }
                    _ => {}
                }
            }
        }
    }
}
