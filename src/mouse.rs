//! Mouse support.
use std::io::stdout;

use bevy::prelude::*;
use crossterm::{
    ExecutableCommand,
    event::{DisableMouseCapture, EnableMouseCapture},
};

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

#[derive(Resource, Default)]
pub struct MouseCaptureEnabled;

fn setup(mut commands: Commands) -> Result {
    stdout().execute(EnableMouseCapture)?;
    commands.insert_resource(MouseCaptureEnabled);
    Ok(())
}

impl Drop for MouseCaptureEnabled {
    fn drop(&mut self) {
        let _ = stdout().execute(DisableMouseCapture);
    }
}
