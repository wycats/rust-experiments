use getset::Getters;
use std::{fmt::Debug, path::PathBuf};

use crate::{
    reporter::StartedReporter, spec::It, test::RunnableSuite, DurationPrecision, ReportResult,
    Reporter, ReporterOutput, SuiteOutcome,
};

use super::{
    finalized::FinalizedSuite, mutable::SuiteWithMutableState, state::SuiteWithState,
    traits::SuiteExt,
};

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct SuiteDetails {
    name: String,
    pub(crate) nesting: usize,
    pub(crate) skip: bool,
    pub(crate) export: Option<PathBuf>,
    pub(crate) precision: DurationPrecision,
}

impl SuiteDetails {
    pub(crate) fn suite_name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug)]
pub struct DescribedSuite {
    details: SuiteDetails,
    reporter: Option<Box<dyn StartedReporter>>,
    nested: Vec<FinalizedSuite>,
}

impl RunnableSuite for DescribedSuite {
    type State = ();

    fn run_with(self, output: ReporterOutput) -> ReportResult<SuiteOutcome> {
        self.state(()).run_with(output)
    }
}

impl SuiteExt for DescribedSuite {
    fn details_mut(&mut self) -> &mut SuiteDetails {
        &mut self.details
    }
}

impl Into<FinalizedSuite> for DescribedSuite {
    fn into(self) -> FinalizedSuite {
        self.state(()).into()
    }
}

impl DescribedSuite {
    pub fn new(name: impl Into<String>) -> Self {
        let suite_name = name.into();

        Self {
            details: SuiteDetails {
                nesting: 0,
                name: suite_name,
                export: None,
                precision: DurationPrecision::Millis,
                skip: false,
            },
            reporter: None,
            nested: vec![],
        }
    }

    pub fn reporter(mut self, reporter: impl Reporter + 'static) -> Self {
        self.reporter = Some(Box::new(reporter.start()));
        self
    }

    pub fn state<T>(self, state: T) -> SuiteWithState<T>
    where
        T: Clone + Debug + 'static,
    {
        let mut suite = SuiteWithState::new(self.details, self.reporter, state);

        for nested in self.nested {
            suite = suite.suite(nested);
        }

        suite
    }

    pub fn mutable_state<T>(self, state: T) -> SuiteWithMutableState<T>
    where
        T: Debug + 'static,
    {
        let mut suite = SuiteWithMutableState::top(self.details, self.reporter, state);

        for nested in self.nested {
            suite = suite.suite(nested);
        }

        suite
    }

    pub fn suite(mut self, suite: impl Into<FinalizedSuite>) -> Self {
        self.nested.push(suite.into());
        self
    }

    pub fn suites(mut self, suites: Vec<impl Into<FinalizedSuite>>) -> Self {
        for suite in suites.into_iter() {
            self.nested.push(suite.into());
        }

        self
    }

    pub fn specs(self, callback: impl FnOnce(&mut It)) -> SuiteWithState<()> {
        let mut suite = self.state(());

        let mut it = It::new();

        callback(&mut it);

        for spec in it.specs() {
            suite = suite.spec(spec);
        }

        suite
    }
}
