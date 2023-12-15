use std::error;
use std::{
    io::{self, stdout, Stdout},
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{canvas::*, *},
};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub counter: u8,
    pub x: f64,
    pub y: f64,
    pub ball: Circle,
    pub playground: Rect,
    pub vx: f64,
    pub vy: f64,
    pub tick_count: u64,
    pub marker: Marker,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            x: 0.0,
            y: 0.0,
            ball: Circle {
                x: 20.0,
                y: 40.0,
                radius: 10.0,
                color: Color::Red,
            },
            playground: Rect {
                x: 10,
                y: 10,
                width: 200,
                height: 100,
            },
            vx: 1.0,
            vy: 1.0,
            tick_count: 0,
            marker: Marker::Dot,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }

    pub fn map_canvas(&self) -> impl Widget + '_ {
        Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("World"))
            .marker(self.marker)
            .paint(|ctx| {
                ctx.draw(&Map {
                    color: Color::Green,
                    resolution: MapResolution::High,
                });
                ctx.print(self.x, -self.y, "You are here".yellow());
            })
            .x_bounds([-180.0, 180.0])
            .y_bounds([-90.0, 90.0])
    }

    pub fn pong_canvas(&self) -> impl Widget + '_ {
        Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("Pong"))
            .marker(self.marker)
            .paint(|ctx| {
                ctx.draw(&self.ball);
            })
            .x_bounds([10.0, 210.0])
            .y_bounds([10.0, 110.0])
    }

    pub fn boxes_canvas(&self, area: Rect) -> impl Widget {
        let (left, right, bottom, top) =
            (0.0, area.width as f64, 0.0, area.height as f64 * 2.0 - 4.0);
        Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("Rects"))
            .marker(self.marker)
            .x_bounds([left, right])
            .y_bounds([bottom, top])
            .paint(|ctx| {
                for i in 0..=11 {
                    ctx.draw(&Rectangle {
                        x: (i * i + 3 * i) as f64 / 2.0 + 2.0,
                        y: 2.0,
                        width: i as f64,
                        height: i as f64,
                        color: Color::Red,
                    });
                    ctx.draw(&Rectangle {
                        x: (i * i + 3 * i) as f64 / 2.0 + 2.0,
                        y: 21.0,
                        width: i as f64,
                        height: i as f64,
                        color: Color::Blue,
                    });
                }
                for i in 0..100 {
                    if i % 10 != 0 {
                        ctx.print(i as f64 + 1.0, 0.0, format!("{i}", i = i % 10));
                    }
                    if i % 2 == 0 && i % 10 != 0 {
                        ctx.print(0.0, i as f64, format!("{i}", i = i % 10));
                    }
                }
            })
    }
}
