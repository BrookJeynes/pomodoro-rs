use std::time::Duration;

use super::pomodoro_mode::PomodoroMode;

#[derive(PartialEq)]
pub enum TimerStatus {
    Playing,
    Paused,
}

pub struct Timer {
    pub status: TimerStatus,
    pub time_remaining: Duration,
    pub total_time: Duration,
    pub percentage: u16,
    pub pomodoro_mode: PomodoroMode,
    pub ticked: bool,
}

fn calculate_time_as_percentage(total_time: f32, time_left: f32) -> u16 {
    ((1.0 - (time_left / total_time)) * 100.0) as u16
}

impl Timer {
    pub fn tick(&mut self) {
        self.time_remaining = Duration::from(self.time_remaining) - Duration::from_secs(1);
        self.percentage = calculate_time_as_percentage(
            self.total_time.as_secs() as f32,
            self.time_remaining.as_secs() as f32,
        );
    }

    pub fn pause(&mut self) {
        self.status = TimerStatus::Paused;
    }

    pub fn unpause(&mut self) {
        self.status = TimerStatus::Playing;
    }
}

impl Timer {
    fn get_seconds(&self) -> u64 {
        self.time_remaining.as_secs() % 60
    }

    fn get_minutes(&self) -> u64 {
        (self.time_remaining.as_secs() / 60) % 60
    }

    fn get_hours(&self) -> u64 {
        (self.time_remaining.as_secs() / 60) / 60
    }

    pub fn mm_ss(&self) -> String {
        format!("{}:{}", self.get_minutes(), self.get_seconds())
    }

    pub fn hh_mm_ss(&self) -> String {
        format!("{}:{}:{}", self.get_hours(), self.get_minutes(), self.get_seconds())
    }
}

impl Timer {
    pub fn new(timer: Duration, mode: PomodoroMode) -> Self {
        Self {
            status: TimerStatus::Paused,
            time_remaining: timer,
            percentage: 0,
            total_time: timer,
            pomodoro_mode: mode,
            ticked: false,
        }

    }
}
