//! Status display UI components (validation and teleport results)

use std::path::PathBuf;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::App;
use lantern_teleport_core::CurrentPosition;

/// Render validation success screen
pub fn render_validation_success(f: &mut Frame, app: &App, position: &CurrentPosition) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(10),   // Content
            Constraint::Length(3), // Footer
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Save File Valid!")
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Lantern Teleport TUI ")
                .title_style(Style::default().fg(Color::Cyan)),
        );
    f.render_widget(title, chunks[0]);

    // Content
    let content = build_validation_success_content(position, app.save_file_path.as_ref());
    let content_block = Paragraph::new(content)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(content_block, chunks[1]);

    // Footer
    let footer = Paragraph::new(Line::from(vec![
        Span::styled("Enter", Style::default().fg(Color::Cyan)),
        Span::raw(": Continue  "),
        Span::styled("Esc", Style::default().fg(Color::Cyan)),
        Span::raw(": Change file  "),
        Span::styled("q", Style::default().fg(Color::Cyan)),
        Span::raw(": Quit"),
    ]))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

/// Build validation success content
fn build_validation_success_content(
    position: &CurrentPosition,
    save_path: Option<&PathBuf>,
) -> Vec<Line<'static>> {
    let path_str = save_path
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    vec![
        Line::from(""),
        Line::from(Span::styled(
            "✓ Valid Bloodborne save file detected",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled("File: ", Style::default().fg(Color::Gray))),
        Line::from(Span::styled(path_str, Style::default().fg(Color::White))),
        Line::from(""),
        Line::from(Span::styled(
            "Current Position:",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!(
                "Map ID: {:02X}{:02X}{:02X}{:02X}",
                position.map_id[0], position.map_id[1], position.map_id[2], position.map_id[3]
            ),
            Style::default().fg(Color::White),
        )),
        Line::from(Span::styled(
            format!(
                "X: {:.2}  Y: {:.2}  Z: {:.2}",
                position.x, position.y, position.z
            ),
            Style::default().fg(Color::White),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Press Enter to select destination...",
            Style::default().fg(Color::Gray),
        )),
    ]
}

/// Render validation error screen
pub fn render_validation_error(f: &mut Frame, error: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(8),    // Content
            Constraint::Length(3), // Footer
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Validation Failed")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Lantern Teleport TUI ")
                .title_style(Style::default().fg(Color::Cyan)),
        );
    f.render_widget(title, chunks[0]);

    // Content
    let content = vec![
        Line::from(""),
        Line::from(Span::styled(
            "✗ Invalid save file",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled("Error:", Style::default().fg(Color::Yellow))),
        Line::from(Span::styled(
            error.to_string(),
            Style::default().fg(Color::White),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Please select a valid Bloodborne save file.",
            Style::default().fg(Color::Gray),
        )),
        Line::from(""),
    ];

    let content_block = Paragraph::new(content)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(content_block, chunks[1]);

    // Footer
    let footer = Paragraph::new(Line::from(vec![
        Span::styled("Enter/Esc", Style::default().fg(Color::Cyan)),
        Span::raw(": Go back  "),
        Span::styled("q", Style::default().fg(Color::Cyan)),
        Span::raw(": Quit"),
    ]))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

/// Render teleport success screen
pub fn render_teleport_success(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(10),   // Content
            Constraint::Length(3), // Footer
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Success!")
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Lantern Teleport TUI ")
                .title_style(Style::default().fg(Color::Cyan)),
        );
    f.render_widget(title, chunks[0]);

    // Content
    let content = build_teleport_success_content(app);
    let content_block = Paragraph::new(content)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(content_block, chunks[1]);

    // Footer
    let footer = Paragraph::new(Line::from(vec![
        Span::styled("Enter", Style::default().fg(Color::Cyan)),
        Span::raw(": Teleport again  "),
        Span::styled("Esc", Style::default().fg(Color::Cyan)),
        Span::raw(": Change file  "),
        Span::styled("q", Style::default().fg(Color::Cyan)),
        Span::raw(": Quit"),
    ]))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

/// Build teleport success content
fn build_teleport_success_content(app: &App) -> Vec<Line<'static>> {
    let location_info = app
        .selected_destination
        .map(|loc| {
            (
                loc.name.to_string(),
                loc.region.to_string(),
                loc.x,
                loc.y,
                loc.z,
            )
        })
        .unwrap_or_else(|| ("Unknown".to_string(), "Unknown".to_string(), 0.0, 0.0, 0.0));

    vec![
        Line::from(""),
        Line::from(Span::styled(
            "✓ Successfully teleported!",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Destination:",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!("{} ({})", location_info.0, location_info.1),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!(
                "X: {:.2}  Y: {:.2}  Z: {:.2}",
                location_info.2, location_info.3, location_info.4
            ),
            Style::default().fg(Color::White),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Your save file has been updated.",
            Style::default().fg(Color::Gray),
        )),
        Line::from(Span::styled(
            "Load your game to spawn at the new location!",
            Style::default().fg(Color::Gray),
        )),
        Line::from(""),
    ]
}

/// Render teleport error screen
pub fn render_teleport_error(f: &mut Frame, error: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(8),    // Content
            Constraint::Length(3), // Footer
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Teleport Failed")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Lantern Teleport TUI ")
                .title_style(Style::default().fg(Color::Cyan)),
        );
    f.render_widget(title, chunks[0]);

    // Content
    let content = vec![
        Line::from(""),
        Line::from(Span::styled(
            "✗ Teleport failed!",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled("Error:", Style::default().fg(Color::Yellow))),
        Line::from(Span::styled(
            error.to_string(),
            Style::default().fg(Color::White),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Your save file was not modified.",
            Style::default().fg(Color::Gray),
        )),
        Line::from(""),
    ];

    let content_block = Paragraph::new(content)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(content_block, chunks[1]);

    // Footer
    let footer = Paragraph::new(Line::from(vec![
        Span::styled("Enter/Esc", Style::default().fg(Color::Cyan)),
        Span::raw(": Go back  "),
        Span::styled("q", Style::default().fg(Color::Cyan)),
        Span::raw(": Quit"),
    ]))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}
