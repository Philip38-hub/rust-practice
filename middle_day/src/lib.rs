use chrono::{NaiveDate, Datelike, Weekday as WD};

pub mod wd {
    pub use chrono::Weekday;
}

pub fn middle_day(year: i32) -> Option<wd::Weekday> {
    let is_leap = NaiveDate::from_ymd_opt(year, 12,31)?.ordinal() == 366;

    if is_leap {
        return None;
    }
    let mid_date = NaiveDate::from_yo_opt(year, 183)?;
    Some(mid_date.weekday())
}
