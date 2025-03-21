use anyhow::Result;
use re_log::ResultExt as _;
use std::ops::RangeInclusive;
use time::{format_description::FormatItem, OffsetDateTime, UtcOffset};

use super::Duration;

/// How to display a [`Time`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum TimeZone {
    /// Convert to local timezone and display as such.
    Local,

    /// Display as UTC.
    Utc,

    /// Show as seconds since unix epoch
    UnixEpoch,
}

/// A date-time represented as nanoseconds since unix epoch
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Time(i64);

impl Time {
    #[inline]
    pub fn now() -> Self {
        let nanos_since_epoch = web_time::SystemTime::UNIX_EPOCH
            .elapsed()
            .expect("Expected system clock to be set to after 1970")
            .as_nanos() as _;
        Self(nanos_since_epoch)
    }

    #[inline]
    pub fn nanos_since_epoch(&self) -> i64 {
        self.0
    }

    #[inline]
    pub fn from_ns_since_epoch(ns_since_epoch: i64) -> Self {
        Self(ns_since_epoch)
    }

    #[inline]
    pub fn from_us_since_epoch(us_since_epoch: i64) -> Self {
        Self(us_since_epoch * 1_000)
    }

    #[inline]
    pub fn from_seconds_since_epoch(secs: f64) -> Self {
        Self::from_ns_since_epoch((secs * 1e9).round() as _)
    }

    /// If true, this time is likely relative to unix epoch.
    pub fn is_absolute_date(&self) -> bool {
        let nanos_since_epoch = self.nanos_since_epoch();
        let years_since_epoch = nanos_since_epoch / 1_000_000_000 / 60 / 60 / 24 / 365;

        20 <= years_since_epoch && years_since_epoch <= 150
    }

    /// Returns the absolute datetime if applicable.
    pub fn to_datetime(self) -> Option<OffsetDateTime> {
        let ns_since_epoch = self.nanos_since_epoch();
        if self.is_absolute_date() {
            OffsetDateTime::from_unix_timestamp_nanos(ns_since_epoch as i128).ok()
        } else {
            None
        }
    }

    pub fn is_exactly_midnight(&self) -> bool {
        // This is correct despite leap seconds because
        // during positive leap seconds, UTC actually has a discontinuity
        // (the same integer is reused for two different times).
        // See https://en.wikipedia.org/wiki/Unix_time#Leap_seconds
        self.nanos_since_epoch() % (24 * 60 * 60 * 1_000_000_000) == 0
    }

    fn time_string(
        datetime: OffsetDateTime,
        parsed_format: &Vec<FormatItem<'_>>,
        time_zone_for_timestamps: TimeZone,
    ) -> String {
        let r = (|| -> Result<String, time::error::Format> {
            match time_zone_for_timestamps {
                TimeZone::Local => {
                    if let Ok(local_offset) = UtcOffset::current_local_offset() {
                        // Return in the local timezone.
                        let local_datetime = datetime.to_offset(local_offset);
                        local_datetime.format(&parsed_format)
                    } else {
                        // Fallback to UTC.
                        // Skipping `err` description from logging because as of writing it doesn't add much, see
                        // https://github.com/time-rs/time/blob/v0.3.29/time/src/error/indeterminate_offset.rs
                        re_log::warn_once!("Failed to access local timezone offset to UTC.");
                        Ok(format!("{}Z", datetime.format(&parsed_format)?))
                    }
                }
                TimeZone::Utc => Ok(format!("{}Z", datetime.format(&parsed_format)?)),
                TimeZone::UnixEpoch => datetime.format(&parsed_format),
            }
        })();
        r.ok_or_log_error().unwrap_or_default()
    }

    /// Human-readable formatting
    pub fn format(&self, time_zone_for_timestamps: TimeZone) -> String {
        let nanos_since_epoch = self.nanos_since_epoch();

        if let Some(datetime) = self.to_datetime() {
            let is_whole_second = nanos_since_epoch % 1_000_000_000 == 0;
            let is_whole_millisecond = nanos_since_epoch % 1_000_000 == 0;
            let prefix = match time_zone_for_timestamps {
                TimeZone::UnixEpoch => "[unix_timestamp]",
                TimeZone::Utc | TimeZone::Local => "[hour]:[minute]:[second]",
            };

            let time_format = if is_whole_second {
                prefix.to_owned()
            } else if is_whole_millisecond {
                format!("{prefix}.[subsecond digits:3]")
            } else {
                format!("{prefix}.[subsecond digits:6]")
            };

            let date_is_today = datetime.date() == OffsetDateTime::now_utc().date();
            let date_format = format!("[year]-[month]-[day] {time_format}");
            #[allow(clippy::unwrap_used)] // date_format is okay!
            let parsed_format = if date_is_today {
                time::format_description::parse(&time_format).unwrap()
            } else {
                time::format_description::parse(&date_format).unwrap()
            };

            Self::time_string(datetime, &parsed_format, time_zone_for_timestamps)
        } else {
            // Relative time
            let secs = nanos_since_epoch as f64 * 1e-9;

            let is_whole_second = nanos_since_epoch % 1_000_000_000 == 0;

            let secs = re_format::FloatFormatOptions::DEFAULT_f64
                .with_always_sign(true)
                .with_decimals(if is_whole_second { 0 } else { 3 })
                .with_strip_trailing_zeros(false)
                .format(secs);
            format!("{secs}s")
        }
    }

    /// Best effort parsing of a string into a [`Time`] from a human readable string.
    pub fn parse(s: &str, time_zone_for_timestamps: TimeZone) -> Option<Self> {
        // Try parsing as a simple relative time with unit suffix (e.g. "1.234s", "1.234ms")
        let suffixes = [("s", 1e9), ("ms", 1e6), ("us", 1e3), ("ns", 1.0)];
        for (suffix, to_ns) in suffixes {
            if let Some(s) = s.strip_suffix(suffix) {
                if let Ok(value) = s.parse::<f64>() {
                    return Some(Self::from_ns_since_epoch((value * to_ns).round() as i64));
                }
            }
        }

        // Parse a relative time of day.
        let time_formats = [
            // Just time with milliseconds
            "[hour]:[minute]:[second].[subsecond]",
            // Just time
            "[hour]:[minute]:[second]",
        ];
        for format in time_formats {
            let format = time::format_description::parse_borrowed::<2>(format)
                .expect("Invalid format string");

            if let Ok(time) = time::Time::parse(s, &format).map(|t| {
                let (h, m, s, ns) = t.as_hms_nano();
                Self::from_ns_since_epoch(
                    ns as i64
                        + s as i64 * time::convert::Nanosecond::per(time::convert::Second) as i64
                        + m as i64 * time::convert::Nanosecond::per(time::convert::Minute) as i64
                        + h as i64 * time::convert::Nanosecond::per(time::convert::Hour) as i64,
                )
            }) {
                return Some(time);
            }
        }

        // Try parsing as an absolute time
        fn parse_date_time(
            s: &str,
            format_str: &str,
            time_zone_for_timestamps: TimeZone,
        ) -> Option<Time> {
            let format = time::format_description::parse_borrowed::<2>(format_str)
                .expect("Invalid format string");

            // If the string ends with Z, this is a UTC timezone independent of the time zone we set earlier.
            let (s, time_zone_for_timestamps) = if let Some(s) = s.strip_suffix('Z') {
                (s, TimeZone::Utc)
            } else {
                (s, time_zone_for_timestamps)
            };

            let datetime = match time_zone_for_timestamps {
                TimeZone::UnixEpoch | TimeZone::Utc => time::PrimitiveDateTime::parse(s, &format)
                    .ok()?
                    .assume_utc(),

                TimeZone::Local => {
                    if let Ok(local_offset) = UtcOffset::current_local_offset() {
                        let datetime = time::PrimitiveDateTime::parse(s, &format).ok()?;
                        datetime.assume_offset(local_offset)
                    } else {
                        return None;
                    }
                }
            };

            Some(Time::from_ns_since_epoch(
                datetime.unix_timestamp_nanos() as i64
            ))
        }

        // Try different format patterns
        let date_time_formats = [
            // Full date and time with milliseconds
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]",
            // Full date and time
            "[year]-[month]-[day] [hour]:[minute]:[second]",
        ];
        for format in date_time_formats {
            if let Some(time) = parse_date_time(s, format, time_zone_for_timestamps) {
                return Some(time);
            }
        }

        None
    }

    /// Useful when showing dates/times on a timeline
    /// and you want it compact.
    ///
    /// Shows dates when zoomed out, shows times when zoomed in,
    /// shows relative millisecond when really zoomed in.
    pub fn format_time_compact(&self, time_zone_for_timestamps: TimeZone) -> String {
        let ns = self.nanos_since_epoch();
        let relative_ns = ns % 1_000_000_000;
        let is_whole_second = relative_ns == 0;
        if is_whole_second {
            if let Some(datetime) = self.to_datetime() {
                let is_whole_minute = ns % 60_000_000_000 == 0;
                let time_format = if self.is_exactly_midnight() {
                    "[year]-[month]-[day]"
                } else if is_whole_minute {
                    match time_zone_for_timestamps {
                        TimeZone::UnixEpoch => "[unix_timestamp]",
                        TimeZone::Utc | TimeZone::Local => "[hour]:[minute]",
                    }
                } else {
                    match time_zone_for_timestamps {
                        TimeZone::UnixEpoch => "[unix_timestamp]",
                        TimeZone::Utc | TimeZone::Local => "[hour]:[minute]:[second]",
                    }
                };
                #[allow(clippy::unwrap_used)] // time_format is okay!
                let parsed_format = time::format_description::parse(time_format).unwrap();

                return Self::time_string(datetime, &parsed_format, time_zone_for_timestamps);
            }

            crate::Duration::from_nanos(ns).to_string()
        } else {
            // We are in the sub-second resolution.
            // Showing the full time (HH:MM:SS.XXX or 3h 2m 6s …) becomes too long,
            // so instead we switch to showing the time as milliseconds since the last whole second:
            let ms = relative_ns as f64 * 1e-6;
            if relative_ns % 1_000_000 == 0 {
                format!("{ms:+.0} ms")
            } else if relative_ns % 100_000 == 0 {
                format!("{ms:+.1} ms")
            } else if relative_ns % 10_000 == 0 {
                format!("{ms:+.2} ms")
            } else if relative_ns % 1_000 == 0 {
                format!("{ms:+.3} ms")
            } else if relative_ns % 100 == 0 {
                format!("{ms:+.4} ms")
            } else if relative_ns % 10 == 0 {
                format!("{ms:+.5} ms")
            } else {
                format!("{ms:+.6} ms")
            }
        }
    }

    // Human-readable formatting with a custom time_format.
    pub fn format_time_custom(
        &self,
        time_format: &str,
        time_zone_for_timestamps: TimeZone,
    ) -> Option<String> {
        let datetime = self.to_datetime()?;
        let parsed_format = time::format_description::parse(time_format).ok()?;

        Some(Self::time_string(
            datetime,
            &parsed_format,
            time_zone_for_timestamps,
        ))
    }

    #[inline]
    pub fn lerp(range: RangeInclusive<Self>, t: f32) -> Self {
        let (min, max) = (range.start().0, range.end().0);
        Self(min + ((max - min) as f64 * (t as f64)).round() as i64)
    }
}

impl std::fmt::Debug for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format(TimeZone::Utc).fmt(f)
    }
}

impl std::ops::Sub for Time {
    type Output = Duration;

    #[inline]
    fn sub(self, rhs: Self) -> Duration {
        Duration::from_nanos(self.0.saturating_sub(rhs.0))
    }
}

impl std::ops::Add<Duration> for Time {
    type Output = Self;

    #[inline]
    fn add(self, duration: Duration) -> Self::Output {
        Self(self.0.saturating_add(duration.as_nanos()))
    }
}

impl std::ops::AddAssign<Duration> for Time {
    #[inline]
    fn add_assign(&mut self, duration: Duration) {
        self.0 = self.0.saturating_add(duration.as_nanos());
    }
}

impl std::ops::Sub<Duration> for Time {
    type Output = Self;

    #[inline]
    fn sub(self, duration: Duration) -> Self::Output {
        Self(self.0.saturating_sub(duration.as_nanos()))
    }
}

impl TryFrom<std::time::SystemTime> for Time {
    type Error = std::time::SystemTimeError;

    fn try_from(time: std::time::SystemTime) -> Result<Self, Self::Error> {
        time.duration_since(std::time::SystemTime::UNIX_EPOCH)
            .map(|duration_since_epoch| Self(duration_since_epoch.as_nanos() as _))
    }
}

// On non-wasm32 builds, `web_time::SystemTime` is a re-export of `std::time::SystemTime`,
// so it's covered by the above `TryFrom`.
#[cfg(target_arch = "wasm32")]
impl TryFrom<web_time::SystemTime> for Time {
    type Error = web_time::SystemTimeError;

    fn try_from(time: web_time::SystemTime) -> Result<Self, Self::Error> {
        time.duration_since(web_time::SystemTime::UNIX_EPOCH)
            .map(|duration_since_epoch| Self(duration_since_epoch.as_nanos() as _))
    }
}

impl TryFrom<time::OffsetDateTime> for Time {
    type Error = core::num::TryFromIntError;

    fn try_from(datetime: time::OffsetDateTime) -> Result<Self, Self::Error> {
        i64::try_from(datetime.unix_timestamp_nanos()).map(Self::from_ns_since_epoch)
    }
}

// ---------------

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::{datetime, time};

    #[test]
    fn test_formatting_short_times() {
        assert_eq!(
            &Time::from_us_since_epoch(42_000_000).format(TimeZone::Utc),
            "+42s"
        );
        assert_eq!(
            &Time::from_us_since_epoch(69_000).format(TimeZone::Utc),
            "+0.069s"
        );
        assert_eq!(
            &Time::from_us_since_epoch(69_900).format(TimeZone::Utc),
            "+0.070s"
        );

        assert_eq!(
            &Time::from_us_since_epoch(42_000_000).format(TimeZone::Local),
            "+42s"
        );
        assert_eq!(
            &Time::from_us_since_epoch(42_123_000_000).format(TimeZone::Local),
            "+42 123s"
        );
        assert_eq!(
            &Time::from_us_since_epoch(69_000).format(TimeZone::Local),
            "+0.069s"
        );
        assert_eq!(
            &Time::from_us_since_epoch(69_900).format(TimeZone::Local),
            "+0.070s"
        );

        assert_eq!(
            &Time::from_us_since_epoch(42_000_000).format(TimeZone::UnixEpoch),
            "+42s"
        );
        assert_eq!(
            &Time::from_us_since_epoch(69_000).format(TimeZone::UnixEpoch),
            "+0.069s"
        );
        assert_eq!(
            &Time::from_us_since_epoch(69_900).format(TimeZone::UnixEpoch),
            "+0.070s"
        );
    }

    #[test]
    fn test_formatting_whole_second_for_datetime() {
        let datetime = Time::try_from(datetime!(2022-02-28 22:35:42 UTC)).unwrap();
        assert_eq!(&datetime.format(TimeZone::Utc), "2022-02-28 22:35:42Z");
    }

    #[test]
    fn test_formatting_whole_millisecond_for_datetime() {
        let datetime = Time::try_from(datetime!(2022-02-28 22:35:42.069 UTC)).unwrap();
        assert_eq!(&datetime.format(TimeZone::Utc), "2022-02-28 22:35:42.069Z");
    }

    #[test]
    fn test_formatting_many_digits_for_datetime() {
        let datetime = Time::try_from(datetime!(2022-02-28 22:35:42.069_042_7 UTC)).unwrap();
        assert_eq!(
            &datetime.format(TimeZone::Utc),
            "2022-02-28 22:35:42.069042Z"
        ); // format function is not rounding
    }

    /// Check that formatting today times doesn't display the date.
    /// WARNING: this test could easily flake with current implementation
    /// (checking day instead of hour-distance)
    #[test]
    fn test_formatting_today_omit_date() {
        let today = OffsetDateTime::now_utc().replace_time(time!(22:35:42));
        let datetime = Time::try_from(today).unwrap();
        assert_eq!(&datetime.format(TimeZone::Utc), "22:35:42Z");
    }

    #[test]
    fn test_custom_formatting() {
        let datetime = Time::try_from(datetime!(2022-02-28 22:35:42.069_042_7 UTC)).unwrap();
        assert_eq!(
            &datetime
                .format_time_custom("[year]-[month]-[day]", TimeZone::Utc)
                .unwrap(),
            "2022-02-28Z"
        );
        assert_eq!(
            &datetime
                .format_time_custom("[hour]", TimeZone::Utc)
                .unwrap(),
            "22Z"
        );
        assert_eq!(
            &datetime
                .format_time_custom("[hour]:[minute]", TimeZone::Utc)
                .unwrap(),
            "22:35Z"
        );
        assert_eq!(
            &datetime
                .format_time_custom("[hour]:[minute]:[second]", TimeZone::Utc)
                .unwrap(),
            "22:35:42Z"
        );
        assert_eq!(
            &datetime
                .format_time_custom(
                    "[hour]:[minute]:[second].[subsecond digits:3]",
                    TimeZone::Utc
                )
                .unwrap(),
            "22:35:42.069Z"
        );
        assert_eq!(
            &datetime
                .format_time_custom(
                    "[hour]:[minute]:[second].[subsecond digits:6]",
                    TimeZone::Utc
                )
                .unwrap(),
            "22:35:42.069042Z"
        );
    }

    #[test]
    fn test_parsing_time() {
        let all_time_zones = [TimeZone::Utc, TimeZone::Local, TimeZone::UnixEpoch];

        // Test relative time parsing with different units
        // Should be independent of the time zone setting.
        for time_zone in all_time_zones {
            assert_eq!(
                Time::parse("42s", time_zone),
                Some(Time::from_seconds_since_epoch(42.0))
            );
            assert_eq!(
                Time::parse("42.123s", time_zone),
                Some(Time::from_seconds_since_epoch(42.123))
            );
            assert_eq!(
                Time::parse("42ms", time_zone),
                Some(Time::from_seconds_since_epoch(0.042))
            );
            assert_eq!(
                Time::parse("42us", time_zone),
                Some(Time::from_seconds_since_epoch(0.000042))
            );
            assert_eq!(
                Time::parse("42ns", time_zone),
                Some(Time::from_seconds_since_epoch(0.000000042))
            );

            // Hour format.
            assert_eq!(
                Time::parse("22:35:42", time_zone),
                Some(Time::try_from(datetime!(1970-01-01 22:35:42 UTC)).unwrap())
            );

            // Hout format with fractional seconds.
            assert_eq!(
                Time::parse("22:35:42.069", time_zone),
                Some(Time::try_from(datetime!(1970-01-01 22:35:42.069 UTC)).unwrap())
            );
        }

        // Full dates.
        // Fun fact: 1954-04-11 is by some considered the least eventful day in history!
        // Full date and time
        assert_eq!(
            Time::parse("1954-04-11 22:35:42", TimeZone::Utc),
            Some(Time::try_from(datetime!(1954-04-11 22:35:42 UTC)).unwrap())
        );
        // Full date and time with milliseconds
        assert_eq!(
            Time::parse("1954-04-11 22:35:42.069", TimeZone::Utc),
            Some(Time::try_from(datetime!(1954-04-11 22:35:42.069 UTC)).unwrap())
        );
        // Timezone setting doesn't matter if UTC is enabled.
        for time_zone in all_time_zones {
            // Full date and time with Z suffix
            assert_eq!(
                Time::parse("1954-04-11 22:35:42Z", time_zone),
                Some(Time::try_from(datetime!(1954-04-11 22:35:42 UTC)).unwrap())
            );

            // Full date and time with milliseconds with Z suffix
            assert_eq!(
                Time::parse("1954-04-11 22:35:42.069Z", time_zone),
                Some(Time::try_from(datetime!(1954-04-11 22:35:42.069 UTC)).unwrap())
            );
        }

        // Current timezone.
        // Full date and time.
        if let Ok(local_offset) = UtcOffset::current_local_offset() {
            assert_eq!(
                Time::parse("1954-04-11 22:35:42", TimeZone::Local),
                Some(
                    Time::try_from(datetime!(1954-04-11 22:35:42).assume_offset(local_offset))
                        .unwrap()
                )
            );
            // Full date and time with milliseconds
            assert_eq!(
                Time::parse("1954-04-11 22:35:42.069", TimeZone::Local),
                Some(
                    Time::try_from(datetime!(1954-04-11 22:35:42.069).assume_offset(local_offset))
                        .unwrap()
                )
            );
        }

        // Test invalid formats
        assert_eq!(Time::parse("invalid", TimeZone::Utc), None);
        assert_eq!(Time::parse("2022-13-28", TimeZone::Utc), None); // Invalid month
        assert_eq!(Time::parse("2022-02-29", TimeZone::Utc), None); // Invalid day (not leap year)
        assert_eq!(Time::parse("25:00:00", TimeZone::Utc), None); // Invalid hour
        assert_eq!(Time::parse("00:60:00", TimeZone::Utc), None); // Invalid minute
        assert_eq!(Time::parse("00:00:60", TimeZone::Utc), None); // Invalid second
    }
}
