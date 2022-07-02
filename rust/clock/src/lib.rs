use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock(i32);

const MINS_PER_HOUR: i32 = 60;
const MINS_PER_DAY: i32 = MINS_PER_HOUR * 24;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock(
            ((hours * MINS_PER_HOUR + minutes).rem_euclid(MINS_PER_DAY) + MINS_PER_DAY)
                .rem_euclid(MINS_PER_DAY),
        )
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.0 + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            self.0.div_euclid(MINS_PER_HOUR),
            self.0.rem_euclid(MINS_PER_HOUR)
        )
    }
}
