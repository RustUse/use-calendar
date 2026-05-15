#![forbid(unsafe_code)]
//! Primitive date range helpers.
//!
//! These helpers work with inclusive date ranges built from valid calendar
//! dates.
//!
//! # Examples
//!
//! ```rust
//! use use_date::CalendarDate;
//! use use_date_range::{DateRange, intersection, overlaps};
//!
//! let start = CalendarDate::new(2024, 1, 1).unwrap();
//! let end = CalendarDate::new(2024, 1, 3).unwrap();
//! let range = DateRange::new(start, end).unwrap();
//! let other = DateRange::new(CalendarDate::new(2024, 1, 3).unwrap(), CalendarDate::new(2024, 1, 5).unwrap()).unwrap();
//!
//! assert!(range.contains(CalendarDate::new(2024, 1, 2).unwrap()));
//! assert_eq!(range.duration_days(), 2);
//! assert!(overlaps(range, other));
//! assert_eq!(intersection(range, other).unwrap().start(), CalendarDate::new(2024, 1, 3).unwrap());
//! ```

use use_date::{add_days, days_between, CalendarDate};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateRange {
    start: CalendarDate,
    end: CalendarDate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateRangeError {
    InvalidRange,
}

impl DateRange {
    pub fn new(start: CalendarDate, end: CalendarDate) -> Result<Self, DateRangeError> {
        if start > end {
            return Err(DateRangeError::InvalidRange);
        }

        Ok(Self { start, end })
    }

    #[must_use]
    pub fn start(&self) -> CalendarDate {
        self.start
    }

    #[must_use]
    pub fn end(&self) -> CalendarDate {
        self.end
    }

    #[must_use]
    pub fn contains(&self, date: CalendarDate) -> bool {
        self.start <= date && date <= self.end
    }

    #[must_use]
    pub fn duration_days(&self) -> i64 {
        days_between(self.start, self.end)
    }
}

pub fn date_range(
    start: CalendarDate,
    end: CalendarDate,
) -> Result<Vec<CalendarDate>, DateRangeError> {
    let range = DateRange::new(start, end)?;

    Ok((0..=range.duration_days())
        .map(|offset| add_days(start, offset))
        .collect())
}

#[must_use]
pub fn overlaps(a: DateRange, b: DateRange) -> bool {
    a.start <= b.end && b.start <= a.end
}

#[must_use]
pub fn intersection(a: DateRange, b: DateRange) -> Option<DateRange> {
    if !overlaps(a, b) {
        return None;
    }

    DateRange::new(a.start.max(b.start), a.end.min(b.end)).ok()
}

#[cfg(test)]
mod tests {
    use super::{date_range, intersection, overlaps, DateRange, DateRangeError};
    use use_date::CalendarDate;

    #[test]
    fn checks_range_containment_and_duration() {
        let start = CalendarDate::new(2024, 1, 1).unwrap();
        let end = CalendarDate::new(2024, 1, 3).unwrap();
        let range = DateRange::new(start, end).unwrap();

        assert_eq!(range.start(), start);
        assert_eq!(range.end(), end);
        assert!(range.contains(CalendarDate::new(2024, 1, 2).unwrap()));
        assert_eq!(range.duration_days(), 2);
        assert_eq!(date_range(start, end).unwrap().len(), 3);
    }

    #[test]
    fn computes_range_overlap_and_intersection() {
        let a = DateRange::new(
            CalendarDate::new(2024, 1, 1).unwrap(),
            CalendarDate::new(2024, 1, 5).unwrap(),
        )
        .unwrap();
        let b = DateRange::new(
            CalendarDate::new(2024, 1, 4).unwrap(),
            CalendarDate::new(2024, 1, 7).unwrap(),
        )
        .unwrap();
        let c = DateRange::new(
            CalendarDate::new(2024, 1, 6).unwrap(),
            CalendarDate::new(2024, 1, 8).unwrap(),
        )
        .unwrap();

        assert!(overlaps(a, b));
        assert!(!overlaps(a, c));
        assert_eq!(
            intersection(a, b).unwrap(),
            DateRange::new(
                CalendarDate::new(2024, 1, 4).unwrap(),
                CalendarDate::new(2024, 1, 5).unwrap()
            )
            .unwrap()
        );
        assert!(intersection(a, c).is_none());
    }

    #[test]
    fn rejects_reversed_ranges() {
        let start = CalendarDate::new(2024, 1, 5).unwrap();
        let end = CalendarDate::new(2024, 1, 1).unwrap();

        assert_eq!(
            DateRange::new(start, end),
            Err(DateRangeError::InvalidRange)
        );
        assert_eq!(date_range(start, end), Err(DateRangeError::InvalidRange));
    }
}
