pub mod models;
pub mod ui;

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use models::pomodoro_mode::PomodoroMode;
use models::stateful_list::StatefulList;
use models::study_mode::StudyMode;
use models::task::Task;
use models::timer::{Timer, TimerStatus};

use clap::Parser;
use std::error::Error;
use std::io;
use std::path::Path;
use std::time::{Duration, Instant};
use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;
use ui::ui;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Pomodoro timer length.
    #[arg(long, short, default_value_t = 25)]
    pub pomodoro_time: u64,
    /// Short break timer length.
    #[arg(long, short, default_value_t = 5)]
    pub short_break_time: u64,
    /// Long break timer length.
    #[arg(long, short, default_value_t = 15)]
    pub long_break_time: u64,
    /// Path to tasks file.
    #[arg(long, short, default_value_t = String::from("tasks"))]
    pub task_file_path: String,
    /// Whether to open the application in focus mode.
    #[arg(long, short, default_value_t = String::from("false"))]
    pub focus_mode: String,
}

pub struct AppState {
    timer: Timer,
    study_mode: StudyMode,
    tasks: StatefulList<Task>,
    show_help_menu: bool,
}

impl AppState {
    pub fn new(arguments: &Args) -> Self {
        Self {
            timer: Timer::new(
                Duration::from_secs(arguments.pomodoro_time * 60),
                PomodoroMode::Pomodoro,
            ),
            study_mode: match arguments.focus_mode.to_lowercase().as_str() {
                "true" => StudyMode::Zen,
                _ => StudyMode::Normal,
            },
            // Todo: turn path to const
            tasks: StatefulList::with_items(Task::from_file(Path::new(&arguments.task_file_path))),
            show_help_menu: false,
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs(25 * 60), PomodoroMode::Pomodoro),
            study_mode: StudyMode::Normal,
            // Todo: turn path to const
            tasks: StatefulList::with_items(Task::from_file(Path::new("tasks"))),
            show_help_menu: false,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_secs(1);
    let app_state = AppState::new(&args);
    let res = run_app(&mut terminal, app_state, tick_rate, args);

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
    args: Args,
) -> Result<(), Box<dyn Error>> {
    app_state.tasks.next();
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app_state))?;

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
                    KeyCode::Char('r') => match app_state.timer.pomodoro_mode {
                        PomodoroMode::Pomodoro => {
                            app_state.timer = Timer::new(
                                Duration::from_secs(args.pomodoro_time * 60),
                                PomodoroMode::Pomodoro,
                            )
                        }
                        PomodoroMode::LongBreak => {
                            app_state.timer = Timer::new(
                                Duration::from_secs(args.long_break_time * 60),
                                PomodoroMode::LongBreak,
                            )
                        }
                        PomodoroMode::ShortBreak => {
                            app_state.timer = Timer::new(
                                Duration::from_secs(args.short_break_time * 60),
                                PomodoroMode::ShortBreak,
                            )
                        }
                    },
                    KeyCode::Char('f') => match app_state.study_mode {
                        StudyMode::Normal => app_state.study_mode = StudyMode::Zen,
                        StudyMode::Zen => app_state.study_mode = StudyMode::Normal,
                    },

                    // List interaction keys
                    KeyCode::Char('k') | KeyCode::Up => app_state.tasks.previous(),
                    KeyCode::Char('j') | KeyCode::Down => app_state.tasks.next(),
                    KeyCode::Enter => {
                        if let Some(selected) = app_state.tasks.selected() {
                            app_state.tasks.items[selected].complete_task()
                        }
                    }
                    KeyCode::Char('+') => {
                        if let Some(selected) = app_state.tasks.selected() {
                            app_state.tasks.items[selected].complete_pomodoro()
                        }
                    }
                    KeyCode::Char('-') => {
                        if let Some(selected) = app_state.tasks.selected() {
                            app_state.tasks.items[selected].negate_pomodoro()
                        }
                    }

                    // IO interaction keys
                    KeyCode::Char('S') => Task::save(Path::new("tasks"), &app_state.tasks.items)?,

                    // Change Timer controls
                    KeyCode::Char('p') => {
                        app_state.timer = Timer::new(
                            Duration::from_secs(args.pomodoro_time * 60),
                            PomodoroMode::Pomodoro,
                        )
                    }
                    KeyCode::Char('s') => {
                        app_state.timer = Timer::new(
                            Duration::from_secs(args.short_break_time * 60),
                            PomodoroMode::ShortBreak,
                        )
                    }
                    KeyCode::Char('l') => {
                        app_state.timer = Timer::new(
                            Duration::from_secs(args.long_break_time * 60),
                            PomodoroMode::LongBreak,
                        )
                    }

                    // Misc keys
                    KeyCode::Char('?') => app_state.show_help_menu = !app_state.show_help_menu,
                    KeyCode::Char('q') => return Ok(()),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            match app_state.timer.status {
                TimerStatus::Playing => {
                    if app_state.timer.time_remaining.as_secs() != 0 {
                        app_state.timer.tick()
                    }
                }
                TimerStatus::Paused => {}
            }
        }

        last_tick = Instant::now();
    }
}
