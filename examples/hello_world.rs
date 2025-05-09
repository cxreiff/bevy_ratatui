use std::time::Duration;

use bevy::{
    app::{AppExit, ScheduleRunnerPlugin},
    prelude::*,
};
use bevy_ratatui::{RatatuiPlugins, terminal::RatatuiContext};

use ratatui::text::Text;

fn main() {
    let frame_time = Duration::from_secs_f32(1. / 60.);

    App::new()
        .add_plugins((
            #[cfg(not(feature = "windowed"))]
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(frame_time)),
            #[cfg(feature = "windowed")]
            DefaultPlugins,
            RatatuiPlugins {
                enable_input_forwarding: true,
                ..default()
            },
        ))
        .add_systems(PreUpdate, input_system)
        .add_systems(Update, draw_system)
        .run();
}

fn draw_system(mut context: ResMut<RatatuiContext>) -> Result {
    context.draw(|frame| {
        let text = Text::raw("hello world\npress 'q' to quit");
        frame.render_widget(text, frame.area());
    })?;

    Ok(())
}

fn input_system(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::KeyQ) {
        exit.write_default();
    }
}
