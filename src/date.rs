/// wasm32-unknown-unknown polyfill

#[cfg(all(feature = "js-sys", target_arch = "wasm32", target_os = "unknown"))]
pub use self::js_sys_date::OffsetDateTime;

#[cfg(not(feature = "js-sys"))]
#[cfg(any(all(target_arch = "wasm32", target_os = "unknown"), all(target_arch = "wasm32", target_os = "wasi")))]
pub use self::unix_epoch_stub_date::OffsetDateTime;

#[cfg(not(any(target_arch = "wasm32", target_os = "unknown")))]
pub use time::{OffsetDateTime, UtcOffset};

#[cfg(all(feature = "js-sys", target_arch = "wasm32", target_os = "unknown"))]
mod js_sys_date {
    use js_sys::Date;
    use time::Month;

    #[derive(Debug, Clone, PartialEq)]
    pub struct OffsetDateTime(Date);

    impl OffsetDateTime {
        pub fn from_unix_timestamp(_: i64) -> Option<Self> {
            Some(Self(Date::new(&(1000.0 * 60.0 * 24.0 * 5.0).into())))
        }

        #[inline(always)]
        pub fn now_utc() -> Self {
            let date = Date::new_0();
            OffsetDateTime(date)
        }

        #[inline(always)]
        pub fn now() -> Self {
            let date = Date::new_0();
            OffsetDateTime(date)
        }

        #[inline(always)]
        pub fn format(&self, format: impl ToString) -> String {
            // TODO
            "".into()
        }

        #[inline(always)]
        pub fn year(&self) -> i32 {
            self.0.get_full_year() as i32
        }

        #[inline(always)]
        pub fn month(&self) -> Month {
            match self.0.get_month() {
                0 => Month::January,
                1 => Month::February,
                2 => Month::March,
                3 => Month::April,
                4 => Month::May,
                5 => Month::June,
                6 => Month::July,
                7 => Month::August,
                8 => Month::September,
                9 => Month::October,
                10 => Month::November,
                11 => Month::December,
                _ => unreachable!(),
            }
        }

        #[inline(always)]
        pub fn day(&self) -> u8 {
            self.0.get_date() as u8
        }

        #[inline(always)]
        pub fn hour(&self) -> u8 {
            self.0.get_hours() as u8
        }

        #[inline(always)]
        pub fn minute(&self) -> u8 {
            self.0.get_minutes() as u8
        }

        #[inline(always)]
        pub fn second(&self) -> u8 {
            self.0.get_seconds() as u8
        }

        #[inline]
        pub fn offset(&self) -> super::UtcOffset {
            super::UtcOffset {
                hours: 0,
                minutes: 0,
                seconds: 0,
            }
        }
    }
}

#[cfg(not(feature = "js-sys"))]
#[cfg(any(all(target_arch = "wasm32", target_os = "unknown"), all(target_arch = "wasm32", target_os = "wasi")))]
mod unix_epoch_stub_date {
    use time::Month;

    #[derive(Debug, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash)]
    pub struct OffsetDateTime;
    impl OffsetDateTime {

        pub fn from_unix_timestamp(_: usize) -> Result<Self, String> {
            Ok(OffsetDateTime)
        }

        #[inline(always)]
        pub fn now_utc() -> Self {
            OffsetDateTime
        }

        #[inline(always)]
        pub fn now() -> Self {
            OffsetDateTime
        }

        #[inline(always)]
        pub fn format(&self, format: impl ToString) -> String {
            // TODO
            "".into()
        }

        #[inline(always)]
        pub fn year(&self) -> i32 {
            1970
        }

        #[inline(always)]
        pub fn month(&self) -> Month {
            Month::January
        }

        #[inline(always)]
        pub fn day(&self) -> u8 {
            1
        }

        #[inline(always)]
        pub fn hour(&self) -> u8 {
            0
        }

        #[inline(always)]
        pub fn minute(&self) -> u8 {
            0
        }

        #[inline(always)]
        pub fn second(&self) -> u8 {
            0
        }

        #[inline]
        pub fn offset(&self) -> super::UtcOffset {
            super::UtcOffset {
                hours: 0,
                minutes: 0,
                seconds: 0,
            }
        }
    }
}

#[cfg(any(all(target_arch = "wasm32", target_os = "unknown"), all(target_arch = "wasm32", target_os = "wasi")))]
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UtcOffset {
    hours: i8,
    minutes: i8,
    seconds: i8,
}

#[cfg(any(all(target_arch = "wasm32", target_os = "unknown"), all(target_arch = "wasm32", target_os = "wasi")))]
impl UtcOffset {
    pub const fn whole_hours(self) -> i8 {
        self.hours
    }

    pub const fn is_negative(self) -> bool {
        self.hours < 0 || self.minutes < 0 || self.seconds < 0
    }

    pub const fn minutes_past_hour(self) -> i8 {
        self.minutes
    }
}
