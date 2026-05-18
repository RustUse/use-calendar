#![forbid(unsafe_code)]
//! Primitive month helpers.
//!
//! These helpers expose explicit month names and Gregorian days-in-month logic.
//!
//! # Examples
//!
//! ```rust
//! use use_month::{Month, days_in_month, month_from_number};
//!
//! let month = month_from_number(2).unwrap();
//!
//! assert_eq!(month, Month::February);
//! assert_eq!(month.short_name(), "Feb");
//! assert_eq!(days_in_month(2024, 2).unwrap(), 29);
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonthError {
    InvalidMonth,
}

fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

impl Month {
    #[must_use]
    pub fn number(&self) -> u8 {
        match self {
            Self::January => 1,
            Self::February => 2,
            Self::March => 3,
            Self::April => 4,
            Self::May => 5,
            Self::June => 6,
            Self::July => 7,
            Self::August => 8,
            Self::September => 9,
            Self::October => 10,
            Self::November => 11,
            Self::December => 12,
        }
    }

    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::January => "January",
            Self::February => "February",
            Self::March => "March",
            Self::April => "April",
            Self::May => "May",
            Self::June => "June",
            Self::July => "July",
            Self::August => "August",
            Self::September => "September",
            Self::October => "October",
            Self::November => "November",
            Self::December => "December",
        }
    }

    #[must_use]
    pub fn short_name(&self) -> &'static str {
        match self {
            Self::January => "Jan",
            Self::February => "Feb",
            Self::March => "Mar",
            Self::April => "Apr",
            Self::May => "May",
            Self::June => "Jun",
            Self::July => "Jul",
            Self::August => "Aug",
            Self::September => "Sep",
            Self::October => "Oct",
            Self::November => "Nov",
            Self::December => "Dec",
        }
    }
}

#[must_use]
pub fn month_from_number(month: u8) -> Option<Month> {
    Some(match month {
        1 => Month::January,
        2 => Month::February,
        3 => Month::March,
        4 => Month::April,
        5 => Month::May,
        6 => Month::June,
        7 => Month::July,
        8 => Month::August,
        9 => Month::September,
        10 => Month::October,
        11 => Month::November,
        12 => Month::December,
        _ => return None,
    })
}

pub fn days_in_month(year: i32, month: u8) -> Result<u8, MonthError> {
    Ok(match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        },
        _ => return Err(MonthError::InvalidMonth),
    })
}

#[must_use]
pub fn is_valid_month(month: u8) -> bool {
    month_from_number(month).is_some()
}

#[cfg(test)]
mod tests {
    use super::{Month, MonthError, days_in_month, is_valid_month, month_from_number};

    #[test]
    fn maps_month_numbers_and_names() {
        let march = month_from_number(3).unwrap();

        assert_eq!(march, Month::March);
        assert_eq!(march.number(), 3);
        assert_eq!(march.name(), "March");
        assert_eq!(march.short_name(), "Mar");
        assert!(is_valid_month(12));
        assert!(!is_valid_month(13));
    }

    #[test]
    fn counts_days_in_month() {
        assert_eq!(days_in_month(2024, 2).unwrap(), 29);
        assert_eq!(days_in_month(2023, 2).unwrap(), 28);
        assert_eq!(days_in_month(2024, 4).unwrap(), 30);
        assert_eq!(days_in_month(2024, 12).unwrap(), 31);
    }

    #[test]
    fn rejects_invalid_month_values() {
        assert_eq!(days_in_month(2024, 0), Err(MonthError::InvalidMonth));
        assert_eq!(days_in_month(2024, 13), Err(MonthError::InvalidMonth));
    }
}
