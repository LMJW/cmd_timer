/// Timer implementation using condvar and threads
// TODO: maybe implement the timer next time, for now, let's use the time comes with Iced library
use crate::Result;
use anyhow::anyhow;
#[derive(Debug, Default, PartialEq, Eq)]
pub struct TimeoutDuration {
    pub hours: i32,
    pub minutes: i32,
    pub seconds: i32,
    // track the progress of the timeout
    pub progress: i32,
}

impl TimeoutDuration {
    pub fn total_seconds(&self) -> i32 {
        (self.hours * 60 + self.minutes) * 60 + self.seconds
    }

    pub fn is_zero(&self) -> bool {
        self.total_seconds() == 0
    }

    pub fn is_valid(self) -> Result<Self> {
        if self.total_seconds() > 24 * 60 * 60 {
            return Err(anyhow!(
                "Invalid: Cannot have a time that is greater than 24 hours"
            ));
        }
        Ok(self)
    }

    pub fn new(hours: i32, minutes: i32, seconds: i32) -> Self {
        Self {
            hours,
            minutes,
            seconds,
            progress: 0,
        }
    }

    pub fn tick(&mut self) {
        self.progress += 1
    }

    pub fn is_timeout(&self) -> bool {
        self.progress > self.total_seconds()
    }

    pub fn summary(&self) -> String {
        format!(
            "Total time: {} Hour {} Minutes {} Seconds",
            self.hours, self.minutes, self.seconds
        )
    }

    pub fn total_duration(&self) -> String {
        let hour = self.progress / 3600;
        let minute = (self.progress / 60) % 60;
        let seconds = self.progress % 60;
        format!(
            "Total duration: {:02}:{:02}:{:02}",
            &hour, &minute, &seconds
        )
    }
}

impl Into<String> for &TimeoutDuration {
    fn into(self) -> String {
        let current = self.total_seconds() - self.progress;
        let sign = if current > 0 { "" } else { "-" };
        let hour = current.abs() / 3600;
        let minute = (current.abs() / 60) % 60;
        let seconds = current.abs() % 60;
        format!("{}{:02}:{:02}:{:02}", &sign, hour, minute, seconds)
    }
}
