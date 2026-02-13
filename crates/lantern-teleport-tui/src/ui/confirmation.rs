//! Confirmation dialog UI component

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::App;
use lantern_teleport_core::Location;

/// Render the confirmation dialog
pub fn render_confirmation(f: &mut Frame, app: &App) {
    // Create a centered dialog box
    let dialog_area = centered_rect(60, 50, f.area());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // Title
            Constraint::Min(8),    // Content
            Constraint::Length(3), // Buttons
            Constraint::Length(2), // Footer
        ])
        .split(dialog_area);

    // Title
    let title = Paragraph::new("Confirm Teleport")
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .borders(Borders::TOP)
                .title(" Lantern Teleport TUI ")
                .title_style(Style::default().fg(Color::Cyan)),
        );
    f.render_widget(title, chunks[0]);

    // Content
    if let Some(location) = app.selected_destination {
        let content = build_confirmation_content(location);
        let content_block = Paragraph::new(content)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT));
        f.render_widget(content_block, chunks[1]);
    }

    // Buttons
    let button_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    let cancel_style = if !app.confirm_selection {
        Style::default()
            .fg(Color::Red)
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let confirm_style = if app.confirm_selection {
        Style::default()
            .fg(Color::Green)
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let cancel_button = Paragraph::new("Cancel")
        .alignment(Alignment::Center)
        .style(cancel_style)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(cancel_button, button_chunks[0]);

    let confirm_button = Paragraph::new("Confirm")
        .alignment(Alignment::Center)
        .style(confirm_style)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(confirm_button, button_chunks[1]);

    // Footer
    let footer = Paragraph::new(Line::from(vec![
        Span::styled("←/→", Style::default().fg(Color::Cyan)),
        Span::raw(": Switch  "),
        Span::styled("Enter", Style::default().fg(Color::Cyan)),
        Span::raw(": Select  "),
        Span::styled("Esc", Style::default().fg(Color::Cyan)),
        Span::raw(": Cancel  "),
        Span::styled("q", Style::default().fg(Color::Cyan)),
        Span::raw(": Quit"),
    ]))
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(footer, chunks[3]);
}

/// Build the confirmation content text
fn build_confirmation_content(location: &Location) -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        Line::from(Span::styled(
            "You are about to teleport to:",
            Style::default().fg(Color::White),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("{} ({})", location.name, location.region),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!(
                "X: {:.2}  Y: {:.2}  Z: {:.2}",
                location.x, location.y, location.z
            ),
            Style::default().fg(Color::Cyan),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "This will modify your save file.",
            Style::default().fg(Color::Red),
        )),
        Line::from(Span::styled(
            "Make sure you have a backup!",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ]
}

/// Helper function to create a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
