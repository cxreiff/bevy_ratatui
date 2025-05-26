//! A collection of plugins for building terminal-based applications with [Bevy] and [Ratatui].
//!
//! # Example
//!
//! ```rust,no_run
//! use std::time::Duration;
//!
//! use bevy::{
//!     app::{AppExit, ScheduleRunnerPlugin},
//!     prelude::*,
//! };
//! use bevy_ratatui::{event::KeyEvent, RatatuiContext, RatatuiPlugins};
//! use crossterm::event::KeyCode;
//! use ratatui::text::Text;
//!
//! fn main() {
//!     let frame_time = Duration::from_secs_f32(1. / 60.);
//!
//!     App::new()
//!         .add_plugins((
//!             MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(frame_time)),
//!             RatatuiPlugins::default(),
//!         ))
//!         .add_systems(PreUpdate, input_system)
//!         .add_systems(Update, draw_system)
//!         .run();
//! }
//!
//! fn draw_system(mut context: ResMut<RatatuiContext>) -> Result {
//!     context.draw(|frame| {
//!         let text = Text::raw("hello world\npress 'q' to quit");
//!         frame.render_widget(text, frame.area());
//!     })?;
//!
//!     Ok(())
//! }
//!
//! fn input_system(mut events: EventReader<KeyEvent>, mut exit: EventWriter<AppExit>) {
//!     for event in events.read() {
//!         if let KeyCode::Char('q') = event.code {
//!             exit.send_default();
//!         }
//!     }
//! }
//! ```
//!
//! See the [examples] directory for more examples.
//!
//! # Input Forwarding
//!
//! The terminal input can be forwarded to the bevy input system. See the
//! [input_forwarding] module documentation for details.
//!
//! [Bevy]: https://bevyengine.org
//! [Ratatui]: https://ratatui.rs
//! [examples]: https://github.com/cxreiff/bevy_ratatui/tree/main/examples

mod context_trait;
#[cfg(feature = "crossterm")]
mod crossterm_context;
mod ratatui_context;
mod ratatui_plugin;
#[cfg(feature = "windowed")]
mod windowed_context;

pub use ratatui_context::RatatuiContext;
pub use ratatui_plugin::RatatuiPlugins;

pub mod context {
    pub use super::context_trait::TerminalContext;
    #[cfg(feature = "crossterm")]
    pub use super::crossterm_context::context::CrosstermContext;
    pub use super::ratatui_context::DefaultContext;
    pub use super::ratatui_plugin::ContextPlugin;
    #[cfg(feature = "windowed")]
    pub use super::windowed_context::context::WindowedContext;
}

#[cfg(feature = "crossterm")]
pub mod cleanup {
    pub use super::crossterm_context::cleanup::CleanupPlugin;
}

#[cfg(feature = "crossterm")]
pub mod error {
    pub use super::crossterm_context::error::ErrorPlugin;
}

#[cfg(feature = "crossterm")]
pub mod event {
    pub use super::crossterm_context::event::{
        CrosstermEvent, EventPlugin, FocusEvent, InputSet, KeyEvent, MouseEvent, PasteEvent,
        ResizeEvent,
    };
}

#[cfg(feature = "crossterm")]
pub mod kitty {
    pub use super::crossterm_context::kitty::{KittyEnabled, KittyPlugin};
}

#[cfg(feature = "crossterm")]
pub mod mouse {
    pub use super::crossterm_context::mouse::{MouseEnabled, MousePlugin};
}

#[cfg(feature = "crossterm")]
pub mod translation {
    pub use super::crossterm_context::translation::*;
}

#[cfg(feature = "windowed")]
pub mod windowed {
    pub use super::windowed_context::plugin::WindowedPlugin;
}
