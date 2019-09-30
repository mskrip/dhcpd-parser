use std::fmt;
use std::cmp;

pub type IPAddress = String;
pub type MACAddress = String;

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct Date {
    pub weekday: i64,
    pub year: i64,
    pub month: i64,
    pub day: i64,
    pub hour: i64,
    pub minute: i64,
    pub second: i64,
}

impl Date {
    pub fn from(weekday: String, date: String, time: String) -> Result<Date, String> {
        // Parses from `weekday year/month/day hour:minute:second` format as
        // specified in OpenBSD man page
        let mut result = Date::new();
        result.weekday = weekday.parse::<i64>().expect("Error parsing weekday");
        if result.weekday < 0 || result.weekday > 6 {
            return Err(format!("Weekday should be a number between 0 and 6. {} is not", weekday));
        }

        let d: Vec<&str> = date.split('/').collect();
        if d.len() != 3 {
            return Err(format!("{} does not have expected date format (YYYY/MM/DD)", date));
        }
        result.year = d[0].to_string().parse::<i64>().expect("Year should be a number");
        result.month = d[1].to_string().parse::<i64>().expect("Month should be a number");
        if result.month < 1 {
            return Err(format!("Month should be a number >= 1. {} is not", result.month));
        }
        result.day = d[2].to_string().parse::<i64>().expect("Day should be a number");
        if result.day < 1 {
            return Err(format!("Day should be a number between >= 1. {} is not", result.day));
        }

        let t: Vec<&str> = time.split(':').collect();
        if t.len() != 3 {
            return Err(format!("{} does not have expected time format (HH:mm:ss)", time));
        }
        result.hour = t[0].to_string().parse::<i64>().expect("Hour should be a number");
        if result.hour < 0 || result.hour > 23 {
            return Err(format!("Hour should be a number between 0 and 23. {} is not", result.hour));
        }
        result.minute = t[1].to_string().parse::<i64>().expect("Minute should be a number");
        if result.minute < 0 || result.hour > 59 {
            return Err(format!("Minute should be a number between 0 and 59. {} is not", result.minute));
        }
        result.second = t[2].to_string().parse::<i64>().expect("Second should be a number");
        if result.hour < 0 || result.hour > 59 {
            return Err(format!("Second should be a number between 0 and 59. {} is not", result.second));
        }

        Ok(result)
    }

    pub fn new() -> Date {
        Date {
            weekday: 0,
            year: 1970,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
        }
    }
    fn weekday_to_string(self) -> String {
        match self.weekday {
            0 => "Sunday".to_owned(),
            1 => "Monday".to_owned(),
            2 => "Tuesday".to_owned(),
            3 => "Wednesday".to_owned(),
            4 => "Thursday".to_owned(),
            5 => "Friday".to_owned(),
            6 => "Saturday".to_owned(),
            _ => "Not a valid weekday".to_owned(),
        }
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,"{} {}/{:0>2}/{:0>2} {:0>2}:{:0>2}:{:0>2}",
            self.weekday_to_string(),
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second,
        )
    }
}

impl cmp::PartialOrd for Date {
    fn partial_cmp(&self, other: &Date) -> Option<cmp::Ordering> {
        if self.year != other.year {
            return self.year.partial_cmp(&other.year);
        }

        if self.month != other.month {
            return self.month.partial_cmp(&other.month);
        }

        if self.day != other.day {
            return self.day.partial_cmp(&other.day);
        }

        if self.hour != other.hour {
            return self.hour.partial_cmp(&other.hour);
        }

        if self.minute != other.minute {
            return self.minute.partial_cmp(&other.minute);
        }

        if self.second != other.second {
            return self.second.partial_cmp(&other.second);
        }

        None
    }
}

impl cmp::Ord for Date {
    fn cmp(&self, other: &Date) -> cmp::Ordering {
        return self.partial_cmp(other).unwrap();
    }
}
