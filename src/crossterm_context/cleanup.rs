use bevy::prelude::*;

use crate::RatatuiContext;

use super::{kitty::KittyEnabled, mouse::MouseEnabled};

/// Plugin responsible for cleaning up resources in the correct order when exiting.
///
/// If raw mode, the alternate view, and the Kitty protocol are disabled in the wrong order, it can
/// cause issues for the terminal buffer after the application exits.
pub struct CleanupPlugin;

impl Plugin for CleanupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Last, cleanup);
    }
}

fn cleanup(mut exit: MessageReader<AppExit>, mut commands: Commands) {
    for _ in exit.read() {
        commands.remove_resource::<KittyEnabled>();
        commands.remove_resource::<MouseEnabled>();
        commands.remove_resource::<RatatuiContext>();
    }
}
