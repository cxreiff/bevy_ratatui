//! This app demonstrates:
//!
//! - Using ScheduleRunnerPlugin to run the bevy app loop without a window.
//! - Using the RatatuiContext resource to draw widgets to the terminal.
//! - Using Events to read input and communicate between systems.
//!
//! Keys:
//! - Left & Right: modify the counter
//! - Q or Esc: quit
//! - P: panic (tests the color_eyre panic hooks)

use core::panic;
use std::time::Duration;

use bevy::{
    app::{AppExit, ScheduleRunnerPlugin},
    diagnostic::FrameCount,
    prelude::*,
    state::app::StatesPlugin,
};
use bevy_ratatui::{RatatuiPlugins, event::KeyEvent, terminal::RatatuiContext};
use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::WidgetRef,
};

fn main() {
    let frame_time = Duration::from_secs_f32(1. / 60.);

    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(frame_time)),
            RatatuiPlugins::default(),
            StatesPlugin,
        ))
        .init_resource::<BackgroundColor>()
        .init_resource::<Counter>()
        .init_state::<AppState>()
        .add_event::<CounterEvent>()
        .add_systems(PreUpdate, keyboard_input_system)
        .add_systems(
            Update,
            (ui_system, update_counter_system, background_color_system),
        )
        .add_systems(OnEnter(AppState::Negative), start_background_color_timer)
        .add_systems(OnEnter(AppState::Positive), start_background_color_timer)
        .run();
}

fn ui_system(
    mut context: ResMut<RatatuiContext>,
    frame_count: Res<FrameCount>,
    counter: Res<Counter>,
    app_state: Res<State<AppState>>,
    bg_color: Res<BackgroundColor>,
) -> Result {
    context.draw(|frame| {
        let area = frame.area();
        let frame_count = Line::from(format!("Frame Count: {}", frame_count.0)).right_aligned();
        frame.render_widget(bg_color.as_ref(), area);
        frame.render_widget(frame_count, area);
        frame.render_widget(counter.as_ref(), area);
        frame.render_widget(app_state.get(), area)
    })?;

    Ok(())
}

fn keyboard_input_system(
    mut events: EventReader<KeyEvent>,
    mut app_exit: EventWriter<AppExit>,
    mut counter_events: EventWriter<CounterEvent>,
) {
    for event in events.read() {
        match event.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                app_exit.write_default();
            }
            KeyCode::Char('p') => {
                panic!("Panic!");
            }
            KeyCode::Left => {
                counter_events.write(CounterEvent::Decrement);
            }
            KeyCode::Right => {
                counter_events.write(CounterEvent::Increment);
            }
            _ => {}
        }
    }
}

#[derive(Default, Resource, Debug, Deref, DerefMut)]
struct Counter(i32);

impl Counter {
    fn decrement(&mut self) {
        self.0 -= 1;
    }

    fn increment(&mut self) {
        self.0 += 1;
    }
}

#[derive(Debug, Clone, Copy, Event, PartialEq, Eq)]
enum CounterEvent {
    Increment,
    Decrement,
}

fn update_counter_system(
    mut counter: ResMut<Counter>,
    mut events: EventReader<CounterEvent>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for event in events.read() {
        match event {
            CounterEvent::Increment => counter.increment(),
            CounterEvent::Decrement => counter.decrement(),
        }
        // demonstrates changing something in the app state based on the counter value
        if counter.0 < 0 {
            app_state.set(AppState::Negative);
        } else {
            app_state.set(AppState::Positive);
        }
    }
}

impl WidgetRef for Counter {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let counter = format!("Counter: {}", self.0);
        Line::from(counter).render_ref(area, buf);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum AppState {
    Negative,
    #[default]
    Positive,
}

impl WidgetRef for AppState {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let state = match self {
            AppState::Negative => "Negative",
            AppState::Positive => "Positive",
        };
        Line::from(state).centered().render_ref(area, buf);
    }
}

#[derive(Debug, Component, Deref, DerefMut)]
struct ColorChangeTimer {
    #[deref]
    timer: Timer,
    start_color: Color,
}

fn start_background_color_timer(mut commands: Commands, bg_color: Res<BackgroundColor>) {
    commands.spawn(ColorChangeTimer {
        timer: Timer::from_seconds(2.0, TimerMode::Once),
        start_color: bg_color.0,
    });
}

#[derive(Debug, Resource, Deref, DerefMut)]
struct BackgroundColor(Color);

impl Default for BackgroundColor {
    fn default() -> Self {
        BackgroundColor(Color::Rgb(0, 0, 0))
    }
}

impl WidgetRef for BackgroundColor {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, Style::new().bg(self.0));
    }
}

/// Change the background color over time when the app state changes from negative to positive
/// or vice versa.
fn background_color_system(
    time: Res<Time>,
    query: Single<(Entity, &mut ColorChangeTimer)>,
    app_state: Res<State<AppState>>,
    mut commands: Commands,
    mut bg_color: ResMut<BackgroundColor>,
) {
    let (entity, mut timer) = query.into_inner();
    timer.tick(time.delta());
    let end_color = match app_state.get() {
        AppState::Negative => Color::Rgb(191, 0, 0),
        AppState::Positive => Color::Rgb(0, 63, 128),
    };
    bg_color.0 = interpolate(timer.start_color, end_color, timer.fraction())
        .expect("only works for rgb colors");
    if timer.just_finished() {
        commands.entity(entity).despawn();
    }
}

/// Interpolate between two colors based on the fraction
///
/// This is just a simple linear interpolation between the two colors (a real implementation would
/// use a color space that is perceptually uniform).
fn interpolate(start: Color, end: Color, fraction: f32) -> Option<Color> {
    let Color::Rgb(start_red, start_green, start_blue) = start else {
        return None;
    };
    let Color::Rgb(end_red, end_green, end_blue) = end else {
        return None;
    };
    Some(Color::Rgb(
        (start_red as f32 + (end_red as f32 - start_red as f32) * fraction) as u8,
        (start_green as f32 + (end_green as f32 - start_green as f32) * fraction) as u8,
        (start_blue as f32 + (end_blue as f32 - start_blue as f32) * fraction) as u8,
    ))
}
