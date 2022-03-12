use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        //
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {} minutes to existing Clock time", minutes);
    }
    pub fn to_string(&self) -> String {
        let added_zero_hours = if self.hours < 10 { "0" } else { "" };
        let added_zero_minutes = if self.minutes < 10 { "0" } else { "" };
        format!(
            "{}{}:{}{}",
            added_zero_hours, self.hours, added_zero_minutes, self.minutes
        )
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
