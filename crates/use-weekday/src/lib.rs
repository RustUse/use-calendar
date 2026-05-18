#![forbid(unsafe_code)]
//! Primitive weekday helpers.
//!
//! The helpers here compute weekdays deterministically from Gregorian dates.
//!
//! # Examples
//!
//! ```rust
//! use use_weekday::{Weekday, is_weekend, weekday_for_date};
//!
//! let weekday = weekday_for_date(2024, 5, 17).unwrap();
//!
//! assert_eq!(weekday, Weekday::Friday);
//! assert_eq!(weekday.number_from_monday(), 5);
//! assert!(!is_weekend(weekday));
//! ```

use use_date::{CalendarDate, days_between};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeekdayError {
    InvalidDate,
}

impl Weekday {
    #[must_use]
    pub fn number_from_monday(&self) -> u8 {
        match self {
            Self::Monday => 1,
            Self::Tuesday => 2,
            Self::Wednesday => 3,
            Self::Thursday => 4,
            Self::Friday => 5,
            Self::Saturday => 6,
            Self::Sunday => 7,
        }
    }

    #[must_use]
    pub fn number_from_sunday(&self) -> u8 {
        match self {
            Self::Sunday => 1,
            Self::Monday => 2,
            Self::Tuesday => 3,
            Self::Wednesday => 4,
            Self::Thursday => 5,
            Self::Friday => 6,
            Self::Saturday => 7,
        }
    }

    #[must_use]
    pub fn is_weekend(&self) -> bool {
        matches!(self, Self::Saturday | Self::Sunday)
    }

    #[must_use]
    pub fn is_weekday(&self) -> bool {
        !self.is_weekend()
    }
}

pub fn weekday_for_date(year: i32, month: u8, day: u8) -> Result<Weekday, WeekdayError> {
    let epoch = CalendarDate::new(1970, 1, 1).unwrap();
    let date = CalendarDate::new(year, month, day).map_err(|_| WeekdayError::InvalidDate)?;
    let offset = days_between(epoch, date).rem_euclid(7);

    Ok(match offset {
        0 => Weekday::Thursday,
        1 => Weekday::Friday,
        2 => Weekday::Saturday,
        3 => Weekday::Sunday,
        4 => Weekday::Monday,
        5 => Weekday::Tuesday,
        _ => Weekday::Wednesday,
    })
}

#[must_use]
pub fn is_weekend(weekday: Weekday) -> bool {
    weekday.is_weekend()
}

#[must_use]
pub fn is_weekday(weekday: Weekday) -> bool {
    weekday.is_weekday()
}

#[cfg(test)]
mod tests {
    use super::{Weekday, WeekdayError, is_weekday, is_weekend, weekday_for_date};

    #[test]
    fn calculates_known_weekdays() {
        assert_eq!(weekday_for_date(1970, 1, 1).unwrap(), Weekday::Thursday);
        assert_eq!(weekday_for_date(2024, 2, 29).unwrap(), Weekday::Thursday);
        assert_eq!(weekday_for_date(2024, 5, 17).unwrap(), Weekday::Friday);
    }

    #[test]
    fn checks_weekend_helpers() {
        assert_eq!(Weekday::Monday.number_from_monday(), 1);
        assert_eq!(Weekday::Sunday.number_from_sunday(), 1);
        assert!(Weekday::Saturday.is_weekend());
        assert!(Weekday::Tuesday.is_weekday());
        assert!(is_weekday(Weekday::Friday));
        assert!(is_weekend(Weekday::Sunday));
    }

    #[test]
    fn rejects_invalid_weekday_dates() {
        assert_eq!(
            weekday_for_date(2024, 2, 30),
            Err(WeekdayError::InvalidDate)
        );
    }
}
