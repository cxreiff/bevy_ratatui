use bevy::{
    app::{Plugin, PluginGroup, PluginGroupBuilder, Startup},
    prelude::{Commands, Result},
};

use crate::{RatatuiContext, context::DefaultContext};

use crate::context::TerminalContext;

/// A plugin group that includes all the plugins in the Ratatui crate.
///
/// # Example
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy_ratatui::RatatuiPlugins;
///
/// App::new().add_plugins(RatatuiPlugins::default());
/// ```
pub struct RatatuiPlugins {
    /// Use kitty protocol if available and enabled.
    pub enable_kitty_protocol: bool,
    /// Capture mouse if enabled.
    pub enable_mouse_capture: bool,
    /// Forwards terminal input events to the bevy input system if enabled.
    pub enable_input_forwarding: bool,
}

impl Default for RatatuiPlugins {
    fn default() -> Self {
        Self {
            enable_kitty_protocol: true,
            enable_mouse_capture: false,
            enable_input_forwarding: false,
        }
    }
}

impl PluginGroup for RatatuiPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();

        builder = builder.add(ContextPlugin);

        builder = DefaultContext::configure_plugin_group(&self, builder);

        builder
    }
}

/// The plugin responsible for adding the `RatatuiContext` resource to your bevy application.
pub struct ContextPlugin;

impl Plugin for ContextPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, context_setup);
    }
}

/// A startup system that sets up the terminal context.
pub fn context_setup(mut commands: Commands) -> Result {
    let terminal = RatatuiContext::init()?;
    commands.insert_resource(terminal);

    Ok(())
}
