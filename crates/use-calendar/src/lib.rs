#![forbid(unsafe_code)]
//! Thin facade for the `use-calendar` workspace.
//!
//! The crate reexports the focused calendar crates directly so consumers can
//! opt into one dependency while still using the smaller APIs.
//!
//! # Examples
//!
//! ```rust
//! use use_calendar::*;
//!
//! let start = CalendarDate::new(2024, 1, 31).unwrap();
//! let quarter = quarter_for_month(start.month()).unwrap();
//! let dates = recurring_dates(start, RecurrenceFrequency::Monthly, 1, 3).unwrap();
//!
//! assert_eq!(quarter, Quarter::Q1);
//! assert_eq!(dates[1], CalendarDate::new(2024, 2, 29).unwrap());
//! assert!(is_business_day(CalendarDate::new(2024, 2, 29).unwrap()));
//! ```

pub use use_business_day;
pub use use_business_day::*;
pub use use_date;
pub use use_date::*;
pub use use_date_range;
pub use use_date_range::*;
pub use use_month;
pub use use_month::*;
pub use use_quarter;
pub use use_quarter::*;
pub use use_recurrence;
pub use use_recurrence::*;
pub use use_weekday;
pub use use_weekday::*;
pub use use_year;
pub use use_year::*;

#[cfg(test)]
mod tests {
    use super::{
        BusinessDayConvention, CalendarDate, Quarter, RecurrenceFrequency, adjust_business_day,
        business_days_between, date_range, quarter_for_month, recurring_dates,
    };

    #[test]
    fn facade_reexports_workspace_apis() {
        let start = CalendarDate::new(2024, 1, 31).unwrap();
        let dates = recurring_dates(start, RecurrenceFrequency::Monthly, 1, 3).unwrap();
        let range = date_range(start, CalendarDate::new(2024, 2, 2).unwrap()).unwrap();

        assert_eq!(quarter_for_month(start.month()).unwrap(), Quarter::Q1);
        assert_eq!(dates[1], CalendarDate::new(2024, 2, 29).unwrap());
        assert_eq!(range.len(), 3);
        assert_eq!(
            business_days_between(start, CalendarDate::new(2024, 2, 2).unwrap()).unwrap(),
            3
        );
        assert_eq!(
            adjust_business_day(
                CalendarDate::new(2024, 2, 3).unwrap(),
                BusinessDayConvention::Following
            ),
            CalendarDate::new(2024, 2, 5).unwrap()
        );
    }
}
