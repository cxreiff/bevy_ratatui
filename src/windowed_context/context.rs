use std::fmt::Debug;

use bevy::prelude::*;

use ratatui::Terminal;

use soft_ratatui::SoftBackend;

use crate::context::TerminalContext;

use super::plugin::WindowedPlugin;

/// Ratatui context that will set up a window and render the ratatui buffer using a 2D texture,
/// instead of drawing to a terminal buffer.
#[derive(Deref, DerefMut)]
pub struct WindowedContext(Terminal<SoftBackend>);

impl Debug for WindowedContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WindowedContext()")
    }
}

impl TerminalContext<SoftBackend> for WindowedContext {
    fn init() -> Result<Self> {
        let backend = SoftBackend::new_with_font(
            15,
            15,
            16,
            include_bytes!("../../assets/FiraMono-Medium.ttf"),
        );
        let terminal = Terminal::new(backend)?;
        Ok(Self(terminal))
    }

    fn restore() -> Result<()> {
        Ok(())
    }

    fn configure_plugin_group(
        _group: &crate::RatatuiPlugins,
        mut builder: bevy::app::PluginGroupBuilder,
    ) -> bevy::app::PluginGroupBuilder {
        builder = builder.add(WindowedPlugin);

        builder
    }
}
