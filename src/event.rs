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
//! use crossterm::event::KeyCode;
//!
//! fn keyboard_input_system(mut events: EventReader<KeyEvent>, mut exit: EventWriter<AppExit>) {
//!     for event in events.read() {
//!         match event.code {
//!             KeyCode::Char('q') | KeyCode::Esc => {
//!                 exit.send_default();
//!             }
//!             _ => {}
//!         }
//!     }
//! }
//! ```
use std::time::Duration;

use bevy::{app::AppExit, prelude::*};
use crossterm::event::{self, Event::Key, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::Size;

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

/// A plugin for handling events.
///
/// This plugin adds the `KeyEvent` event, and a system that reads events from crossterm and sends
/// them to the `KeyEvent` event.
pub struct EventPlugin {
    /// Adds an input handler that signals bevy to exit when an interrupt keypress (control+c) is read.
    control_c_interrupt: bool,
}

impl Default for EventPlugin {
    fn default() -> Self {
        Self {
            control_c_interrupt: true,
        }
    }
}

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<KeyEvent>()
            .add_event::<MouseEvent>()
            .add_event::<FocusEvent>()
            .add_event::<ResizeEvent>()
            .add_event::<PasteEvent>()
            .add_event::<CrosstermEvent>()
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

/// An event that is sent whenever an event is read from crossterm.
#[derive(Debug, Deref, Event, PartialEq, Eq, Clone, Hash)]
pub struct CrosstermEvent(pub event::Event);

/// An event that is sent whenever a key event is read from crossterm.
#[derive(Debug, Deref, Event, PartialEq, Eq, Clone, Hash)]
pub struct KeyEvent(pub event::KeyEvent);

/// An event that is sent whenever a mouse event is read from crossterm.
#[derive(Debug, Clone, Copy, Event, PartialEq, Eq, Deref)]
pub struct MouseEvent(pub event::MouseEvent);

/// An event that is sent when the terminal gains or loses focus.
#[derive(Debug, Clone, Copy, Event, PartialEq, Eq)]
pub enum FocusEvent {
    Gained,
    Lost,
}

/// An event that is sent when the terminal is resized.
#[derive(Debug, Clone, Copy, Event, PartialEq, Eq, Deref)]
pub struct ResizeEvent(pub Size);

/// An event that is sent when text is pasted into the terminal.
#[derive(Debug, Clone, Event, PartialEq, Eq, Deref)]
pub struct PasteEvent(pub String);

/// System that reads events from crossterm and sends them to the `KeyEvent` event.
///
/// This system reads events from crossterm and sends them to the `KeyEvent` event. It also sends
/// an `AppExit` event when `Ctrl+C` is pressed.
pub fn crossterm_event_system(
    mut events: EventWriter<CrosstermEvent>,
    mut keys: EventWriter<KeyEvent>,
    mut mouse: EventWriter<MouseEvent>,
    mut focus: EventWriter<FocusEvent>,
    mut paste: EventWriter<PasteEvent>,
    mut resize: EventWriter<ResizeEvent>,
    mut exit: EventWriter<AppExit>,
) -> Result {
    while event::poll(Duration::ZERO)? {
        let event = event::read()?;
        match event {
            Key(event) => {
                if event.kind == KeyEventKind::Press
                    && event.modifiers == KeyModifiers::CONTROL
                    && event.code == KeyCode::Char('c')
                {
                    exit.write_default();
                }

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
    mut key_events: EventReader<KeyEvent>,
    mut exit: EventWriter<AppExit>,
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
