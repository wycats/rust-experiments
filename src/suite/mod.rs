pub mod described;
pub mod finalized;
pub mod mutable;
pub mod state;
pub mod traits;

use derive_new::new;
use std::{fmt::Display, ops::Deref, time::Duration};

pub use traits::Suite;

#[derive(Debug, Copy, Clone)]
pub enum DurationPrecision {
    Micros,
    Millis,
    Nanos,
    Seconds,
}

impl Display for DurationPrecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DurationPrecision::Micros => write!(f, "µs"),
            DurationPrecision::Millis => write!(f, "ms"),
            DurationPrecision::Nanos => write!(f, "ns"),
            DurationPrecision::Seconds => write!(f, "sec"),
        }
    }
}

#[derive(Debug, Copy, Clone, new)]
pub struct DurationWithPrecision {
    duration: Duration,
    precision: DurationPrecision,
}

impl Deref for DurationWithPrecision {
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        &self.duration
    }
}

impl Display for DurationWithPrecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.precision {
            DurationPrecision::Micros => {
                let amount = self.duration.as_micros();
                write!(f, "{}µs", amount)
            }
            DurationPrecision::Millis => {
                let amount = self.duration.as_millis();
                write!(f, "{}ms", amount)
            }
            DurationPrecision::Nanos => {
                let amount = self.duration.as_nanos();
                write!(f, "{}ns", amount)
            }
            DurationPrecision::Seconds => {
                let amount = self.duration.as_secs();
                write!(f, "{}sec", amount)
            }
        }
    }
}

pub struct FullSuiteInfo {
    pub duration: DurationWithPrecision,
}
