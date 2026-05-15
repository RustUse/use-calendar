#![forbid(unsafe_code)]
//! Primitive calendar year helpers.
//!
//! These helpers keep leap-year and year-boundary calculations explicit.
//!
//! # Examples
//!
//! ```rust
//! use use_year::{CalendarYear, first_day_of_year, last_day_of_year};
//!
//! let year = CalendarYear::new(2024);
//!
//! assert!(year.is_leap_year());
//! assert_eq!(year.days_in_year(), 366);
//! assert_eq!(first_day_of_year(2024).unwrap().month(), 1);
//! assert_eq!(last_day_of_year(2024).unwrap().day(), 31);
//! ```

use use_date::CalendarDate;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalendarYear {
    year: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YearError {
    InvalidDate,
}

#[must_use]
pub fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

#[must_use]
pub fn days_in_year(year: i32) -> u16 {
    if is_leap_year(year) {
        366
    } else {
        365
    }
}

impl CalendarYear {
    #[must_use]
    pub fn new(year: i32) -> Self {
        Self { year }
    }

    #[must_use]
    pub fn year(&self) -> i32 {
        self.year
    }

    #[must_use]
    pub fn is_leap_year(&self) -> bool {
        is_leap_year(self.year)
    }

    #[must_use]
    pub fn days_in_year(&self) -> u16 {
        days_in_year(self.year)
    }
}

pub fn first_day_of_year(year: i32) -> Result<CalendarDate, YearError> {
    CalendarDate::new(year, 1, 1).map_err(|_| YearError::InvalidDate)
}

pub fn last_day_of_year(year: i32) -> Result<CalendarDate, YearError> {
    CalendarDate::new(year, 12, 31).map_err(|_| YearError::InvalidDate)
}

#[cfg(test)]
mod tests {
    use super::{days_in_year, first_day_of_year, is_leap_year, last_day_of_year, CalendarYear};
    use use_date::CalendarDate;

    #[test]
    fn checks_leap_years_and_lengths() {
        let year = CalendarYear::new(2024);

        assert_eq!(year.year(), 2024);
        assert!(year.is_leap_year());
        assert_eq!(year.days_in_year(), 366);
        assert!(!is_leap_year(1900));
        assert!(is_leap_year(2000));
        assert_eq!(days_in_year(2023), 365);
    }

    #[test]
    fn builds_year_boundary_dates() {
        assert_eq!(
            first_day_of_year(2024).unwrap(),
            CalendarDate::new(2024, 1, 1).unwrap()
        );
        assert_eq!(
            last_day_of_year(2024).unwrap(),
            CalendarDate::new(2024, 12, 31).unwrap()
        );
    }
}
