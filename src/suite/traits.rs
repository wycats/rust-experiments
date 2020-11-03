use std::{fmt::Debug, io::BufWriter, path::PathBuf};

use crate::{DurationPrecision, ReportResult, ReporterOutput, SuiteOutcome};

use super::described::SuiteDetails;

// pub trait FinalizedSuiteTrait: Sized {}

pub trait RunnableSuite: Debug + Sized {
    type State;

    fn run(self) -> ReportResult<SuiteOutcome<Self::State>> {
        self.run_with(ReporterOutput::stdout())
    }

    fn run_with(self, output: ReporterOutput) -> ReportResult<SuiteOutcome<Self::State>>;

    fn to_string(self) -> ReportResult<String> {
        let buffer = vec![];
        let mut writer = BufWriter::new(buffer);
        let output = ReporterOutput::buffer(&mut writer);

        self.run_with(output)?;

        let buffer = writer.into_inner().unwrap();

        Ok(String::from_utf8_lossy(&buffer).to_string())
    }
}

pub trait Suite<T>: Sized + Debug
where
    T: Clone + 'static,
{
}

pub trait StateSuite<T>: Suite<T>
where
    T: Clone + 'static,
{
    fn get_state(&self) -> T;
}

pub trait SuiteExt: Sized + Debug {
    fn details_mut(&mut self) -> &mut SuiteDetails;

    fn skip(mut self) -> Self {
        self.details_mut().skip = true;
        self
    }

    fn export_to(mut self, path: impl Into<PathBuf>) -> Self {
        self.details_mut().export = Some(path.into());
        self
    }

    fn precision(mut self, precision: impl Into<DurationPrecision>) -> Self {
        self.details_mut().precision = precision.into();
        self
    }

    fn in_milliseconds(self) -> Self {
        self.precision(DurationPrecision::Millis)
    }

    fn in_microseconds(self) -> Self {
        self.precision(DurationPrecision::Micros)
    }

    fn in_nanoseconds(self) -> Self {
        self.precision(DurationPrecision::Nanos)
    }

    fn in_seconds(self) -> Self {
        self.precision(DurationPrecision::Seconds)
    }
}
