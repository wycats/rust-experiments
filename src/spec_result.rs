use derive_new::new;
use getset::Getters;
use laboratory_expectations::{traits::MatchResult, MatchError};

use std::{
    fmt::Display,
    time::{Duration, Instant},
};

use crate::{suite::DurationWithPrecision, DurationPrecision};

#[derive(Debug, Clone)]
pub struct SpecDesc {
    pub name: String,
    pub suite_name: String,
}

#[derive(Debug, Clone)]
pub struct ReporterSpecInfo {
    pub name: String,
    pub suite_name: String,
    pub number: usize,
}

impl ReporterSpecInfo {
    pub(crate) fn done(&self, status: SpecStatus, duration: DurationWithPrecision) -> SpecInfo {
        SpecInfo {
            name: self.name.clone(),
            suite_name: self.suite_name.clone(),
            number: self.number,
            status,
            duration,
        }
    }

    pub(crate) fn skipped(&self, precision: DurationPrecision) -> SpecInfo {
        let duration = Instant::now().elapsed();

        SpecInfo {
            name: self.name.clone(),
            suite_name: self.suite_name.clone(),
            number: self.number,
            status: SpecStatus::Skipped,
            duration: DurationWithPrecision::new(duration, precision),
        }
    }
}

#[derive(Debug, Clone, Getters)]
pub struct SpecInfo {
    #[get = "pub"]
    pub name: String,
    #[get = "pub"]
    pub suite_name: String,
    #[get = "pub"]
    pub number: usize,
    #[get = "pub"]
    pub status: SpecStatus,
    #[get = "pub"]
    pub duration: DurationWithPrecision,
}

#[derive(Debug, Clone, new)]
pub struct FinishedSpec {
    pub(crate) desc: SpecDesc,
    pub(crate) result: SpecStatus,
}

#[derive(Debug, Copy, Clone)]
pub struct SuccessResult {
    duration: Duration,
}

#[derive(Debug, Clone)]
pub struct ErrorResult {
    error: String,
    duration: Duration,
}

impl Display for ErrorResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

#[derive(Debug, Clone)]
pub enum SpecStatus {
    Success,
    Failure(MatchError),
    Skipped,
}

impl FinishedSpec {
    pub fn skipped(suite_name: impl Into<String>, name: impl Into<String>) -> FinishedSpec {
        FinishedSpec {
            desc: SpecDesc {
                name: name.into(),
                suite_name: suite_name.into(),
            },
            result: SpecStatus::Skipped,
        }
    }

    pub fn ran(
        suite_name: impl Into<String>,
        name: impl Into<String>,
        result: MatchResult,
    ) -> FinishedSpec {
        let (name, suite_name) = (name.into(), suite_name.into());

        FinishedSpec {
            desc: SpecDesc { name, suite_name },
            result: match result {
                Ok(_) => SpecStatus::Success,
                Err(err) => SpecStatus::Failure(err),
            },
        }
    }

    pub fn get_name(&self) -> &str {
        &self.desc.name
    }

    pub fn get_suite_name(&self) -> &str {
        &self.desc.suite_name
    }
}
