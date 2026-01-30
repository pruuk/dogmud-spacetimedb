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

use std::process::Command;

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

    // Auto-look on startup
    app.messages.push("".to_string()); // Blank line
    if let Err(e) = handle_command("look", &mut app).await {
        app.messages
            .push(format!("Failed to get initial location: {}", e));
    }

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

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char(c) => app.input.push(c),
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Enter => {
                        let cmd = app.input.clone();
                        app.messages.push(format!("> {}", cmd));
                        app.input.clear();

                        // Parse and handle command
                        handle_command(&cmd, app).await?;
                    }
                    _ => {}
                }
            }
        }
    }
}

async fn handle_command(cmd: &str, app: &mut AppState) -> Result<()> {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(());
    }

    match parts[0] {
        "move" | "go" => {
            if parts.len() < 2 {
                app.messages.push("Usage: move <direction>".to_string());
                return Ok(());
            }
            let direction = parts[1];

            let output = Command::new("spacetime")
                .args(&[
                    "call",
                    "--server",
                    "http://localhost:3000",
                    "dogmud",
                    "move_player",
                    direction,
                ])
                .output()?;

            if output.status.success() {
                app.messages.push(format!("You moved {}", direction));
                app.messages.push("".to_string());

                // Auto-look after moving
                do_look(app).await?;
            } else {
                let err = String::from_utf8_lossy(&output.stderr);
                if err.contains("Error:") {
                    if let Some(start) = err.find("Error: Response text: ") {
                        let error_msg = &err[start + 22..];
                        if let Some(end) = error_msg.find('\n') {
                            app.messages.push(format!("✗ {}", &error_msg[..end]));
                        } else {
                            app.messages.push(format!("✗ {}", error_msg));
                        }
                    } else {
                        app.messages.push("Movement failed".to_string());
                    }
                }
            }
        }
        "attack" => {
            if parts.len() < 2 {
                app.messages.push("Usage: attack <target_id>".to_string());
                return Ok(());
            }
            let target_id = parts[1];

            let output = Command::new("spacetime")
                .args(&[
                    "call",
                    "--server",
                    "http://localhost:3000",
                    "dogmud",
                    "attack",
                    target_id,
                ])
                .output()?;

            if output.status.success() {
                app.messages.push("You attack!".to_string());
            } else {
                let err = String::from_utf8_lossy(&output.stderr);
                if err.contains("Error:") {
                    app.messages.push(format!("Attack failed: {}", err));
                } else {
                    app.messages.push("You attack!".to_string());
                }
            }
        }
        "look" | "l" => {
            do_look(app).await?;
        }
        "help" => {
            app.messages.push("Commands:".to_string());
            app.messages
                .push("  move/go <direction> (north/south/east/west)".to_string());
            app.messages.push("  attack <target_id>".to_string());
            app.messages
                .push("  look/l - describe current room".to_string());
            app.messages.push("  help - show this message".to_string());
            app.messages.push("  q - quit".to_string());
        }
        _ => {
            app.messages.push(format!(
                "Unknown command: {}. Type 'help' for commands.",
                parts[0]
            ));
        }
    }

    Ok(())
}

// Separate function to avoid recursion issues
async fn do_look(app: &mut AppState) -> Result<()> {
    let output = Command::new("spacetime")
        .args(&[
            "call",
            "--server",
            "http://localhost:3000",
            "dogmud",
            "look",
        ])
        .output()?;

    if output.status.success() {
        let logs = Command::new("spacetime")
            .args(&["logs", "--server", "http://localhost:3000", "dogmud"])
            .output()?;

        if logs.status.success() {
            let log_str = String::from_utf8_lossy(&logs.stdout);
            let lines: Vec<&str> = log_str.lines().collect();

            let mut start_idx = None;
            let mut end_idx = None;

            for (i, line) in lines.iter().enumerate().rev() {
                if end_idx.is_none() && line.contains("<<<LOOK_END>>>") {
                    end_idx = Some(i);
                }
                if end_idx.is_some() && line.contains("<<<LOOK_START>>>") {
                    start_idx = Some(i);
                    break;
                }
            }

            if let (Some(start), Some(end)) = (start_idx, end_idx) {
                for line in &lines[start..=end] {
                    if line.contains("<<<LOOK_START>>>") || line.contains("<<<LOOK_END>>>") {
                        continue;
                    }

                    if let Some(pos) = line.find("look server") {
                        if let Some(msg_start) = line[pos..].find(": ") {
                            let message = &line[pos + msg_start + 2..];
                            if !message.is_empty() {
                                app.messages.push(message.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
