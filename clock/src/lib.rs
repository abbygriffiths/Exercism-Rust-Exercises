use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut hours = hours;

        let mut minutes = minutes;
        while minutes < 0 {
            minutes += 60;
            hours -= 1;
        }

        while hours < 0 {
            hours += 24;
        }

        if minutes >= 60 {
            hours = hours + (minutes / 60);
        }

        Self {
            hours: (hours + 24) % 24,
            minutes: (minutes + 60) % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let new_minutes = self.minutes + minutes;

        Self::new(self.hours, new_minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("{:0>2}:{:0>2}", self.hours, self.minutes).fmt(f)
    }
}
