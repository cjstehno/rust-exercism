use std::fmt;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
    minutes: u32
}

impl Clock {
    pub fn new(hours: i16, minutes: i16) -> Clock {
        let mut clock_minutes: u32 = 0;

        if minutes >= 0 {
            clock_minutes = minutes as u32 % 1440;
        } else {
            clock_minutes = 1440 - (i16::abs(minutes) % 1440) as u32;
        }

        if hours >= 0 {
            clock_minutes += hours as u32 * 60 % 1440;
        } else {
            clock_minutes += (1440 - (i16::abs(hours) * 60) % 1440) as u32;
        }

        clock_minutes = clock_minutes % 1440;

        Clock { minutes: clock_minutes }
    }

    pub fn add_minutes(&self, mins: i64) -> Clock {
        unimplemented!()
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