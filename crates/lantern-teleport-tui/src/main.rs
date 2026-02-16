//! Lantern Teleport TUI - Terminal User Interface for Bloodborne teleportation
//!
//! A ratatui-based TUI for selecting save files and teleporting to lantern locations.

mod app;
mod event;
mod ui;

use std::time::Duration;

use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use app::{App, AppMode};
use event::{EventHandler, KeyAction, TerminalEvent};

fn main() -> std::io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and event handler
    let mut app = App::new();
    app.refresh_file_list();

    let events = EventHandler::new(Duration::from_millis(250));

    // Main loop
    let res = run_app(&mut terminal, &mut app, events);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
    events: EventHandler,
) -> std::io::Result<()> {
    loop {
        // Draw the current frame
        terminal.draw(|f| render_frame(f, app))?;

        // Handle events
        match events.next()? {
            TerminalEvent::Key(key) => {
                // Convert to KeyAction
                let action = KeyAction::from_key_event(key);

                // Global quit handler
                if matches!(action, Some(KeyAction::Quit)) {
                    app.quit();
                }

                // Mode-specific handlers
                match &app.mode {
                    AppMode::FileBrowser => handle_file_browser_input(app, action),
                    AppMode::Validating => {}
                    AppMode::ValidationSuccess(_) => handle_validation_success_input(app, action),
                    AppMode::ValidationError(_) => handle_validation_error_input(app, action),
                    AppMode::LocationSelection => handle_location_selection_input(app, action),
                    AppMode::LocationSearch => handle_location_search_input(app, action),
                    AppMode::Confirmation => handle_confirmation_input(app, action),
                    AppMode::Teleporting => {}
                    AppMode::TeleportSuccess => handle_teleport_success_input(app, action),
                    AppMode::TeleportError(_) => handle_teleport_error_input(app, action),
                }
            }
            TerminalEvent::Resize(_, _) => {
                // Terminal resized, will redraw on next iteration
            }
            _ => {}
        }

        // Handle automatic state transitions
        match &app.mode {
            AppMode::Validating => {
                app.validate_save_file();
            }
            AppMode::Teleporting => {
                app.execute_teleport();
            }
            _ => {}
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn render_frame(f: &mut ratatui::Frame, app: &App) {
    match &app.mode {
        AppMode::FileBrowser => ui::render_file_browser(f, app),
        AppMode::Validating => {
            // Show a loading message
            render_loading(f, "Validating save file...");
        }
        AppMode::ValidationSuccess(position) => ui::render_validation_success(f, app, position),
        AppMode::ValidationError(error) => ui::render_validation_error(f, error),
        AppMode::LocationSelection | AppMode::LocationSearch => ui::render_location_list(f, app),
        AppMode::Confirmation => ui::render_confirmation(f, app),
        AppMode::Teleporting => {
            render_loading(f, "Teleporting...");
        }
        AppMode::TeleportSuccess => ui::render_teleport_success(f, app),
        AppMode::TeleportError(error) => ui::render_teleport_error(f, error),
    }
}

fn render_loading(f: &mut ratatui::Frame, message: &str) {
    use ratatui::{
        layout::Alignment,
        style::{Color, Style},
        widgets::{Block, Borders, Paragraph},
    };

    let paragraph = Paragraph::new(message)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Lantern Teleport TUI ")
                .title_style(Style::default().fg(Color::Cyan)),
        );
    f.render_widget(paragraph, f.area());
}

// Input handlers for each mode

fn handle_file_browser_input(app: &mut App, action: Option<KeyAction>) {
    match action {
        Some(KeyAction::Up) => app.move_file_up(),
        Some(KeyAction::Down) => app.move_file_down(),
        Some(KeyAction::Enter) => app.navigate_to_selected(),
        _ => {}
    }
}

fn handle_validation_success_input(app: &mut App, action: Option<KeyAction>) {
    match action {
        Some(KeyAction::Enter) => app.mode = AppMode::LocationSelection,
        Some(KeyAction::Escape) => app.go_back_to_file_browser(),
        _ => {}
    }
}

fn handle_validation_error_input(app: &mut App, action: Option<KeyAction>) {
    match action {
        Some(KeyAction::Enter) | Some(KeyAction::Escape) => app.go_back_to_file_browser(),
        _ => {}
    }
}

fn handle_location_selection_input(app: &mut App, action: Option<KeyAction>) {
    match action {
        Some(KeyAction::Up) => app.move_location_up(),
        Some(KeyAction::Down) => app.move_location_down(),
        Some(KeyAction::Enter) => app.select_location(),
        Some(KeyAction::Escape) => app.go_back_to_file_browser(),
        Some(KeyAction::Search) => app.mode = AppMode::LocationSearch,
        _ => {}
    }
}

fn handle_location_search_input(app: &mut App, action: Option<KeyAction>) {
    match action {
        Some(KeyAction::Enter) => {
            // Exit search mode but keep the filter
            app.mode = AppMode::LocationSelection;
        }
        Some(KeyAction::Escape) => app.clear_search(),
        Some(KeyAction::Backspace) => {
            app.search_query.pop();
            app.apply_search_filter();
        }
        Some(KeyAction::Char(c)) => {
            app.search_query.push(c);
            app.apply_search_filter();
        }
        Some(KeyAction::Up) => app.move_location_up(),
        Some(KeyAction::Down) => app.move_location_down(),
        _ => {}
    }
}

fn handle_confirmation_input(app: &mut App, action: Option<KeyAction>) {
    match action {
        Some(KeyAction::Left) => app.move_confirm_left(),
        Some(KeyAction::Right) => app.move_confirm_right(),
        Some(KeyAction::Enter) => app.mode = AppMode::Teleporting,
        Some(KeyAction::Escape) => app.mode = AppMode::LocationSelection,
        _ => {}
    }
}

fn handle_teleport_success_input(app: &mut App, action: Option<KeyAction>) {
    match action {
        Some(KeyAction::Enter) => {
            // Go back to location selection for another teleport
            app.mode = AppMode::LocationSelection;
            app.selected_destination = None;
        }
        Some(KeyAction::Escape) => app.go_back_to_file_browser(),
        _ => {}
    }
}

fn handle_teleport_error_input(app: &mut App, action: Option<KeyAction>) {
    match action {
        Some(KeyAction::Enter) | Some(KeyAction::Escape) => {
            app.mode = AppMode::LocationSelection;
        }
        _ => {}
    }
}
