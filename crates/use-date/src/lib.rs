#![forbid(unsafe_code)]
//! Primitive proleptic Gregorian date helpers.
//!
//! These helpers keep calendar dates explicit and deterministic without adding
//! timezone handling.
//!
//! # Examples
//!
//! ```rust
//! use use_date::{CalendarDate, add_days, day_of_year, days_between, is_valid_date};
//!
//! let leap_day = CalendarDate::new(2024, 2, 29).unwrap();
//! let next_day = add_days(leap_day, 1);
//!
//! assert!(is_valid_date(2024, 2, 29));
//! assert_eq!(day_of_year(2024, 2, 29).unwrap(), 60);
//! assert_eq!(next_day, CalendarDate::new(2024, 3, 1).unwrap());
//! assert_eq!(days_between(leap_day, next_day), 1);
//! ```

const DAYS_BEFORE_MONTH: [u16; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CalendarDate {
    year: i32,
    month: u8,
    day: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateError {
    InvalidMonth,
    InvalidDay,
}

fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn days_in_month(year: i32, month: u8) -> Option<u8> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
        4 | 6 | 9 | 11 => Some(30),
        2 => Some(if is_leap_year(year) { 29 } else { 28 }),
        _ => None,
    }
}

fn day_of_year_unchecked(year: i32, month: u8, day: u8) -> u16 {
    let mut ordinal = DAYS_BEFORE_MONTH[usize::from(month - 1)] + u16::from(day);

    if is_leap_year(year) && month > 2 {
        ordinal += 1;
    }

    ordinal
}

fn days_from_civil(year: i32, month: u8, day: u8) -> i64 {
    let year = i64::from(year) - if month <= 2 { 1 } else { 0 };
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let year_of_era = year - era * 400;
    let month_index = i64::from(month) + if month > 2 { -3 } else { 9 };
    let day_of_year = (153 * month_index + 2) / 5 + i64::from(day) - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;

    era * 146_097 + day_of_era - 719_468
}

fn civil_from_days(days: i64) -> CalendarDate {
    let days = days + 719_468;
    let era = if days >= 0 { days } else { days - 146_096 } / 146_097;
    let day_of_era = days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_index = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_index + 2) / 5 + 1;
    let month = month_index + if month_index < 10 { 3 } else { -9 };
    let year = year + if month <= 2 { 1 } else { 0 };

    CalendarDate {
        year: year as i32,
        month: month as u8,
        day: day as u8,
    }
}

pub fn is_valid_date(year: i32, month: u8, day: u8) -> bool {
    matches!(days_in_month(year, month), Some(days) if (1..=days).contains(&day))
}

impl CalendarDate {
    pub fn new(year: i32, month: u8, day: u8) -> Result<Self, DateError> {
        let max_day = days_in_month(year, month).ok_or(DateError::InvalidMonth)?;

        if !(1..=max_day).contains(&day) {
            return Err(DateError::InvalidDay);
        }

        Ok(Self { year, month, day })
    }

    #[must_use]
    pub fn year(&self) -> i32 {
        self.year
    }

    #[must_use]
    pub fn month(&self) -> u8 {
        self.month
    }

    #[must_use]
    pub fn day(&self) -> u8 {
        self.day
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        is_valid_date(self.year, self.month, self.day)
    }

    #[must_use]
    pub fn day_of_year(&self) -> u16 {
        day_of_year_unchecked(self.year, self.month, self.day)
    }
}

pub fn day_of_year(year: i32, month: u8, day: u8) -> Result<u16, DateError> {
    Ok(CalendarDate::new(year, month, day)?.day_of_year())
}

#[must_use]
pub fn days_between(start: CalendarDate, end: CalendarDate) -> i64 {
    days_from_civil(end.year, end.month, end.day)
        - days_from_civil(start.year, start.month, start.day)
}

#[must_use]
pub fn add_days(date: CalendarDate, days: i64) -> CalendarDate {
    civil_from_days(days_from_civil(date.year, date.month, date.day) + days)
}

#[cfg(test)]
mod tests {
    use super::{CalendarDate, DateError, add_days, day_of_year, days_between, is_valid_date};

    #[test]
    fn validates_dates_and_day_of_year() {
        let leap_day = CalendarDate::new(2024, 2, 29).unwrap();

        assert!(leap_day.is_valid());
        assert!(is_valid_date(2024, 2, 29));
        assert!(!is_valid_date(2023, 2, 29));
        assert_eq!(leap_day.day_of_year(), 60);
        assert_eq!(day_of_year(2024, 12, 31).unwrap(), 366);
    }

    #[test]
    fn adds_days_and_measures_distance() {
        let start = CalendarDate::new(2024, 2, 28).unwrap();
        let end = add_days(start, 2);

        assert_eq!(end, CalendarDate::new(2024, 3, 1).unwrap());
        assert_eq!(days_between(start, end), 2);
        assert_eq!(add_days(end, -2), start);
    }

    #[test]
    fn rejects_invalid_dates() {
        assert_eq!(CalendarDate::new(2024, 13, 1), Err(DateError::InvalidMonth));
        assert_eq!(CalendarDate::new(2024, 2, 0), Err(DateError::InvalidDay));
        assert_eq!(CalendarDate::new(2023, 2, 29), Err(DateError::InvalidDay));
        assert_eq!(day_of_year(2024, 0, 1), Err(DateError::InvalidMonth));
    }
}
