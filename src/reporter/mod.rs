#[macro_use]
pub mod output;

pub mod json;
pub mod spec;

use std::fmt::Debug;

use crate::{
    spec_result::{ReporterSpecInfo, SpecInfo},
    suite::FullSuiteInfo,
    suite_result::SuiteFinished,
    SuiteDetails,
};

use self::output::SuiteOutput;

pub trait ToResult {
    fn to_result(&self) -> Result<(), String>;
}

pub trait Reporter: Clone {
    type Started: StartedReporter;

    fn start(&self) -> Self::Started;
}

#[derive(Debug)]
pub enum ReportError {
    Io(std::io::Error),
    Fmt(std::fmt::Error),
    Error(Box<dyn std::error::Error>),
}

impl<T> From<T> for ReportError
where
    T: std::error::Error + 'static,
{
    fn from(err: T) -> Self {
        ReportError::Error(Box::new(err))
    }
}

pub type ReportResult<T = ()> = Result<T, ReportError>;

pub trait StartedReporter: Debug {
    fn start_suite(&mut self, _out: &mut SuiteOutput, _desc: &SuiteDetails) -> ReportResult {
        Ok(())
    }

    fn end_suite(&mut self, _out: &mut SuiteOutput, _result: &SuiteFinished) -> ReportResult {
        Ok(())
    }

    fn start_test(&mut self, _out: &mut SuiteOutput, _desc: &ReporterSpecInfo) -> ReportResult {
        Ok(())
    }

    fn end_test(&mut self, _out: &mut SuiteOutput, _info: SpecInfo) -> ReportResult {
        Ok(())
    }

    fn finish(&mut self, _out: &mut SuiteOutput, _info: FullSuiteInfo) -> ReportResult {
        Ok(())
    }
}
