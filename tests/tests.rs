mod macros;

// use ansi_term::{ANSIGenericStrings, Color, Style};
// use difference::Difference;
// use laboratory::test::*;
// use laboratory_expectations::ShouldSugar;
// use laboratory_test_helpers::assert_eq;
// use std::fs::read_to_string;

// const EXPECTED_FOLDER: &str = "./tests/expected";
// const OUTPUT_FOLDER: &str = "./tests/output";

// fn get_output_path(test_name: &str) -> String {
//     let mut path = String::from(OUTPUT_FOLDER);
//     path += &format!("/{}", test_name);
//     path
// }

// fn get_expected_path(test_name: &str) -> String {
//     let mut path = String::from(EXPECTED_FOLDER);
//     path += &format!("/{}", test_name);
//     path
// }

// fn get_approval_file(test_name: &str) -> String {
//     read_to_string(get_expected_path(test_name))
//         .unwrap_or_else(|_| panic!("Could not find {}", get_expected_path(test_name)))
// }

// #[test]
// fn get_aprv_file() {
//     let result = get_expected_path("my-test");
//     assert_eq!("./tests/expected/my-test".to_string(), result);
// }

// #[test]
// fn simple_pass() -> ReportResult<()> {
//     fn return_one() -> i32 {
//         1
//     }

//     const TEST_NAME: &str = "simple";

//     let actual = describe("add_one()")
//         .specs(|it| {
//             it.should("return 1", || return_one().should().eq(1));
//         })
//         .export_to(&get_output_path(TEST_NAME))
//         .to_string()?;

//     let control = get_approval_file(TEST_NAME);
//     assert_eq!(actual, control);

//     Ok(())
// }

// #[test]
// fn simple_fail() -> ReportResult {
//     fn add_one() -> i32 {
//         0
//     }

//     const TEST_NAME: &str = "simple_fail";

//     let actual = describe("add_one")
//         .reporter(SpecReporter)
//         .specs(|it| it.should("return 1", || expect(add_one()).to_equal(1)))
//         .export_to(&get_output_path(TEST_NAME))
//         .to_string()?;

//     let control = get_approval_file(TEST_NAME);
//     assert_eq!(actual, control);

//     Ok(())
// }

// #[test]
// fn min() -> ReportResult {
//     fn add_one() -> i32 {
//         1
//     }

//     const TEST_NAME: &str = "min";

//     let actual = describe("add_one")
//         .reporter(MinimalReporter)
//         .specs(|it| it.should("return 1", || expect(add_one()).to_equal(1)))
//         .export_to(&get_output_path(TEST_NAME))
//         .to_string()?;

//     let control = get_approval_file(TEST_NAME);
//     assert_eq!(actual, control);

//     Ok(())
// }

// #[test]
// fn min_fail() -> ReportResult {
//     fn return1() -> i32 {
//         0
//     }

//     const TEST_NAME: &str = "min_fail";

//     let actual = describe("return1")
//         .reporter(MinimalReporter)
//         .specs(|it| it.should("return 1", || expect(return1()).to_equal(1)))
//         .export_to(&get_output_path(TEST_NAME))
//         .to_string()?;

//     let control = get_approval_file(TEST_NAME);
//     assert_eq!(actual, control);

//     Ok(())
// }

// #[test]
// fn json() -> ReportResult {
//     #[allow(unused)]
//     use serde_json::from_str;

//     fn add_one() -> i32 {
//         1
//     }

//     const TEST_NAME: &str = "output_json.json";

//     describe("add_one")
//         .reporter(JsonReporter::new())
//         .specs(|it| it.should("return 1", || expect(add_one()).to_equal(1)))
//         .export_to(&get_output_path(TEST_NAME))
//         .to_string()?;

//     // let _result: SuiteResult = from_str(&actual).expect("could not serialize the result");

//     Ok(())
// }

// #[test]
// fn json_pretty() -> ReportResult {
//     #[allow(unused)]
//     use serde_json::from_str;

//     fn add_one() -> i32 {
//         1
//     }

//     const TEST_NAME: &str = "output_json_pretty.json";

//     describe("add_one")
//         .reporter(JsonReporter::pretty())
//         .specs(|it| it.should("return 1", || expect(add_one()).to_equal(1)))
//         .export_to(&get_output_path(TEST_NAME))
//         .to_string()?;

//     // let _result: SuiteResult = from_str(&result_str).expect("could not serialize the result");

//     Ok(())
// }

// #[test]
// fn suite_skip() -> ReportResult {
//     fn add_one() -> i32 {
//         1
//     }

//     fn return_two() -> i32 {
//         2
//     }

//     const TEST_NAME: &str = "suite_skip";
//     let actual = describe("Library")
//         .suites(vec![
//             describe_skip("add_one()")
//                 .specs(|it| it.should("return 1", || expect(add_one()).to_equal(1))),
//             describe("return_two()")
//                 .specs(|it| it.should("return 2", || expect(return_two()).to_equal(2))),
//         ])
//         .export_to(&get_output_path(TEST_NAME))
//         .to_string()?;

//     let control = get_approval_file(TEST_NAME);
//     assert_eq!(actual, control);

//     Ok(())
// }

// #[test]
// fn spec_skip() -> ReportResult {
//     fn add_one() -> i32 {
//         1
//     }

//     fn return_two() -> i32 {
//         2
//     }

//     const TEST_NAME: &str = "spec_skip";

//     let actual = describe("Library")
//         .suites(vec![
//             describe("add_one()").specs(|it| {
//                 it.skip("return 1", || expect(add_one()).to_equal(1));
//                 it.should("return 1", || expect(add_one()).to_equal(1));
//             }),
//             describe("return_two()")
//                 .specs(|it| it.should("return 2", || expect(return_two()).to_equal(2))),
//         ])
//         .export_to(&get_output_path(TEST_NAME))
//         .to_string()?;

//     let control = get_approval_file(TEST_NAME);
//     assert_eq!(actual, control);

//     Ok(())
// }

// #[test]
// fn spec_only() -> ReportResult {
//     fn add_one() -> i32 {
//         1
//     }

//     fn return_two() -> i32 {
//         2
//     }

//     const TEST_NAME: &str = "spec_only";
//     let result_str = describe("Library")
//         .suites(vec![
//             describe("add_one()").specs(|it| {
//                 it.only("return 1", || expect(add_one()).to_equal(1));
//                 it.should("return 3", || expect(add_one()).to_equal(3));
//             }),
//             describe("return_two()")
//                 .specs(|it| it.should("return 2", || expect(return_two()).to_equal(2))),
//         ])
//         .export_to(&get_output_path(TEST_NAME))
//         .to_string()?;

//     let control = get_approval_file(TEST_NAME);
//     assert_eq!(result_str, control);

//     Ok(())
// }

// #[test]
// fn state_passing() -> ReportResult {
//     #[derive(Clone, Debug)]
//     struct Counter {
//         count: i32,
//     }

//     impl Counter {
//         pub fn new() -> Counter {
//             Counter { count: 0 }
//         }
//     }

//     fn return_count(counter: &mut Counter) -> i32 {
//         counter.count
//     }
//     fn return_incr_count(counter: &mut Counter) -> i32 {
//         counter.count + 1
//     }

//     let actual = describe("Library")
//         .state(Counter::new())
//         .describe("return_count()", |it| {
//             it.should("return 1", |counter| {
//                 counter.count += 1;
//                 expect(return_count(counter)).to_equal(1)
//             });
//             it.should("return 1 again", |counter| {
//                 counter.count += 1;
//                 expect(return_count(counter)).to_equal(1)
//             });
//         })
//         .describe("return_incr_count()", |it| {
//             it.should("return 2", |counter| {
//                 counter.count += 1;
//                 expect(return_incr_count(counter)).to_equal(2)
//             })
//         })
//         .suite(describe("return_two()").specs(|it| it.should("return 2", || expect(2).to_equal(2))))
//         .to_string()?;

//     const TEST_NAME: &str = "state_passing";
//     let control = get_approval_file(TEST_NAME);
//     assert_eq!(actual, control);

//     Ok(())
// }

// #[test]
// fn mutable_state_passing() -> ReportResult {
//     #[derive(Clone, Debug)]
//     struct Counter {
//         count: i32,
//     }

//     impl Counter {
//         pub fn new() -> Counter {
//             Counter { count: 0 }
//         }
//     }

//     fn return_one() -> i32 {
//         1
//     }
//     fn return_two() -> i32 {
//         2
//     }

//     let counter = describe("Library")
//         .mutable_state(Counter::new())
//         .describe("return_one()", |it| {
//             it.should("return 1", |counter| {
//                 counter.count += 1;
//                 expect(return_one()).to_equal(1)
//             });
//             it.should("return 1 again", |counter| {
//                 counter.count += 1;
//                 expect(return_one()).to_equal(1)
//             });
//         })
//         .suite(
//             describe("return_two()")
//                 .specs(|it| it.should("return 2", || expect(return_two()).to_equal(2))),
//         )
//         .run_with(ReporterOutput::null())?
//         .into_state();

//     assert_eq!(counter.count, 2);

//     Ok(())
// }

// #[test]
// fn return_result() -> ReportResult {
//     fn add_one(n: i32) -> i32 {
//         n + 1
//     }

//     describe("add_one()")
//         .specs(|it| {
//             it.should("return 1", || expect(add_one(0)).to_equal(1));
//             it.should("return 2", || expect(add_one(0)).to_equal(2));
//         })
//         .to_string()?;

//     Ok(())
// }

// #[test]
// fn micro() -> ReportResult {
//     fn return_one() -> i32 {
//         1
//     }

//     const TEST_NAME: &str = "micro";

//     let actual = describe("add_one()")
//         .specs(|it| it.should("return 1", || expect(return_one()).to_equal(1)))
//         .export_to(&get_output_path(TEST_NAME))
//         .in_microseconds()
//         .to_string()?;
//     // simple spec pass
//     // let control = get_approval_file(TEST_NAME);
//     assert!(actual.contains("µs)"), format!("actual: {:?}", actual));

//     Ok(())
// }

// #[test]
// fn nano() -> ReportResult {
//     fn return_one() -> i32 {
//         1
//     }

//     const TEST_NAME: &str = "nano";

//     let actual = describe("add_one()")
//         .specs(|it| it.should("return 1", || expect(return_one()).to_equal(1)))
//         .export_to(&get_output_path(TEST_NAME))
//         .in_nanoseconds()
//         .to_string()?;

//     // let control = get_approval_file(TEST_NAME);
//     assert!(actual.contains("ns)"));

//     Ok(())
// }

// #[test]
// fn seconds() -> ReportResult {
//     fn return_one() -> i32 {
//         1
//     }

//     const TEST_NAME: &str = "seconds";

//     let actual = describe("add_one()")
//         .specs(|it| it.should("return 1", || expect(return_one()).to_equal(1)))
//         .export_to(&get_output_path(TEST_NAME))
//         .in_seconds()
//         .to_string()?;

//     // let control = get_approval_file(TEST_NAME);
//     assert!(actual.contains("sec)"), "actual: {:?}", actual);

//     Ok(())
// }
