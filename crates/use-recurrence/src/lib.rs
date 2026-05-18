#![forbid(unsafe_code)]
//! Primitive recurrence helpers.
//!
//! The first pass supports simple daily, weekly, monthly, and yearly rules
//! without timezone handling.
//!
//! # Examples
//!
//! ```rust
//! use use_date::CalendarDate;
//! use use_recurrence::{RecurrenceFrequency, recurring_dates};
//!
//! let start = CalendarDate::new(2024, 1, 31).unwrap();
//! let dates = recurring_dates(start, RecurrenceFrequency::Monthly, 1, 4).unwrap();
//!
//! assert_eq!(dates[1], CalendarDate::new(2024, 2, 29).unwrap());
//! assert_eq!(dates[2], CalendarDate::new(2024, 3, 31).unwrap());
//! assert_eq!(dates[3], CalendarDate::new(2024, 4, 30).unwrap());
//! ```

use use_date::{CalendarDate, add_days};
use use_month::days_in_month;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecurrenceFrequency {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecurrenceRule {
    start: CalendarDate,
    frequency: RecurrenceFrequency,
    interval: usize,
    count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecurrenceError {
    InvalidInterval,
    InvalidDate,
    DateOverflow,
}

fn add_months_clamped(date: CalendarDate, months: usize) -> Result<CalendarDate, RecurrenceError> {
    let total_months = i64::from(date.year()) * 12
        + i64::from(date.month() - 1)
        + i64::try_from(months).map_err(|_| RecurrenceError::DateOverflow)?;
    let new_year = total_months.div_euclid(12);
    let new_month = total_months.rem_euclid(12) + 1;
    let new_year = i32::try_from(new_year).map_err(|_| RecurrenceError::DateOverflow)?;
    let new_month = u8::try_from(new_month).map_err(|_| RecurrenceError::DateOverflow)?;
    let max_day = days_in_month(new_year, new_month).map_err(|_| RecurrenceError::InvalidDate)?;
    let new_day = date.day().min(max_day);

    CalendarDate::new(new_year, new_month, new_day).map_err(|_| RecurrenceError::InvalidDate)
}

fn add_years_clamped(date: CalendarDate, years: usize) -> Result<CalendarDate, RecurrenceError> {
    let delta = i32::try_from(years).map_err(|_| RecurrenceError::DateOverflow)?;
    let new_year = date
        .year()
        .checked_add(delta)
        .ok_or(RecurrenceError::DateOverflow)?;
    let max_day =
        days_in_month(new_year, date.month()).map_err(|_| RecurrenceError::InvalidDate)?;
    let new_day = date.day().min(max_day);

    CalendarDate::new(new_year, date.month(), new_day).map_err(|_| RecurrenceError::InvalidDate)
}

fn advance_date(
    start: CalendarDate,
    frequency: RecurrenceFrequency,
    interval: usize,
    occurrence_index: usize,
) -> Result<CalendarDate, RecurrenceError> {
    let total = interval
        .checked_mul(occurrence_index)
        .ok_or(RecurrenceError::DateOverflow)?;

    match frequency {
        RecurrenceFrequency::Daily => {
            let days = i64::try_from(total).map_err(|_| RecurrenceError::DateOverflow)?;
            Ok(add_days(start, days))
        },
        RecurrenceFrequency::Weekly => {
            let days = total.checked_mul(7).ok_or(RecurrenceError::DateOverflow)?;
            let days = i64::try_from(days).map_err(|_| RecurrenceError::DateOverflow)?;
            Ok(add_days(start, days))
        },
        RecurrenceFrequency::Monthly => add_months_clamped(start, total),
        RecurrenceFrequency::Yearly => add_years_clamped(start, total),
    }
}

impl RecurrenceRule {
    pub fn new(
        start: CalendarDate,
        frequency: RecurrenceFrequency,
        interval: usize,
        count: usize,
    ) -> Result<Self, RecurrenceError> {
        if interval == 0 {
            return Err(RecurrenceError::InvalidInterval);
        }

        Ok(Self {
            start,
            frequency,
            interval,
            count,
        })
    }

    pub fn dates(&self) -> Result<Vec<CalendarDate>, RecurrenceError> {
        recurring_dates(self.start, self.frequency, self.interval, self.count)
    }
}

pub fn next_date(
    date: CalendarDate,
    frequency: RecurrenceFrequency,
    interval: usize,
) -> Result<CalendarDate, RecurrenceError> {
    if interval == 0 {
        return Err(RecurrenceError::InvalidInterval);
    }

    advance_date(date, frequency, interval, 1)
}

pub fn recurring_dates(
    start: CalendarDate,
    frequency: RecurrenceFrequency,
    interval: usize,
    count: usize,
) -> Result<Vec<CalendarDate>, RecurrenceError> {
    if interval == 0 {
        return Err(RecurrenceError::InvalidInterval);
    }

    let mut dates = Vec::with_capacity(count);

    for occurrence_index in 0..count {
        dates.push(advance_date(start, frequency, interval, occurrence_index)?);
    }

    Ok(dates)
}

#[cfg(test)]
mod tests {
    use super::{RecurrenceError, RecurrenceFrequency, RecurrenceRule, next_date, recurring_dates};
    use use_date::CalendarDate;

    #[test]
    fn generates_daily_and_weekly_recurrences() {
        let start = CalendarDate::new(2024, 5, 17).unwrap();
        let daily = recurring_dates(start, RecurrenceFrequency::Daily, 1, 3).unwrap();
        let weekly = RecurrenceRule::new(start, RecurrenceFrequency::Weekly, 2, 3)
            .unwrap()
            .dates()
            .unwrap();

        assert_eq!(daily[0], CalendarDate::new(2024, 5, 17).unwrap());
        assert_eq!(daily[2], CalendarDate::new(2024, 5, 19).unwrap());
        assert_eq!(weekly[1], CalendarDate::new(2024, 5, 31).unwrap());
        assert_eq!(
            next_date(start, RecurrenceFrequency::Weekly, 1).unwrap(),
            CalendarDate::new(2024, 5, 24).unwrap()
        );
    }

    #[test]
    fn clamps_monthly_end_of_month_recurrences() {
        let start = CalendarDate::new(2024, 1, 31).unwrap();
        let dates = recurring_dates(start, RecurrenceFrequency::Monthly, 1, 4).unwrap();

        assert_eq!(dates[0], CalendarDate::new(2024, 1, 31).unwrap());
        assert_eq!(dates[1], CalendarDate::new(2024, 2, 29).unwrap());
        assert_eq!(dates[2], CalendarDate::new(2024, 3, 31).unwrap());
        assert_eq!(dates[3], CalendarDate::new(2024, 4, 30).unwrap());
        assert_eq!(
            next_date(start, RecurrenceFrequency::Monthly, 1).unwrap(),
            CalendarDate::new(2024, 2, 29).unwrap()
        );
    }

    #[test]
    fn rejects_invalid_intervals_and_allows_empty_count() {
        let start = CalendarDate::new(2024, 2, 29).unwrap();

        assert_eq!(
            RecurrenceRule::new(start, RecurrenceFrequency::Daily, 0, 1),
            Err(RecurrenceError::InvalidInterval)
        );
        assert_eq!(
            recurring_dates(start, RecurrenceFrequency::Yearly, 1, 0).unwrap(),
            Vec::<CalendarDate>::new()
        );
        assert_eq!(
            next_date(start, RecurrenceFrequency::Yearly, 1).unwrap(),
            CalendarDate::new(2025, 2, 28).unwrap()
        );
    }
}
