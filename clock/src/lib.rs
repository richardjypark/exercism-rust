use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minute_hours = match minutes {
            m if m >= 0 => m / 60,
            m if m % 60 == 0 => m / 60,
            m => m / 60 - 1,
        };

        let hours_converted = match (hours + minute_hours) % 24 {
            h if h >= 0 => h,
            h => 24 + h,
        };

        let minutes_converted = match (minutes) % 60 {
            m if m >= 0 => m,
            m => 60 + m,
        };

        Clock {
            hours: hours_converted,
            minutes: minutes_converted,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minute_hours = match minutes + self.minutes {
            m if m >= 0 => m / 60,
            m if m % 60 == 0 => m / 60,
            m => m / 60 - 1,
        };

        let hours_converted = match (self.hours + minute_hours) % 24 {
            h if h >= 0 => h,
            h => 24 + h,
        };

        let minutes_converted = match (minutes + self.minutes) % 60 {
            m if m >= 0 => m,
            m => 60 + m,
        };

        Self {
            minutes: minutes_converted,
            hours: hours_converted,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
