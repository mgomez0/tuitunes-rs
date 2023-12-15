use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ratatui::{
    prelude::*,
    widgets::{canvas::*, *},
};
use std::io::{self, stdout, Stdout};
use std::time::{Duration, Instant};
use tuitunes_templated::handler::handle_key_events;
use tuitunes_templated::tui::Tui;
use tuitunes_templated::{
    app::{App, AppResult},
    ui::render,
};

mod audio_io;

fn main() -> AppResult<()> {
    let mut terminal = init_terminal()?;
    let mut app = App::new();
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(16);
    loop {
        let _ = terminal.draw(|frame| render(&mut app, frame));
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down => app.y += 1.0,
                    KeyCode::Up => app.y -= 1.0,
                    KeyCode::Right => app.x += 1.0,
                    KeyCode::Left => app.x -= 1.0,
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }
    }
    restore_terminal()
}

fn init_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

fn restore_terminal() -> AppResult<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
