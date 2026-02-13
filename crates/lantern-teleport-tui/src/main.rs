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
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::{App, AppMode};
use event::{
    is_backspace_key, is_down_key, is_enter_key, is_escape_key, is_left_key, is_printable_char,
    is_quit_key, is_right_key, is_search_key, is_up_key,
};
use event::{EventHandler, TerminalEvent};

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
                // Global quit handler
                if is_quit_key(key) {
                    app.quit();
                }

                // Mode-specific handlers
                match &app.mode {
                    AppMode::FileBrowser => handle_file_browser_input(app, key),
                    AppMode::Validating => {
                        // Validation happens automatically, just wait
                    }
                    AppMode::ValidationSuccess(_) => handle_validation_success_input(app, key),
                    AppMode::ValidationError(_) => handle_validation_error_input(app, key),
                    AppMode::LocationSelection => handle_location_selection_input(app, key),
                    AppMode::LocationSearch => handle_location_search_input(app, key),
                    AppMode::Confirmation(_) => handle_confirmation_input(app, key),
                    AppMode::Teleporting => {
                        // Teleport happens automatically, just wait
                    }
                    AppMode::TeleportSuccess => handle_teleport_success_input(app, key),
                    AppMode::TeleportError(_) => handle_teleport_error_input(app, key),
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
        AppMode::Confirmation(_) => ui::render_confirmation(f, app),
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

fn handle_file_browser_input(app: &mut App, key: crossterm::event::KeyEvent) {
    if is_up_key(key) {
        app.move_file_up();
    } else if is_down_key(key) {
        app.move_file_down();
    } else if is_enter_key(key) {
        app.navigate_to_selected();
    }
}

fn handle_validation_success_input(app: &mut App, key: crossterm::event::KeyEvent) {
    if is_enter_key(key) {
        app.mode = AppMode::LocationSelection;
    } else if is_escape_key(key) {
        app.go_back_to_file_browser();
    }
}

fn handle_validation_error_input(app: &mut App, key: crossterm::event::KeyEvent) {
    if is_enter_key(key) || is_escape_key(key) {
        app.go_back_to_file_browser();
    }
}

fn handle_location_selection_input(app: &mut App, key: crossterm::event::KeyEvent) {
    if is_up_key(key) {
        app.move_location_up();
    } else if is_down_key(key) {
        app.move_location_down();
    } else if is_enter_key(key) {
        app.select_location();
    } else if is_escape_key(key) {
        app.go_back_to_file_browser();
    } else if is_search_key(key) {
        app.mode = AppMode::LocationSearch;
    }
}

fn handle_location_search_input(app: &mut App, key: crossterm::event::KeyEvent) {
    if is_enter_key(key) {
        // Exit search mode but keep the filter
        app.mode = AppMode::LocationSelection;
    } else if is_escape_key(key) {
        app.clear_search();
    } else if is_backspace_key(key) {
        app.search_query.pop();
        app.apply_search_filter();
    } else if let Some(c) = is_printable_char(key) {
        app.search_query.push(c);
        app.apply_search_filter();
    } else if is_up_key(key) {
        app.move_location_up();
    } else if is_down_key(key) {
        app.move_location_down();
    }
}

fn handle_confirmation_input(app: &mut App, key: crossterm::event::KeyEvent) {
    if is_left_key(key) {
        app.move_confirm_left();
    } else if is_right_key(key) {
        app.move_confirm_right();
    } else if is_enter_key(key) {
        app.mode = AppMode::Teleporting;
    } else if is_escape_key(key) {
        app.mode = AppMode::LocationSelection;
    }
}

fn handle_teleport_success_input(app: &mut App, key: crossterm::event::KeyEvent) {
    if is_enter_key(key) {
        // Go back to location selection for another teleport
        app.mode = AppMode::LocationSelection;
        app.selected_destination = None;
    } else if is_escape_key(key) {
        app.go_back_to_file_browser();
    }
}

fn handle_teleport_error_input(app: &mut App, key: crossterm::event::KeyEvent) {
    if is_enter_key(key) || is_escape_key(key) {
        app.mode = AppMode::LocationSelection;
    }
}
