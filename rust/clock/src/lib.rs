use std::cmp;
use std::fmt;

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_hours = pad_with_zeros(self.hours);
        let display_minutes = pad_with_zeros(self.minutes);
        write!(f, "{}:{}", display_hours, display_minutes)
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hours: {}, minutes: {}", self.hours, self.minutes)
    }
}

impl cmp::PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours_24 = minutes_to_24h(hours * 60 + minutes);
        Clock {
            hours: (hours_24 / 60) % 24,
            minutes: hours_24 % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_minutes = minutes + self.hours * 60 + self.minutes;
        let hours_24 = minutes_to_24h(total_minutes);
        Clock {
            hours: (hours_24 / 60) % 24,
            minutes: hours_24 % 60,
        }
    }
}

fn minutes_to_24h(value: i32) -> i32 {
    if value < 0 {
        // add 1440 until it is more than 0
        return minutes_to_24h(value + 1440);
    } else if value > 1440 {
        // substract 1440 until it is less than 1440
        return minutes_to_24h(value - 1440);
    }
    value
}

fn pad_with_zeros(value: i32) -> String {
    let mut padding = String::from("0");
    let value_str = value.to_string();
    if value > 9 {
        value_str
    } else {
        padding.push_str(&value_str);
        padding
    }
}
