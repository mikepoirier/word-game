use chrono::Duration as ChronoDuration;
use std::{convert::TryInto, fmt::Display, time::Duration as StdDuration};

#[derive(Debug, Clone)]
pub enum FormattedDuration {
    FormattedChronoDuration(ChronoDuration),
    FormattedStdDuration(StdDuration),
}

impl From<ChronoDuration> for FormattedDuration {
    fn from(duration: ChronoDuration) -> Self {
        FormattedDuration::FormattedChronoDuration(duration)
    }
}

impl From<StdDuration> for FormattedDuration {
    fn from(duration: StdDuration) -> Self {
        FormattedDuration::FormattedStdDuration(duration)
    }
}

impl Display for FormattedDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let secs = match self {
            FormattedDuration::FormattedChronoDuration(d) => d.num_seconds(),
            FormattedDuration::FormattedStdDuration(d) => d.as_secs().try_into().expect("Could not convert seconds to i64"),
        };

        let nanos = match self {
            FormattedDuration::FormattedChronoDuration(d) => d.num_nanoseconds().unwrap(),
            FormattedDuration::FormattedStdDuration(d) => d.subsec_nanos().into(),
        };

        if secs == 0 && nanos == 0 {
            return write!(f, "0s")
        }

        let years = secs / 31_557_600; //365.25 days
        let ydays = secs % 31_557_600;
        let months = ydays / 2_630_016;
        let mdays = ydays % 2_630_016;
        let days = mdays / 86400;
        let day_secs = mdays % 86400;
        let hours = day_secs / 3600;
        let minutes = day_secs % 3600 / 60;
        let seconds = day_secs % 60;
        let millis = nanos / 1_000_000;
        let micros = nanos / 1000 % 1000;
        let nanosec = nanos % 1000;

        format_plural(f, years, "year")?;
        format_plural(f, months, "month")?;
        format_plural(f, days, "day")?;
        format_item(f, hours, "h")?;
        format_item(f, minutes, "m")?;
        format_item(f, seconds, "s")?;
        format_item(f, millis, "ms")?;
        format_item(f, micros, "us")?;
        format_item(f, nanosec, "ns")
    }
}

fn format_plural(f: &mut std::fmt::Formatter<'_>, item: i64, name: &str) -> std::fmt::Result {
    if item > 0 {
        if item > 1 {
            write!(f, "{}{}s", item, name)
        } else {
            write!(f, "{}{}", item, name)
        }
    } else {
        Ok(())
    }
}

fn format_item (f: &mut std::fmt::Formatter<'_>, item: i64, name: &str) -> std::fmt::Result {
    if item > 0 {
        write!(f, "{}{}", item, name)
    } else {
        Ok(())
    }
}
