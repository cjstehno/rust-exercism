use std::fmt;

const MINUTES_IN_DAY: u64 = 1440;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
    minutes: u64
}

impl Clock {
    pub fn new(hours: i16, minutes: i16) -> Clock {
        let mut clock_minutes: u64 = 0;

        if minutes >= 0 {
            clock_minutes = minutes as u64 % MINUTES_IN_DAY;
        } else {
            clock_minutes = MINUTES_IN_DAY - (i16::abs(minutes) as u64 % MINUTES_IN_DAY);
        }

        if hours >= 0 {
            clock_minutes += hours as u64 * 60 % MINUTES_IN_DAY;
        } else {
            clock_minutes += MINUTES_IN_DAY - (i16::abs(hours) * 60) as u64 % MINUTES_IN_DAY;
        }

        clock_minutes = clock_minutes % MINUTES_IN_DAY;

        Clock { minutes: clock_minutes }
    }

    pub fn add_minutes(&self, mins: i64) -> Clock {
        let clock_minutes = (self.minutes + mins as u64) % MINUTES_IN_DAY;
        Clock { minutes: clock_minutes }
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