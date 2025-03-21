use std::time::Duration;

use bevy::{
    app::{AppExit, ScheduleRunnerPlugin},
    prelude::*,
    utils::error,
};
use bevy_ratatui::{event::KeyEvent, terminal::RatatuiContext, RatatuiPlugins};
use crossterm::event::KeyCode;
use ratatui::text::Text;

fn main() {
    let frame_time = Duration::from_secs_f32(1. / 60.);

    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(frame_time)),
            RatatuiPlugins::default(),
        ))
        .add_systems(PreUpdate, input_system)
        .add_systems(Update, draw_system.map(error))
        .run();
}

fn draw_system(mut context: ResMut<RatatuiContext>) -> std::io::Result<()> {
    context.draw(|frame| {
        let text = Text::raw("hello world\npress 'q' to quit");
        frame.render_widget(text, frame.area());
    })?;

    Ok(())
}

fn input_system(mut events: EventReader<KeyEvent>, mut exit: EventWriter<AppExit>) {
    for event in events.read() {
        if let KeyCode::Char('q') = event.code {
            exit.send_default();
        }
    }
}
