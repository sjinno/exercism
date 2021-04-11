use chrono::{naive, Duration};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: Duration,
    mins: Duration,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            hours: Duration::hours((hours + minutes.div_euclid(60)).rem_euclid(24) as i64),
            mins: Duration::minutes(minutes.rem_euclid(60) as i64),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            hours: self.hours + Duration::hours(minutes.div_euclid(60) as i64),
            mins: self.mins + Duration::minutes(minutes.rem_euclid(60) as i64),
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let naive_time = naive::NaiveTime::from_hms(0, 0, 0) + self.hours + self.mins;
        write!(f, "{}", naive_time.format("%H:%M"))
    }
}
