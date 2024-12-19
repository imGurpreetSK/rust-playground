use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut mm = minutes % 60;
        let mut hh = (hours + minutes / 60) % 24;
        if mm < 0 {
            mm = 60 + mm;
            hh = hh - 1;
        }
        if hh < 0 {
            hh = 24 + hh
        }

        Clock {
            hours: hh,
            minutes: mm,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
