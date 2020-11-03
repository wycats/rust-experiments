use std::time::Instant;

use crate::{
    reporter::StartedReporter, spec_result::ReporterSpecInfo, spec_result::SpecStatus,
    suite_result::BuildSuiteResult, ReportResult, ReporterOutput, Spec, SuiteOutcome,
};

use super::{
    described::SuiteDetails, traits::RunnableSuite, traits::SuiteExt, DurationWithPrecision,
    FullSuiteInfo,
};

#[derive(Debug)]
pub struct FinalizedTopSuite {
    suite: FinalizedSuite,
    reporter: Box<dyn StartedReporter>,
}

impl RunnableSuite for FinalizedTopSuite {
    type State = ();

    fn run_with(self, mut output: ReporterOutput) -> ReportResult<SuiteOutcome> {
        let Self {
            suite,
            mut reporter,
        } = self;

        suite.run_with_reporter(&mut output, &mut *reporter, 0)
    }
}

#[derive(Debug)]
pub struct FinalizedSuite {
    details: SuiteDetails,
    specs: Vec<Spec>,
    nested: Vec<FinalizedSuite>,
}

impl SuiteExt for FinalizedSuite {
    fn details_mut(&mut self) -> &mut SuiteDetails {
        &mut self.details
    }
}

impl FinalizedSuite {
    pub(crate) fn new(
        details: SuiteDetails,
        specs: Vec<Spec>,
        nested: Vec<FinalizedSuite>,
    ) -> FinalizedSuite {
        Self {
            details,
            specs,
            nested,
        }
    }

    fn nest(mut self, nesting: usize) -> Self {
        self.details.nesting = nesting;

        self.nested = self
            .nested
            .into_iter()
            .map(|suite| suite.nest(nesting + 1))
            .collect();

        self
    }

    pub(crate) fn top(self, reporter: Box<dyn StartedReporter>) -> FinalizedTopSuite {
        FinalizedTopSuite {
            suite: self.nest(0),
            reporter,
        }
    }

    pub fn spec(mut self, spec: Spec) -> Self {
        self.specs.push(spec);
        self
    }

    pub fn run_with_reporter(
        self,
        output: &mut ReporterOutput,
        reporter: &mut dyn StartedReporter,
        depth: usize,
    ) -> ReportResult<SuiteOutcome> {
        let Self {
            details,
            specs,
            nested,
            ..
        } = self;

        let mut suite_output = output.for_suite(specs.len());

        let suite_name = details.suite_name();
        let precision = details.precision;

        reporter.start_suite(&mut suite_output, &details)?;

        let mut results = BuildSuiteResult::new(suite_name);

        let suite_start = Instant::now();

        for (i, spec) in specs.into_iter().enumerate() {
            let info = ReporterSpecInfo {
                suite_name: suite_name.to_string(),
                name: spec.name.clone(),
                number: i,
            };

            if details.skip {
                reporter.end_test(&mut suite_output, info.skipped(precision))?;
            } else {
                let start = Instant::now();
                // reporter.start_test(&mut suite_output, &info)?;

                let finished = spec.run(suite_name);

                let duration = DurationWithPrecision::new(start.elapsed(), precision);
                // let done_info = info.done(DurationWithPrecision::new(start.elapsed(), precision));

                match &finished.result {
                    SpecStatus::Success => {
                        reporter.end_test(
                            &mut suite_output,
                            info.done(SpecStatus::Success, duration),
                        )?;
                    }
                    SpecStatus::Failure(error) => {
                        reporter.end_test(
                            &mut suite_output,
                            info.done(SpecStatus::Failure(error.clone()), duration),
                        )?;
                    }
                    SpecStatus::Skipped => {
                        reporter.end_test(
                            &mut suite_output,
                            info.done(SpecStatus::Skipped, duration),
                        )?;
                    }
                }

                results.add_finished(finished);
            }
        }

        let mut output = suite_output.child();

        for suite in nested {
            println!("{:#?}", suite);
            let mut output = output.for_nested();
            suite.run_with_reporter(&mut output, reporter, depth + 1)?;
        }

        drop(output);

        let finished = results.finish(DurationWithPrecision::new(suite_start.elapsed(), precision));
        reporter.end_suite(&mut suite_output, &finished)?;

        if depth == 0 {
            reporter.finish(
                &mut suite_output,
                FullSuiteInfo {
                    duration: DurationWithPrecision::new(suite_start.elapsed(), precision),
                },
            )?;
        }

        Ok(SuiteOutcome::Finished(finished))
    }
}
