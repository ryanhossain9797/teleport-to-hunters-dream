//! Event handling for the TUI application

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use std::time::Duration;

/// Terminal event types
pub enum TerminalEvent {
    /// Key press event
    Key(KeyEvent),
    /// Mouse event
    Mouse(MouseEvent),
    /// Terminal resize
    Resize(u16, u16),
    /// Tick event (for animations/updates)
    Tick,
}

/// Event handler with tick rate for animations
pub struct EventHandler {
    /// Tick rate for periodic updates
    tick_rate: Duration,
}

impl EventHandler {
    /// Create a new event handler with the specified tick rate
    pub fn new(tick_rate: Duration) -> Self {
        Self { tick_rate }
    }

    /// Poll for the next terminal event
    ///
    /// Returns `TerminalEvent::Tick` if no event occurs within the tick rate
    pub fn next(&self) -> std::io::Result<TerminalEvent> {
        if event::poll(self.tick_rate)? {
            match event::read()? {
                Event::Key(key) => Ok(TerminalEvent::Key(key)),
                Event::Mouse(mouse) => Ok(TerminalEvent::Mouse(mouse)),
                Event::Resize(width, height) => Ok(TerminalEvent::Resize(width, height)),
                _ => Ok(TerminalEvent::Tick),
            }
        } else {
            Ok(TerminalEvent::Tick)
        }
    }
}

/// Check if the event is a quit key combination
pub fn is_quit_key(key: KeyEvent) -> bool {
    matches!(
        key,
        KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } | KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
            ..
        }
    )
}

/// Check if the event is an up navigation key
pub fn is_up_key(key: KeyEvent) -> bool {
    matches!(
        key,
        KeyEvent {
            code: KeyCode::Up | KeyCode::Char('k'),
            modifiers: KeyModifiers::NONE,
            ..
        }
    )
}

/// Check if the event is a down navigation key
pub fn is_down_key(key: KeyEvent) -> bool {
    matches!(
        key,
        KeyEvent {
            code: KeyCode::Down | KeyCode::Char('j'),
            modifiers: KeyModifiers::NONE,
            ..
        }
    )
}

/// Check if the event is a left navigation key
pub fn is_left_key(key: KeyEvent) -> bool {
    matches!(
        key,
        KeyEvent {
            code: KeyCode::Left | KeyCode::Char('h'),
            modifiers: KeyModifiers::NONE,
            ..
        }
    )
}

/// Check if the event is a right navigation key
pub fn is_right_key(key: KeyEvent) -> bool {
    matches!(
        key,
        KeyEvent {
            code: KeyCode::Right | KeyCode::Char('l'),
            modifiers: KeyModifiers::NONE,
            ..
        }
    )
}

/// Check if the event is an enter/select key
pub fn is_enter_key(key: KeyEvent) -> bool {
    matches!(
        key,
        KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
            ..
        }
    )
}

/// Check if the event is an escape key
pub fn is_escape_key(key: KeyEvent) -> bool {
    matches!(
        key,
        KeyEvent {
            code: KeyCode::Esc,
            modifiers: KeyModifiers::NONE,
            ..
        }
    )
}

/// Check if the event is a search activation key (/)
pub fn is_search_key(key: KeyEvent) -> bool {
    matches!(
        key,
        KeyEvent {
            code: KeyCode::Char('/'),
            modifiers: KeyModifiers::NONE,
            ..
        }
    )
}

/// Check if the event is a backspace key
pub fn is_backspace_key(key: KeyEvent) -> bool {
    matches!(
        key,
        KeyEvent {
            code: KeyCode::Backspace,
            modifiers: KeyModifiers::NONE,
            ..
        }
    )
}

/// Check if the event is a printable character
pub fn is_printable_char(key: KeyEvent) -> Option<char> {
    match key {
        KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
            ..
        } => Some(c),
        _ => None,
    }
}
