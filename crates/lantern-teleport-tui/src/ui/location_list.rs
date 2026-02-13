//! Location list UI component with search functionality

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::app::{App, AppMode};

/// Render the location selection screen
pub fn render_location_list(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(3), // Search bar
            Constraint::Min(10),   // Location list
            Constraint::Length(3), // Footer
        ])
        .split(f.area());

    // Title block
    let title = Paragraph::new("Select Destination").block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Lantern Teleport TUI ")
            .title_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
    );
    f.render_widget(title, chunks[0]);

    // Search bar
    let search_style = if matches!(app.mode, AppMode::LocationSearch) {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Gray)
    };

    let search_text = if app.search_query.is_empty() && !matches!(app.mode, AppMode::LocationSearch)
    {
        "Press / to search...".to_string()
    } else {
        format!(
            "Search: {}{}",
            app.search_query,
            if matches!(app.mode, AppMode::LocationSearch) {
                "█"
            } else {
                ""
            }
        )
    };

    let search_bar = Paragraph::new(search_text)
        .style(search_style)
        .block(Block::default().borders(Borders::ALL).title(" Search "));
    f.render_widget(search_bar, chunks[1]);

    // Build location items and calculate selected display index
    let (items, selected_display_idx) = build_location_items_with_selection(app);
    let total_locations = app.get_total_filtered_locations();

    // Location list with built-in scrolling via ListState
    let mut state = ListState::default();
    state.select(Some(selected_display_idx));

    let location_list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(format!(
            " Locations ({}/{}) ",
            app.selected_location.saturating_add(1).min(total_locations),
            total_locations
        )))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("► ");

    f.render_stateful_widget(location_list, chunks[2], &mut state);

    // Footer
    let footer_text = if matches!(app.mode, AppMode::LocationSearch) {
        "Type to search  Enter: Confirm  Esc: Clear search  q: Quit"
    } else {
        "/: Search  ↑/↓: Navigate  Enter: Select  Esc: Change file  q: Quit"
    };

    let footer = Paragraph::new(footer_text).block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[3]);
}

/// Build the list items for the location list, returning items and the display index of selected item
fn build_location_items_with_selection(app: &App) -> (Vec<ListItem<'static>>, usize) {
    let mut items = Vec::new();
    let mut current_location_idx = 0;
    let mut selected_display_idx = 0;
    let mut display_idx = 0;

    for group in &app.filtered_location_groups {
        // Region header
        items.push(ListItem::new(Line::from(Span::styled(
            group.region,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::UNDERLINED),
        ))));
        display_idx += 1;

        // Separator line
        items.push(ListItem::new(Line::from(Span::styled(
            "─".repeat(60),
            Style::default().fg(Color::DarkGray),
        ))));
        display_idx += 1;

        // Locations in this region
        for location in &group.locations {
            if current_location_idx == app.selected_location {
                selected_display_idx = display_idx;
            }

            let coords = format!(
                "(X: {:.2}, Y: {:.2}, Z: {:.2})",
                location.x, location.y, location.z
            );
            let text = format!("  {} {}", location.name, coords);

            items.push(ListItem::new(text));
            display_idx += 1;
            current_location_idx += 1;
        }

        // Empty line between groups
        items.push(ListItem::new(""));
        display_idx += 1;
    }

    (items, selected_display_idx)
}
