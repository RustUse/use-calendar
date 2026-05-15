#![forbid(unsafe_code)]
//! Primitive calendar quarter helpers.
//!
//! These helpers provide simple calendar quarter boundaries and checks.
//!
//! # Examples
//!
//! ```rust
//! use use_quarter::{Quarter, month_in_quarter, quarter_end_date, quarter_for_month};
//!
//! assert_eq!(quarter_for_month(5).unwrap(), Quarter::Q2);
//! assert_eq!(quarter_end_date(2024, Quarter::Q1).day(), 31);
//! assert!(month_in_quarter(5, Quarter::Q2).unwrap());
//! ```

use use_date::CalendarDate;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quarter {
    Q1,
    Q2,
    Q3,
    Q4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuarterError {
    InvalidMonth,
}

impl Quarter {
    #[must_use]
    pub fn number(&self) -> u8 {
        match self {
            Self::Q1 => 1,
            Self::Q2 => 2,
            Self::Q3 => 3,
            Self::Q4 => 4,
        }
    }

    #[must_use]
    pub fn start_month(&self) -> u8 {
        match self {
            Self::Q1 => 1,
            Self::Q2 => 4,
            Self::Q3 => 7,
            Self::Q4 => 10,
        }
    }

    #[must_use]
    pub fn end_month(&self) -> u8 {
        match self {
            Self::Q1 => 3,
            Self::Q2 => 6,
            Self::Q3 => 9,
            Self::Q4 => 12,
        }
    }
}

pub fn quarter_for_month(month: u8) -> Result<Quarter, QuarterError> {
    Ok(match month {
        1..=3 => Quarter::Q1,
        4..=6 => Quarter::Q2,
        7..=9 => Quarter::Q3,
        10..=12 => Quarter::Q4,
        _ => return Err(QuarterError::InvalidMonth),
    })
}

#[must_use]
pub fn quarter_start_date(year: i32, quarter: Quarter) -> CalendarDate {
    CalendarDate::new(year, quarter.start_month(), 1).unwrap()
}

#[must_use]
pub fn quarter_end_date(year: i32, quarter: Quarter) -> CalendarDate {
    let (month, day) = match quarter {
        Quarter::Q1 => (3, 31),
        Quarter::Q2 => (6, 30),
        Quarter::Q3 => (9, 30),
        Quarter::Q4 => (12, 31),
    };

    CalendarDate::new(year, month, day).unwrap()
}

pub fn month_in_quarter(month: u8, quarter: Quarter) -> Result<bool, QuarterError> {
    Ok(quarter_for_month(month)? == quarter)
}

#[cfg(test)]
mod tests {
    use super::{
        month_in_quarter, quarter_end_date, quarter_for_month, quarter_start_date, Quarter,
        QuarterError,
    };
    use use_date::CalendarDate;

    #[test]
    fn maps_months_to_quarters() {
        assert_eq!(quarter_for_month(1).unwrap(), Quarter::Q1);
        assert_eq!(quarter_for_month(5).unwrap(), Quarter::Q2);
        assert_eq!(quarter_for_month(8).unwrap(), Quarter::Q3);
        assert_eq!(quarter_for_month(12).unwrap(), Quarter::Q4);
        assert_eq!(Quarter::Q3.start_month(), 7);
        assert_eq!(Quarter::Q4.end_month(), 12);
    }

    #[test]
    fn builds_quarter_boundaries() {
        assert_eq!(
            quarter_start_date(2024, Quarter::Q2),
            CalendarDate::new(2024, 4, 1).unwrap()
        );
        assert_eq!(
            quarter_end_date(2024, Quarter::Q2),
            CalendarDate::new(2024, 6, 30).unwrap()
        );
        assert!(month_in_quarter(5, Quarter::Q2).unwrap());
        assert!(!month_in_quarter(7, Quarter::Q2).unwrap());
    }

    #[test]
    fn rejects_invalid_quarter_months() {
        assert_eq!(quarter_for_month(0), Err(QuarterError::InvalidMonth));
        assert_eq!(
            month_in_quarter(13, Quarter::Q1),
            Err(QuarterError::InvalidMonth)
        );
    }
}
