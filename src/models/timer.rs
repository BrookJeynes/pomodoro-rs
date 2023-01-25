use std::time::Duration;

// 25 minutes
const TIME: u64 = 60 * 25;

#[derive(PartialEq)]
pub enum TimerStatus {
    Playing,
    Paused,
}

pub struct Timer {
    pub status: TimerStatus,
    pub timer: Duration,
    pub percentage: u16,
}

fn calculate_time_as_percentage(total_time: f32, time_left: f32) -> u16 {
    ((1.0 - (time_left / total_time)) * 100.0) as u16
}

impl Timer {
    pub fn tick(&mut self) -> Option<()> {
        self.timer = Duration::from(self.timer) - Duration::from_secs(1);
        self.percentage = calculate_time_as_percentage(TIME as f32, self.timer.as_secs() as f32);

        if self.timer.as_secs() == 0 {
            return None;
        }

        Some(())
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
        self.timer.as_secs() % 60
    }

    fn get_minutes(&self) -> u64 {
        (self.timer.as_secs() / 60) % 60
    }

    pub fn mm_ss(&self) -> String {
        format!("{}:{}", self.get_minutes(), self.get_seconds())
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            status: TimerStatus::Paused,
            timer: Duration::from_secs(TIME),
            percentage: 0,
        }
    }
}
