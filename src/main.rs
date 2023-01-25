pub mod models;
pub mod ui;

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use models::pomodoro_mode::PomodoroMode;
use models::study_mode::StudyMode;
use models::timer::{Timer, TimerStatus};
use std::error::Error;
use std::io;
use std::time::{Duration, Instant};
use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;
use ui::ui;

pub struct AppState {
    timer: Timer,
    study_mode: StudyMode,
    pomodoro_mode: PomodoroMode,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            timer: Timer::default(),
            study_mode: StudyMode::Normal,
            pomodoro_mode: PomodoroMode::Pomodoro,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_secs(1);
    let app_state = AppState::default();
    let res = run_app(&mut terminal, app_state, tick_rate);

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

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app_state: AppState,
    tick_rate: Duration,
) -> Result<(), Box<dyn Error>> {
    let mut last_tick = Instant::now();
    let mut next_tick = Some(());

    loop {
        terminal.draw(|f| ui(f, &app_state))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    // Timer interaction keys
                    KeyCode::Char(' ') => match app_state.timer.status {
                        TimerStatus::Paused => app_state.timer.unpause(),
                        TimerStatus::Playing => app_state.timer.pause(),
                    },
                    KeyCode::Char('r') => app_state.timer = Timer::default(),
                    KeyCode::Char('f') => match app_state.study_mode {
                        StudyMode::Normal => app_state.study_mode = StudyMode::Zen,
                        StudyMode::Zen => app_state.study_mode = StudyMode::Normal,
                    },

                    // Exit keys
                    KeyCode::Char('q') => return Ok(()),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            if let Some(_) = next_tick {
                match app_state.timer.status {
                    TimerStatus::Playing => next_tick = app_state.timer.tick(),
                    TimerStatus::Paused => {}
                }
            }

            last_tick = Instant::now();
        }
    }
}
