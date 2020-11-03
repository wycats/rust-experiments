use std::{
    cell::RefCell,
    cell::RefMut,
    fmt::Debug,
    rc::{Rc, Weak},
};

use crate::{
    reporter::StartedReporter, spec::TypedIt, spec::TypedMutableIt, spec::TypedSpec,
    test::RunnableSuite, ReportResult, Reporter, ReporterOutput, Spec, SpecReporter, SuiteExt,
    SuiteOutcome,
};

use super::{
    described::{DescribedSuite, SuiteDetails},
    finalized::FinalizedSuite,
};

#[derive(Debug)]
pub struct WeakRef<T> {
    cell: Weak<RefCell<T>>,
}

impl<T> Clone for WeakRef<T> {
    fn clone(&self) -> Self {
        WeakRef {
            cell: self.cell.clone(),
        }
    }
}

impl<T> WeakRef<T> {
    pub fn mut_ref<U>(&self, callback: impl Fn(&mut T) -> U) -> U {
        let cell: Rc<RefCell<T>> = self.cell.upgrade().unwrap();
        let mut value: RefMut<_> = cell.borrow_mut();

        callback(&mut *value)
    }
}

#[derive(Debug)]
pub struct UniqueStrongRef<T> {
    cell: Rc<RefCell<T>>,
}

impl<T> UniqueStrongRef<T> {
    pub(crate) fn new(value: T) -> UniqueStrongRef<T> {
        UniqueStrongRef {
            cell: Rc::new(RefCell::new(value)),
        }
    }

    pub fn unwrap(self) -> T {
        let inner = match Rc::try_unwrap(self.cell) {
            Ok(inner) => inner,
            Err(_) => unreachable!(),
        };

        inner.into_inner()
    }

    pub(crate) fn weak(&self) -> WeakRef<T> {
        WeakRef {
            cell: Rc::downgrade(&self.cell),
        }
    }
}

pub struct SuiteWithMutableState<T>
where
    T: Debug + 'static,
{
    details: SuiteDetails,
    reporter: Option<Box<dyn StartedReporter>>,
    specs: Vec<Spec>,
    state: UniqueStrongRef<T>,
    nested: Vec<FinalizedSuite>,
}

impl<T> Debug for SuiteWithMutableState<T>
where
    T: Debug + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SuiteWithMutableState")
            .field("details", &self.details)
            .field("specs", &self.specs)
            .field("nested", &self.nested)
            .finish()
    }
}

impl<T> RunnableSuite for SuiteWithMutableState<T>
where
    T: Debug + 'static,
{
    type State = T;

    fn run_with(self, output: ReporterOutput) -> ReportResult<SuiteOutcome<Self::State>> {
        // let state = RefCount::weak(&self.state);
        let reporter = self
            .reporter
            .unwrap_or_else(|| Box::new(SpecReporter.start()));
        let finalized = FinalizedSuite::new(self.details, self.specs, self.nested);
        let state = self.state;

        let run = finalized.top(reporter).run_with(output)?;
        // println!("FINALIZE");
        let state = state.unwrap();

        Ok(run.with_state(state))
    }
}

impl<T> SuiteExt for SuiteWithMutableState<T>
where
    T: Debug + 'static,
{
    fn details_mut(&mut self) -> &mut SuiteDetails {
        &mut self.details
    }
}

impl<T> Into<FinalizedSuite> for SuiteWithMutableState<T>
where
    T: Debug + 'static,
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

impl<T> SuiteWithMutableState<T>
where
    T: Debug + 'static,
{
    pub(crate) fn top(
        details: SuiteDetails,
        reporter: Option<Box<dyn StartedReporter>>,
        state: T,
    ) -> SuiteWithMutableState<T> {
        SuiteWithMutableState {
            details,
            reporter,
            specs: vec![],
            state: UniqueStrongRef::new(state),
            nested: vec![],
        }
    }

    pub fn spec(mut self, spec: TypedSpec<WeakRef<T>>) -> SuiteWithMutableState<T> {
        let state = self.state.weak();

        self.specs.push(spec.with_state(move || state.clone()));
        self
    }

    pub fn describe(
        mut self,
        name: impl Into<String>,
        callback: impl FnOnce(&mut TypedMutableIt<T>),
    ) -> Self {
        let mut suite = DescribedSuite::new(name).state(self.state.weak());

        let mut it = TypedMutableIt::new();
        callback(&mut it);

        for spec in it.specs() {
            suite = suite.spec(spec);
        }

        self.nested.push(suite.into());
        self
    }

    pub fn specs(self, callback: impl FnOnce(&mut TypedIt<Rc<RefCell<T>>>)) -> Self {
        let mut it = TypedIt::<Rc<RefCell<T>>>::new();

        callback(&mut it);

        self
    }

    pub fn suite(mut self, suite: impl Into<FinalizedSuite>) -> Self {
        self.nested.push(suite.into());
        self
    }

    pub fn nest(mut self, suite: DescribedSuite) -> Self {
        // for suite in suites.drain(..) {
        self.nested.push(suite.state(self.state.weak()).into());
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
