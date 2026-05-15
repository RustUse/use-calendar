#![forbid(unsafe_code)]
//! Primitive business-day helpers.
//!
//! The first pass treats Monday through Friday as business days and ignores
//! holiday calendars.
//!
//! # Examples
//!
//! ```rust
//! use use_business_day::{BusinessDayConvention, add_business_days, adjust_business_day, is_business_day};
//! use use_date::CalendarDate;
//!
//! let friday = CalendarDate::new(2024, 5, 17).unwrap();
//! let saturday = CalendarDate::new(2024, 5, 18).unwrap();
//!
//! assert!(is_business_day(friday));
//! assert!(!is_business_day(saturday));
//! assert_eq!(add_business_days(friday, 1), CalendarDate::new(2024, 5, 20).unwrap());
//! assert_eq!(adjust_business_day(saturday, BusinessDayConvention::Following), CalendarDate::new(2024, 5, 20).unwrap());
//! ```

use use_date::{add_days, CalendarDate};
use use_weekday::weekday_for_date;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusinessDayConvention {
    Following,
    Preceding,
    ModifiedFollowing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusinessDayError {
    InvalidRange,
}

#[must_use]
pub fn is_business_day(date: CalendarDate) -> bool {
    weekday_for_date(date.year(), date.month(), date.day())
        .unwrap()
        .is_weekday()
}

#[must_use]
pub fn next_business_day(date: CalendarDate) -> CalendarDate {
    let mut current = add_days(date, 1);

    while !is_business_day(current) {
        current = add_days(current, 1);
    }

    current
}

#[must_use]
pub fn previous_business_day(date: CalendarDate) -> CalendarDate {
    let mut current = add_days(date, -1);

    while !is_business_day(current) {
        current = add_days(current, -1);
    }

    current
}

#[must_use]
pub fn add_business_days(date: CalendarDate, days: i64) -> CalendarDate {
    if days == 0 {
        return date;
    }

    let mut current = date;
    let mut remaining = days.abs();
    let step = if days > 0 { 1 } else { -1 };

    while remaining > 0 {
        current = add_days(current, step);

        if is_business_day(current) {
            remaining -= 1;
        }
    }

    current
}

pub fn business_days_between(
    start: CalendarDate,
    end: CalendarDate,
) -> Result<usize, BusinessDayError> {
    if start > end {
        return Err(BusinessDayError::InvalidRange);
    }

    let mut count = 0;
    let mut current = start;

    loop {
        if is_business_day(current) {
            count += 1;
        }

        if current == end {
            break;
        }

        current = add_days(current, 1);
    }

    Ok(count)
}

#[must_use]
pub fn adjust_business_day(date: CalendarDate, convention: BusinessDayConvention) -> CalendarDate {
    if is_business_day(date) {
        return date;
    }

    match convention {
        BusinessDayConvention::Following => next_business_day(date),
        BusinessDayConvention::Preceding => previous_business_day(date),
        BusinessDayConvention::ModifiedFollowing => {
            let following = next_business_day(date);

            if following.month() != date.month() {
                previous_business_day(date)
            } else {
                following
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        add_business_days, adjust_business_day, business_days_between, is_business_day,
        next_business_day, previous_business_day, BusinessDayConvention, BusinessDayError,
    };
    use use_date::CalendarDate;

    #[test]
    fn detects_business_days_and_adjusts() {
        let friday = CalendarDate::new(2024, 5, 17).unwrap();
        let saturday = CalendarDate::new(2024, 5, 18).unwrap();
        let monday = CalendarDate::new(2024, 5, 20).unwrap();

        assert!(is_business_day(friday));
        assert!(!is_business_day(saturday));
        assert_eq!(next_business_day(friday), monday);
        assert_eq!(previous_business_day(monday), friday);
        assert_eq!(
            adjust_business_day(saturday, BusinessDayConvention::Following),
            monday
        );
    }

    #[test]
    fn adds_and_counts_business_days() {
        let friday = CalendarDate::new(2024, 5, 17).unwrap();
        let monday = CalendarDate::new(2024, 5, 20).unwrap();
        let tuesday = CalendarDate::new(2024, 5, 21).unwrap();

        assert_eq!(add_business_days(friday, 1), monday);
        assert_eq!(add_business_days(monday, -1), friday);
        assert_eq!(business_days_between(friday, tuesday).unwrap(), 3);
    }

    #[test]
    fn supports_modified_following_and_reversed_ranges() {
        let saturday = CalendarDate::new(2024, 8, 31).unwrap();
        let friday = CalendarDate::new(2024, 8, 30).unwrap();

        assert_eq!(
            adjust_business_day(saturday, BusinessDayConvention::ModifiedFollowing),
            friday
        );
        assert_eq!(
            business_days_between(
                CalendarDate::new(2024, 5, 21).unwrap(),
                CalendarDate::new(2024, 5, 17).unwrap()
            ),
            Err(BusinessDayError::InvalidRange)
        );
    }
}
