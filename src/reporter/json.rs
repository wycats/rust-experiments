use derive_new::new;
use serde::{Deserialize, Serialize};
use std::{fmt::Write, time::Duration};

use crate::{
    spec_result::{SpecInfo, SpecStatus},
    suite::described::SuiteDetails,
    suite::DurationWithPrecision,
    suite_result::SuiteFinished,
};

use super::{output::SuiteOutput, ReportResult, Reporter, StartedReporter};

#[derive(Debug, Serialize, Deserialize)]
struct JsonOutput {
    name: String,
    passing: usize,
    failing: usize,
    ignored: usize,
    child_suites: Vec<()>,
    child_tests: Vec<JsonTest>,
    duration: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct JsonTest {
    name: String,
    full_name: String,
    pass: bool,
    error_message: Option<String>,
    duration: Duration,
}

impl JsonTest {
    fn from_test(info: SpecInfo) -> JsonTest {
        JsonTest {
            name: info.name.clone(),
            full_name: format!("{} {}", info.suite_name, info.name),
            pass: match info.status {
                SpecStatus::Success => true,
                SpecStatus::Failure(_) => false,
                SpecStatus::Skipped => false,
            },
            error_message: match info.status {
                SpecStatus::Success => None,
                SpecStatus::Failure(err) => Some(format!("{:?}", err)),
                SpecStatus::Skipped => None,
            },
            duration: *info.duration,
        }
    }
}

#[derive(Debug, Clone, new)]
pub struct JsonReporter {
    #[new(value = "false")]
    pretty: bool,
}

impl JsonReporter {
    pub fn pretty() -> JsonReporter {
        JsonReporter { pretty: true }
    }
}

#[derive(Debug, Clone)]
pub struct StartedJsonReporter {
    pretty: bool,
    suite_name: Option<String>,
    tests: Vec<SpecInfo>,
}

impl StartedJsonReporter {
    fn into_json(self, duration: DurationWithPrecision) -> JsonOutput {
        let StartedJsonReporter { tests, .. } = self;

        let mut passing = 0;
        let mut failing = 0;
        let mut ignored = 0;
        let mut json_tests: Vec<JsonTest> = vec![];

        for test in tests.into_iter() {
            match test.status {
                SpecStatus::Success => passing += 1,
                SpecStatus::Failure(_) => failing += 1,
                SpecStatus::Skipped => ignored += 1,
            }

            json_tests.push(JsonTest::from_test(test));
        }

        JsonOutput {
            name: self.suite_name.unwrap(),
            passing,
            failing,
            ignored,
            child_suites: vec![],
            child_tests: json_tests,
            duration: *duration,
        }
    }
}

impl Reporter for JsonReporter {
    type Started = StartedJsonReporter;

    fn start(&self) -> Self::Started {
        StartedJsonReporter {
            pretty: self.pretty,
            suite_name: None,
            tests: vec![],
        }
    }
}

impl StartedReporter for StartedJsonReporter {
    fn start_suite(&mut self, _out: &mut SuiteOutput, desc: &SuiteDetails) -> ReportResult {
        self.suite_name = Some(desc.name().clone());
        Ok(())
    }

    fn end_suite(&mut self, out: &mut SuiteOutput, result: &SuiteFinished) -> ReportResult {
        let pretty = self.pretty;

        let json = self.clone().into_json(result.duration());
        let string = if pretty {
            serde_json::to_string_pretty(&json)?
        } else {
            serde_json::to_string(&json)?
        };

        write!(out, "{}", string)?;

        Ok(())
    }

    fn end_test(&mut self, _out: &mut SuiteOutput, desc: SpecInfo) -> ReportResult {
        self.tests.push(desc);

        Ok(())
    }
}
