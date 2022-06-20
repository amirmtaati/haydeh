use crate::app::{app::App, audio::Audio};
use crate::ui::*;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

pub fn run(tick_rate: Duration) -> Result<(), io::Error> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new()?;
    let res = draw(app, &mut terminal, tick_rate);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

pub fn draw<B: Backend>(
    mut app: App,
    terminal: &mut Terminal<B>,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            handle_event(&mut app)?;
        }

        if last_tick.elapsed() >= tick_rate {
            app.data.artist_songs = Audio::get_artist_songs(
                app.data.artists[app.artist_index].clone(),
                &app.data.audio_path,
            )?;
            last_tick = Instant::now();
        }

        if app.states.should_quit {
            return Ok(());
        }
    }
}

pub fn handle_event(app: &mut App) -> Result<(), io::Error> {
    if let Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Char(c) => {
                app.on_key(c);
            }
            KeyCode::Down => {
                app.artists_down();
            }
            KeyCode::Up => {
                app.artists_up();
            }
            KeyCode::Enter => {
                app.on_play();
            }
            _ => {}
        }
    }

    Ok(())
}
