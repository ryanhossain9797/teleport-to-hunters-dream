//! File browser UI component

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::app::App;

/// Render the file browser screen
pub fn render_file_browser(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(10),   // File list
            Constraint::Length(3), // Footer
        ])
        .split(f.area());

    // Title block
    let title_block = Block::default()
        .borders(Borders::ALL)
        .title(" Lantern Teleport TUI - Select Save File ")
        .title_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );

    let current_path = app.current_path.to_string_lossy();
    let title_text = Paragraph::new(current_path.as_ref()).block(title_block);
    f.render_widget(title_text, chunks[0]);

    // File list with built-in scrolling via ListState
    let items: Vec<ListItem> = app
        .file_list
        .iter()
        .map(|entry| {
            let prefix = if entry.is_dir { "📁 " } else { "📄 " };
            let name = format!("{}{}", prefix, entry.name);
            ListItem::new(name)
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.selected_file));

    let file_list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(format!(
            " Files ({}/{}) ",
            app.selected_file.saturating_add(1).min(app.file_list.len()),
            app.file_list.len()
        )))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("► ");

    f.render_stateful_widget(file_list, chunks[1], &mut state);

    // Footer
    let footer = Paragraph::new(Line::from(vec![
        Span::styled("↑/↓", Style::default().fg(Color::Cyan)),
        Span::raw(": Navigate  "),
        Span::styled("Enter", Style::default().fg(Color::Cyan)),
        Span::raw(": Select  "),
        Span::styled("q", Style::default().fg(Color::Cyan)),
        Span::raw(": Quit"),
    ]))
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(footer, chunks[2]);
}
