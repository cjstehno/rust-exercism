use std::fmt;

const MINUTES_IN_DAY: i64 = 1440;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
    minutes: u64
}

impl Clock {
    pub fn new(hours: i64, minutes: i64) -> Clock {
        let mut clock_minutes: i64 = if minutes >= 0 {
            minutes % MINUTES_IN_DAY
        } else {
            MINUTES_IN_DAY - (i64::abs(minutes) % MINUTES_IN_DAY)
        };

        if hours >=0 {
            clock_minutes += (hours * 60) % MINUTES_IN_DAY;
        } else {
            clock_minutes += MINUTES_IN_DAY - (i64::abs(hours * 60) % MINUTES_IN_DAY);
        }

        Clock { minutes: (clock_minutes % MINUTES_IN_DAY) as u64 }
    }

    pub fn add_minutes(&self, mins: i64) -> Clock {
        let mut clock_minutes: i64 = self.minutes as i64;

        if mins >= 0 {
            clock_minutes += mins % MINUTES_IN_DAY;
        } else {
            clock_minutes += MINUTES_IN_DAY - (i64::abs(mins) % MINUTES_IN_DAY)
        };

        Clock {
            minutes: (clock_minutes % MINUTES_IN_DAY) as u64
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
