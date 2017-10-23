#[derive(Debug)] #[derive(PartialEq)]
pub struct Clock {
    hours: u16,
    minutes: u16
}

impl Clock {
    pub fn new(hours: i16, minutes: i16) -> Clock {
        let mut the_hours = hours;

        let the_minutes = if minutes < 0 {
            60 + minutes
        } else if minutes >= 60 {
            the_hours += minutes / 60;
            minutes % 60
        } else {
            minutes
        };

        the_hours = if the_hours < 0 {
            24 + (the_hours % 24)
        } else if the_hours >= 23 {
            the_hours % 24
        } else {
            the_hours
        };

        Clock {
            hours: the_hours as u16,
            minutes: the_minutes as u16
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}:{}",
            if self.hours < 10 { format!("0{}", self.hours) } else { format!("{}", self.hours) },
            if self.minutes < 10 { format!("0{}", self.minutes) } else { format!("{}", self.minutes) }
        )
    }

    pub fn add_minutes(&self, mins: i64) -> Clock {
        unimplemented!()
    }
}