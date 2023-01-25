use core::fmt;

pub enum PomodoroMode {
    Pomodoro,
    ShortBreak,
    LongBreak,
}

impl fmt::Display for PomodoroMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            PomodoroMode::Pomodoro => "Pomodoro",
            PomodoroMode::ShortBreak => "Short Break",
            PomodoroMode::LongBreak => "Long Break",
        };

        write!(f, "{}", text)
    }
}
