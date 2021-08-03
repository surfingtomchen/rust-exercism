use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Clock {
    pub hours: u32,
    pub minutes: u32,
}

impl Clock {
    fn positive_mod(n: i32, m: u32) -> u32 {
        ((n % m as i32 + m as i32) % m as i32) as u32
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock::from(hours * 60 + minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::from((self.hours * 60 + self.minutes) as i32 + minutes)
    }
}

impl From<i32> for Clock {
    fn from(minutes: i32) -> Self {
        let m = Self::positive_mod(minutes, 60 * 24);
        Clock {
            hours: m / 60,
            minutes: m % 60,
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}