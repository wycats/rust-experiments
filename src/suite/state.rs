use std::fmt::Debug;

use crate::{
    reporter::StartedReporter, spec::TypedIt, spec::TypedSpec, test::RunnableSuite, ReportResult,
    Reporter, ReporterOutput, Spec, SpecReporter, SuiteExt, SuiteOutcome,
};

use super::{
    described::{DescribedSuite, SuiteDetails},
    finalized::FinalizedSuite,
};

impl<T> RunnableSuite for SuiteWithState<T>
where
    T: Clone + Debug + 'static,
{
    type State = T;

    fn run_with(self, output: ReporterOutput) -> ReportResult<SuiteOutcome<T>> {
        let state = self.state.clone();
        let reporter = self
            .reporter
            .unwrap_or_else(|| Box::new(SpecReporter.start()));
        let finalized = FinalizedSuite::new(self.details, self.specs, self.nested);
        Ok(finalized.top(reporter).run_with(output)?.with_state(state))
    }
}

pub struct SuiteWithState<T>
where
    T: Clone + 'static,
{
    details: SuiteDetails,
    reporter: Option<Box<dyn StartedReporter>>,
    specs: Vec<Spec>,
    state: T,
    nested: Vec<FinalizedSuite>,
}

impl<T> Debug for SuiteWithState<T>
where
    T: Clone + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SuiteWithState")
            .field("details", &self.details)
            .field("specs", &self.specs)
            .field("nested", &self.nested)
            .finish()
    }
}

impl<T> SuiteExt for SuiteWithState<T>
where
    T: Clone + 'static,
{
    fn details_mut(&mut self) -> &mut SuiteDetails {
        &mut self.details
    }
}

impl<T> Into<FinalizedSuite> for SuiteWithState<T>
where
    T: Clone + 'static,
{
    fn into(self) -> FinalizedSuite {
        let has_only = self.specs.iter().any(|s| s.is_only());
        let Self {
            details,
            mut specs,
            nested,
            ..
        } = self;

        if has_only {
            specs = specs.into_iter().map(|s| s.in_only_suite()).collect();
        }

        FinalizedSuite::new(details, specs, nested)
    }
}

impl<T> SuiteWithState<T>
where
    T: Clone + Debug + 'static,
{
    pub(crate) fn new(
        details: SuiteDetails,
        reporter: Option<Box<dyn StartedReporter>>,
        state: T,
    ) -> SuiteWithState<T> {
        SuiteWithState {
            details,
            reporter,
            specs: vec![],
            state,
            nested: vec![],
        }
    }

    pub fn spec(mut self, spec: TypedSpec<T>) -> SuiteWithState<T> {
        let state = self.state.clone();

        self.specs.push(spec.with_state(move || state.clone()));
        self
    }

    pub fn describe(
        mut self,
        name: impl Into<String>,
        callback: impl FnOnce(&mut TypedIt<T>),
    ) -> Self {
        let mut suite = DescribedSuite::new(name).state(self.state.clone());

        let mut it = TypedIt::new();
        callback(&mut it);

        for spec in it.specs() {
            suite = suite.spec(spec);
        }

        self.nested.push(suite.into());
        self
    }

    pub fn specs(self, callback: impl FnOnce(&mut TypedIt<T>)) -> Self {
        let mut it = TypedIt::<T>::new();

        callback(&mut it);

        self
    }

    pub fn suite(mut self, suite: impl Into<FinalizedSuite>) -> Self {
        self.nested.push(suite.into());
        self
    }

    pub fn nest(mut self, suite: DescribedSuite) -> Self {
        // for suite in suites.drain(..) {
        self.nested.push(suite.state(self.state.clone()).into());
        // }

        self
    }

    pub fn suites(mut self, mut suites: Vec<impl Into<FinalizedSuite>>) -> Self {
        for suite in suites.drain(..) {
            self.nested.push(suite.into());
        }

        self
    }

    // pub fn run(self) -> ReportResult<SuiteOutcome> {
    //     self.to_suite().run()
    // }

    // pub fn run_with(self, mut output: ReporterOutput) -> ReportResult<SuiteOutcome> {
    //     self.to_suite().run_with(output.child())
    // }
}
