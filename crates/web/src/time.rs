//! Time and calendar days.

/// Milliseconds since the Unix epoch. Returns 0 on the server, where timing
/// is never needed (challenges only run in the browser).
pub fn now_ms() -> f64 {
    #[cfg(feature = "hydrate")]
    {
        js_sys::Date::now()
    }
    #[cfg(not(feature = "hydrate"))]
    {
        0.0
    }
}

/// A calendar day, stored as days since 1970-01-01. Plain integer math, so
/// "yesterday" and "52 weeks ago" are just subtraction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Day(pub i64);

impl Day {
    /// Howard Hinnant's `days_from_civil` algorithm.
    #[cfg_attr(not(any(feature = "hydrate", test)), allow(dead_code))]
    pub const fn from_ymd(y: i64, m: u32, d: u32) -> Day {
        let y = if m <= 2 { y - 1 } else { y };
        let era = if y >= 0 { y } else { y - 399 } / 400;
        let yoe = y - era * 400;
        let mp = (m as i64 + 9) % 12;
        let doy = (153 * mp + 2) / 5 + d as i64 - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        Day(era * 146_097 + doe - 719_468)
    }

    /// Inverse of `from_ymd`.
    pub fn ymd(self) -> (i64, u32, u32) {
        let z = self.0 + 719_468;
        let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
        let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
        (yoe + era * 400 + i64::from(m <= 2), m, d)
    }

    pub fn plus(self, days: i64) -> Day {
        Day(self.0 + days)
    }

    /// 0 = Monday … 6 = Sunday. (1970-01-01 was a Thursday.)
    pub fn weekday(self) -> u32 {
        (self.0 + 3).rem_euclid(7) as u32
    }

    /// Key used in saved progress, e.g. "2026-11-13".
    pub fn key(self) -> String {
        let (y, m, d) = self.ymd();
        format!("{y:04}-{m:02}-{d:02}")
    }
}

/// Today in the learner's local time zone (browser). On the server this is
/// never shown; it returns the Unix epoch.
pub fn today() -> Day {
    #[cfg(feature = "hydrate")]
    {
        let now = js_sys::Date::new_0();
        Day::from_ymd(now.get_full_year() as i64, now.get_month() + 1, now.get_date())
    }
    #[cfg(not(feature = "hydrate"))]
    {
        Day(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn days_round_trip() {
        for (y, m, d) in [(1970, 1, 1), (2000, 2, 29), (2026, 11, 13), (2026, 12, 31), (2028, 3, 1)] {
            assert_eq!(Day::from_ymd(y, m, d).ymd(), (y, m, d));
        }
        assert_eq!(Day::from_ymd(1970, 1, 1), Day(0));
        assert_eq!(Day::from_ymd(2026, 11, 13).key(), "2026-11-13");
        assert_eq!(Day::from_ymd(2026, 3, 1).plus(-1).key(), "2026-02-28");
    }

    #[test]
    fn weekdays() {
        assert_eq!(Day::from_ymd(1970, 1, 1).weekday(), 3); // Thursday
        assert_eq!(Day::from_ymd(2026, 11, 13).weekday(), 4); // Friday
    }
}
