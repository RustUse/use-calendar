# use-calendar

Composable primitive calendar utilities for Rust.

`use-calendar` is part of RustUse, alongside sibling repositories such as
`use-math`, `use-stats`, `use-optimization`, `use-simulation`,
`use-control`, `use-signal`, `use-graph`, `use-materials`,
`use-accessibility`, `use-typography`, `use-time`, and `use-units`.
It groups small, focused crates for dates, weekdays, months, years,
quarters, date ranges, business-day helpers, and simple recurrences.

The RustUse approach in this workspace stays intentionally narrow:

- crates stay small and independently useful
- APIs stay explicit, documented, tested, and composable
- implementations favor practical `i32`, `u8`, `usize`, `bool`, and small enums or structs
- dependencies stay minimal so each crate is easy to audit and adopt

These crates use simple proleptic Gregorian calendar helpers in the first
pass. They do not provide timezone handling yet, and they are not a full
scheduling engine.

## Workspace crates

- `use-calendar`: thin facade crate that reexports the full calendar workspace
- `use-date`: primitive calendar date helpers
- `use-weekday`: deterministic Gregorian weekday helpers
- `use-month`: month lookup and days-in-month helpers
- `use-year`: leap-year and year-boundary helpers
- `use-quarter`: calendar quarter and simple fiscal-period boundary helpers
- `use-date-range`: inclusive date range helpers
- `use-business-day`: Monday-through-Friday business-day helpers
- `use-recurrence`: simple daily, weekly, monthly, and yearly recurrences

## Facade crate

If you want one dependency for the whole workspace, use `use-calendar`.
It reexports each focused crate and exposes the focused APIs directly so this
works:

```rust
use use_calendar::*;

let start = CalendarDate::new(2024, 1, 31).unwrap();
let quarter = quarter_for_month(start.month()).unwrap();
let dates = recurring_dates(start, RecurrenceFrequency::Monthly, 1, 3).unwrap();

assert_eq!(quarter, Quarter::Q1);
assert_eq!(dates[1], CalendarDate::new(2024, 2, 29).unwrap());
assert!(is_business_day(CalendarDate::new(2024, 2, 29).unwrap()));
```

## Status

This workspace is experimental while it remains below `0.3.0`. Expect the
public API to stay small and practical, but still evolve as the RustUse
calendar surface becomes clearer.

## Development

Run the standard workspace checks from the repository root:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo doc --workspace --no-deps
```