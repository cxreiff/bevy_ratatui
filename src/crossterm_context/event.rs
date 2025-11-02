//! Event handling.
//!
//! This module provides a plugin for handling events, and a wrapper around
//! `crossterm::event::KeyEvent`.
//!
//! # Example
//!
//! ```rust
//! use bevy::{app::AppExit, prelude::*};
//! use bevy_ratatui::event::KeyEvent;
//! use ratatui::crossterm::event::KeyCode;
//!
//! fn keyboard_input_system(mut events: MessageReader<KeyEvent>, mut exit: MessageWriter<AppExit>) {
//!     for event in events.read() {
//!         match event.code {
//!             KeyCode::Char('q') | KeyCode::Esc => {
//!                 exit.write_default();
//!             }
//!             _ => {}
//!         }
//!     }
//! }
//! ```
use std::time::Duration;

use bevy::{app::AppExit, prelude::*};
use ratatui::crossterm::event::{self, Event::Key, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::Size;

/// A plugin for handling events.
///
/// This plugin reads events from the terminal environment and forwards them to Bevy using the
/// `KeyEvent` event.
pub struct EventPlugin {
    /// Adds an input handler that signals bevy to exit when an interrupt keypress (control+c) is read.
    pub control_c_interrupt: bool,
}

impl Default for EventPlugin {
    fn default() -> Self {
        Self {
            control_c_interrupt: true,
        }
    }
}

impl Plugin for EventPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_message::<KeyEvent>()
            .add_message::<MouseEvent>()
            .add_message::<FocusEvent>()
            .add_message::<ResizeEvent>()
            .add_message::<PasteEvent>()
            .add_message::<CrosstermEvent>()
            .configure_sets(
                Update,
                (
                    InputSet::Pre,
                    InputSet::EmitCrossterm,
                    InputSet::CheckEmulation,
                    InputSet::EmitBevy,
                    InputSet::Post,
                )
                    .chain(),
            )
            .add_systems(
                PreUpdate,
                crossterm_event_system.in_set(InputSet::EmitCrossterm),
            );

        if self.control_c_interrupt {
            app.add_systems(Update, control_c_interrupt_system.in_set(InputSet::Post));
        }
    }
}

/// InputSet defines when the input events are emitted.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum InputSet {
    /// Run before any input events are emitted.
    Pre,
    /// Emit the crossterm events.
    EmitCrossterm,
    /// Check for emulation
    CheckEmulation,
    /// Emit the bevy events if [crate::input_forwarding::KeyboardPlugin] has been added.
    EmitBevy,
    /// Run after all input events are emitted.
    Post,
}

/// An event that is sent whenever an event is read from crossterm.
#[derive(Debug, Deref, Message, PartialEq, Eq, Clone, Hash)]
pub struct CrosstermEvent(pub event::Event);

/// An event that is sent whenever a key event is read from crossterm.
#[derive(Debug, Deref, Message, PartialEq, Eq, Clone, Hash)]
pub struct KeyEvent(pub event::KeyEvent);

/// An event that is sent whenever a mouse event is read from crossterm.
#[derive(Debug, Clone, Copy, Message, PartialEq, Eq, Deref)]
pub struct MouseEvent(pub event::MouseEvent);

/// An event that is sent when the terminal gains or loses focus.
#[derive(Debug, Clone, Copy, Message, PartialEq, Eq)]
pub enum FocusEvent {
    Gained,
    Lost,
}

/// An event that is sent when the terminal is resized.
#[derive(Debug, Clone, Copy, Message, PartialEq, Eq, Deref)]
pub struct ResizeEvent(pub Size);

/// An event that is sent when text is pasted into the terminal.
#[derive(Debug, Clone, Message, PartialEq, Eq, Deref)]
pub struct PasteEvent(pub String);

/// System that reads events from crossterm and sends them to the `KeyEvent` event.
///
/// This system reads events from crossterm and sends them to the `KeyEvent` event. It also sends
/// an `AppExit` event when `Ctrl+C` is pressed.
pub fn crossterm_event_system(
    mut events: MessageWriter<CrosstermEvent>,
    mut keys: MessageWriter<KeyEvent>,
    mut mouse: MessageWriter<MouseEvent>,
    mut focus: MessageWriter<FocusEvent>,
    mut paste: MessageWriter<PasteEvent>,
    mut resize: MessageWriter<ResizeEvent>,
) -> Result {
    while event::poll(Duration::ZERO)? {
        let event = event::read()?;
        match event {
            Key(event) => {
                keys.write(KeyEvent(event));
            }
            event::Event::FocusLost => {
                focus.write(FocusEvent::Lost);
            }
            event::Event::FocusGained => {
                focus.write(FocusEvent::Gained);
            }
            event::Event::Mouse(event) => {
                mouse.write(MouseEvent(event));
            }
            event::Event::Paste(ref s) => {
                paste.write(PasteEvent(s.clone()));
            }
            event::Event::Resize(columns, rows) => {
                resize.write(ResizeEvent(Size::new(columns, rows)));
            }
        }
        events.write(CrosstermEvent(event));
    }
    Ok(())
}

fn control_c_interrupt_system(
    mut key_events: MessageReader<KeyEvent>,
    mut exit: MessageWriter<AppExit>,
) {
    for event in key_events.read() {
        if event.kind == KeyEventKind::Press
            && event.modifiers == KeyModifiers::CONTROL
            && event.code == KeyCode::Char('c')
        {
            exit.write_default();
        }
    }
}
