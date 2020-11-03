use std::fmt::{Display, Write};

use laboratory_expectations::{block, MatchError};

use crate::{
    spec_result::{SpecInfo, SpecStatus},
    suite::{described::SuiteDetails, FullSuiteInfo},
};

use super::{output::SuiteOutput, ReportResult, Reporter, StartedReporter, ToResult};

#[derive(Debug, Clone)]
pub struct SpecReporter;

#[derive(Debug, Clone)]
pub struct MinimalReporter;

#[derive(Debug, Clone)]
pub struct StartedSpecReporter {
    full: bool,
    passed: Vec<SpecInfo>,
    failed: Vec<(SpecInfo, MatchError)>,
    skipped: Vec<SpecInfo>,
}

pub struct FinishedSpecReporter {}

impl Display for FinishedSpecReporter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FinishedSpecReporter")
    }
}

impl ToResult for FinishedSpecReporter {
    fn to_result(&self) -> Result<(), String> {
        todo!()
    }
}

impl Reporter for MinimalReporter {
    type Started = StartedSpecReporter;

    fn start(&self) -> Self::Started {
        StartedSpecReporter {
            full: false,
            passed: vec![],
            failed: vec![],
            skipped: vec![],
        }
    }
}

impl Reporter for SpecReporter {
    type Started = StartedSpecReporter;

    fn start(&self) -> Self::Started {
        StartedSpecReporter {
            full: true,
            passed: vec![],
            failed: vec![],
            skipped: vec![],
        }
    }
}

impl StartedReporter for StartedSpecReporter {
    fn start_suite(&mut self, out: &mut SuiteOutput, desc: &SuiteDetails) -> ReportResult {
        if self.full {
            out.nest(desc.nesting);
            outln!(out, block! { (desc.name()) });
        }

        Ok(())
    }

    fn end_test(&mut self, out: &mut SuiteOutput, desc: SpecInfo) -> ReportResult {
        match desc.clone() {
            SpecInfo {
                name,
                duration,
                status: SpecStatus::Success,
                ..
            } => {
                if self.full {
                    outln!(+1 => out, block! { " ✓  should " name " (" duration ")" });
                }

                self.passed.push(desc);
            }
            SpecInfo {
                name,
                number,
                duration,
                status: SpecStatus::Failure(err),
                ..
            } => {
                if self.full {
                    outln!(+1 => out, block! { (out.enumerate(number)) ") should " name "(" duration ")" })
                }

                self.failed.push((desc, err));
            }
            SpecInfo {
                name,
                status: SpecStatus::Skipped,
                ..
            } => {
                if self.full {
                    outln!(+1 => out, block! { "    should " name });
                }

                self.skipped.push(desc);
            }
        }

        Ok(())
    }

    fn finish(&mut self, out: &mut SuiteOutput, info: FullSuiteInfo) -> ReportResult {
        if self.full {
            out!(out, "\n\n");
        }

        out.nest(0);

        let fail_count = self.failed.len();
        let pass_count = self.passed.len();

        if fail_count == 0 {
            outln!(
                out,
                block! { "✓ " pass_count " tests completed (" (info.duration) ")" }
            );
        } else {
            outln!(
                out,
                block! { "✖ {} of {} tests failed:" (fail_count) (fail_count + pass_count) }
            );

            outln!(out);

            for (info, err) in &self.failed {
                outln!(
                    out,
                    block! {
                        (out.enumerate(info.number))
                        ") " (info.suite_name()) " should " (info.name()) ": " (err.as_terse())
                    }
                );
            }
        }

        Ok(())
    }
}
