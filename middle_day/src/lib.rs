use chrono::{NaiveDate, Datelike};

pub mod wd {
    pub use chrono::Weekday::{self, *};
}

pub fn middle_day(year: i32) -> Option<wd::Weekday> {
    // Leap year check
    let is_leap = NaiveDate::from_ymd_opt(year, 12, 31)?.ordinal() == 366;
    if is_leap {
        return None;
    }

    // 183rd day is the middle of 365-day year
    let mid_date = NaiveDate::from_yo_opt(year, 183)?;
    Some(mid_date.weekday())
}
