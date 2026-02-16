//! Event handling for the TUI application

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use std::time::Duration;

/// Internal representation of key actions for the TUI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyAction {
    /// Quit the application (Ctrl+C or 'q')
    Quit,
    /// Move up (Up arrow or 'k')
    Up,
    /// Move down (Down arrow or 'j')
    Down,
    /// Move left (Left arrow or 'h')
    Left,
    /// Move right (Right arrow or 'l')
    Right,
    /// Confirm/Select (Enter)
    Enter,
    /// Cancel/Go back (Escape)
    Escape,
    /// Start search ('/')
    Search,
    /// Delete character (Backspace)
    Backspace,
    /// Printable character input
    Char(char),
}

impl KeyAction {
    /// Convert a KeyEvent into a KeyAction
    pub fn from_key_event(key: KeyEvent) -> Option<Self> {
        match key {
            // Quit: Ctrl+C or 'q'
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => Some(KeyAction::Quit),
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
                ..
            } => Some(KeyAction::Quit),

            // Navigation: arrow keys or vim-style hjkl
            KeyEvent {
                code: KeyCode::Up | KeyCode::Char('k'),
                modifiers: KeyModifiers::NONE,
                ..
            } => Some(KeyAction::Up),
            KeyEvent {
                code: KeyCode::Down | KeyCode::Char('j'),
                modifiers: KeyModifiers::NONE,
                ..
            } => Some(KeyAction::Down),
            KeyEvent {
                code: KeyCode::Left | KeyCode::Char('h'),
                modifiers: KeyModifiers::NONE,
                ..
            } => Some(KeyAction::Left),
            KeyEvent {
                code: KeyCode::Right | KeyCode::Char('l'),
                modifiers: KeyModifiers::NONE,
                ..
            } => Some(KeyAction::Right),

            // Enter, Escape, Search, Backspace
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                ..
            } => Some(KeyAction::Enter),
            KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                ..
            } => Some(KeyAction::Escape),
            KeyEvent {
                code: KeyCode::Char('/'),
                modifiers: KeyModifiers::NONE,
                ..
            } => Some(KeyAction::Search),
            KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
                ..
            } => Some(KeyAction::Backspace),

            // Printable characters
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
                ..
            } => Some(KeyAction::Char(c)),

            // Unhandled key
            _ => None,
        }
    }
}

pub enum TerminalEvent {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
    Tick,
}

pub struct EventHandler {
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
