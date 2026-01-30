use crate::state::AppState;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_ui(f: &mut Frame, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(80), // Main buffer
            Constraint::Percentage(10), // Gauges
            Constraint::Percentage(10), // Input
        ])
        .split(f.size());

    // Main text buffer - show only the last N messages that fit
    let available_height = chunks[0].height.saturating_sub(2) as usize; // Subtract borders
    let start_index = app.messages.len().saturating_sub(available_height);

    let lines: Vec<Line> = app
        .messages
        .iter()
        .skip(start_index) // Skip old messages to show only recent ones
        .map(|msg| Line::from(msg.as_str()))
        .collect();

    let buffer =
        Paragraph::new(lines).block(Block::default().title("DOGMUD").borders(Borders::ALL));
    f.render_widget(buffer, chunks[0]);

    // HP/Stamina gauges (rest stays the same)
    let gauges_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let hp_ratio = app.hp as f64 / app.max_hp as f64;
    let hp_gauge = ratatui::widgets::Gauge::default()
        .block(Block::default().title("HP").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Red))
        .ratio(hp_ratio);
    f.render_widget(hp_gauge, gauges_layout[0]);

    let stamina_ratio = app.stamina as f64 / app.max_stamina as f64;
    let stamina_gauge = ratatui::widgets::Gauge::default()
        .block(Block::default().title("Stamina").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Yellow))
        .ratio(stamina_ratio);
    f.render_widget(stamina_gauge, gauges_layout[1]);

    // Input
    let input =
        Paragraph::new(format!("> {}", app.input)).block(Block::default().borders(Borders::ALL));
    f.render_widget(input, chunks[2]);
}
