use crate::{spec_result::FinishedSpec, suite::DurationWithPrecision};

pub struct BuildSuiteResult {
    name: String,
    tests: Vec<FinishedSpec>,
}

impl BuildSuiteResult {
    pub fn new(name: impl Into<String>) -> BuildSuiteResult {
        BuildSuiteResult {
            name: name.into(),
            tests: vec![],
        }
    }

    pub fn add_finished(&mut self, result: FinishedSpec) {
        self.tests.push(result)
    }

    pub fn finish(self, duration: DurationWithPrecision) -> SuiteFinished {
        SuiteFinished {
            name: self.name,
            tests: self.tests,
            duration,
            state: (),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SuiteOutcome<State = ()> {
    Finished(SuiteFinished<State>),
    Skipped,
}

impl<State> SuiteOutcome<State> {
    pub fn with_state<S2>(self, state: S2) -> SuiteOutcome<S2> {
        match self {
            SuiteOutcome::Finished(finished) => SuiteOutcome::Finished(finished.with_state(state)),
            SuiteOutcome::Skipped => SuiteOutcome::Skipped,
        }
    }

    pub fn into_state(self) -> State {
        match self {
            SuiteOutcome::Finished(finished) => finished.state,
            SuiteOutcome::Skipped => panic!("A skipped suite does not have state"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SuiteFinished<State = ()> {
    name: String,
    tests: Vec<FinishedSpec>,
    duration: DurationWithPrecision,
    state: State,
}

impl<State> SuiteFinished<State> {
    fn with_state<S2>(self, state: S2) -> SuiteFinished<S2> {
        SuiteFinished {
            name: self.name,
            tests: self.tests,
            duration: self.duration,
            state,
        }
    }

    pub fn tests(&self) -> impl Iterator<Item = &FinishedSpec> {
        self.tests.iter()
    }

    pub fn duration(&self) -> DurationWithPrecision {
        self.duration
    }

    // pub fn report<R: StartedReporter>(
    //     &mut self,
    //     mut reporter: R,
    //     mut output: ReporterOutput,
    // ) -> ReportResult {
    //     // let mut output = ReporterOutput::stdout();

    //     reporter.start_suite(&mut output)?;

    //     for (i, test) in self.tests.drain(..).enumerate() {
    //         let info = test.desc.info(i);
    //         reporter.start_test(&mut output, &info)?;

    //         match test {
    //             FinishedSpec {
    //                 result: SpecResult::Success(success),
    //                 ..
    //             } => {
    //                 reporter.passed(&mut output, info, success)?;
    //             }
    //             FinishedSpec {
    //                 result: SpecResult::Error(err),
    //                 ..
    //             } => {
    //                 reporter.failed(&mut output, info, err)?;
    //             }
    //             FinishedSpec {
    //                 result: SpecResult::Skipped,
    //                 ..
    //             } => {
    //                 reporter.skipped(&mut output, info)?;
    //             }
    //         }
    //     }

    //     reporter.end_suite(&mut output)?;

    //     reporter.finish(&mut output)
    // }
}

// impl SuiteResult {
//     pub fn new(name: impl Into<String>) -> SuiteResult {
//         SuiteResult {
//             name: name.into(),
//             passing: 0,
//             failing: 0,
//             ignored: 0,
//             child_suites: vec![],
//             child_tests: vec![],
//             duration: Duration::new(0, 0),
//         }
//     }
//     // pub fn add_spec_result (&mut self, spec: SpecResult) {
//     //     self.child_tests.push(spec);
//     // }
//     pub fn updated_from_suite(&mut self, child_result_option: Option<SuiteResult>) {
//         if let Some(child_result) = child_result_option {
//             self.passing += child_result.get_passing();
//             self.failing += child_result.get_failing();
//             self.ignored += child_result.get_ignored();
//             self.duration += child_result.get_duration();
//             self.child_suites.push(child_result);
//         }
//     }
//     pub fn update_from_spec(&mut self, spec: SpecResult) {
//         self.passing += spec.update_passing();
//         self.failing += spec.update_failing();
//         self.ignored += spec.update_ignored();
//         self.duration += *spec.get_duration();
//         self.child_tests.push(spec);
//     }
//     pub fn get_passing(&self) -> u64 {
//         self.passing
//     }
//     pub fn get_failing(&self) -> u64 {
//         self.failing
//     }
//     pub fn get_ignored(&self) -> u64 {
//         self.ignored
//     }
//     pub fn get_child_specs(&self) -> Vec<SpecResult> {
//         self.child_tests.clone()
//     }
//     pub fn get_child_suites(&self) -> Vec<SuiteResult> {
//         self.child_suites.clone()
//     }
//     pub fn get_name(&self) -> &str {
//         &self.name
//     }
//     pub fn get_duration(&self) -> Duration {
//         self.duration
//     }
//     pub fn set_duration(&mut self, duration: Duration) {
//         self.duration = duration
//     }
// }
// impl Clone for SuiteResult {
//     fn clone(&self) -> SuiteResult {
//         SuiteResult {
//             name: self.name.clone(),
//             passing: self.passing,
//             failing: self.failing,
//             ignored: self.ignored,
//             child_suites: self.child_suites.clone(),
//             child_tests: self.child_tests.clone(),
//             duration: self.duration,
//         }
//     }
// }
