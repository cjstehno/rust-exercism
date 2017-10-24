use std::fmt;

const MINUTES_IN_DAY: i64 = 1440;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
    minutes: u64
}

impl Clock {
    pub fn new(hours: i64, minutes: i64) -> Clock {
        Clock::create(Clock::normalize_minutes(minutes) + Clock::normalize_minutes(hours * 60))
    }

    fn create(minutes: i64) -> Clock {
        Clock { minutes: (minutes % MINUTES_IN_DAY) as u64 }
    }

    pub fn add_minutes(&self, mins: i64) -> Clock {
        Clock::create(self.minutes as i64 + Clock::normalize_minutes(mins))
    }

    fn normalize_minutes(mins: i64) -> i64 {
        if mins >= 0 {
            mins % MINUTES_IN_DAY
        } else {
            MINUTES_IN_DAY - (i64::abs(mins) % MINUTES_IN_DAY)
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours = self.minutes / 60;
        let minutes = self.minutes % 60;

        write!(
            f,
            "{}:{}",
            if hours < 10 { format!("0{}", hours) } else if hours == 24 { format!("00") } else { format!("{}", hours) },
            if minutes < 10 { format!("0{}", minutes) } else { format!("{}", minutes) }
        )
    }
}
