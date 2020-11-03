// mod curry;
// mod reporter;
// mod spec;
// mod spec_result;
// mod suite;
// mod suite_result;
// pub mod test;

// pub use laboratory_expectations::Expect;
// pub use reporter::{
//     json::JsonReporter,
//     output::ReporterOutput,
//     spec::{MinimalReporter, SpecReporter, StartedSpecReporter},
//     ReportResult, Reporter,
// };
// pub use serde::{Deserialize, Serialize};
// pub use spec::Spec;
// use suite::described::{DescribedSuite, SuiteDetails};
// pub use suite::{traits::SuiteExt, DurationPrecision, Suite};
// pub use suite_result::SuiteOutcome;

// #[macro_export]
// macro_rules! should_panic {
//     ($name:expr, $handle: expr) => {{
//         use std::panic::{catch_unwind, set_hook, take_hook};

//         set_hook(Box::new(|_| {
//             // println!("");
//         }));
//         let tmp_result = catch_unwind(|| {
//             ($handle)();
//         })
//         .is_ok();
//         let _ = take_hook();
//         if tmp_result == false {
//             Ok(())
//         } else {
//             Err(format!(
//                 "Expected {} to panic but it didn't",
//                 stringify!($name)
//             ))
//         }
//     }};
// }

// #[macro_export]
// macro_rules! should_not_panic {
//     ($name:expr, $handle: expr) => {{
//         use std::panic::{catch_unwind, set_hook, take_hook};

//         set_hook(Box::new(|_| {
//             // println!("");
//         }));
//         let tmp_result = catch_unwind(|| {
//             ($handle)();
//         })
//         .is_ok();
//         let _ = take_hook();
//         if tmp_result == true {
//             Ok(())
//         } else {
//             Err(format!(
//                 "Expected {} to panic but it didn't",
//                 stringify!($name)
//             ))
//         }
//     }};
// }

// pub fn describe(name: &'static str) -> DescribedSuite {
//     DescribedSuite::new(name.to_string())
// }

// pub fn describe_skip(name: &'static str) -> DescribedSuite {
//     DescribedSuite::new(name.to_string()).skip()
// }

// trait SpecCallback {
//     type State: Clone + 'static;

//     fn call(&mut self, state: &mut Self::State) -> Result<(), String>;
// }

// impl<T> SpecCallback for Box<dyn FnMut(&mut T) -> Result<(), String>>
// where
//     T: Clone + 'static,
// {
//     type State = T;

//     fn call(&mut self, state: &mut Self::State) -> Result<(), String> {
//         (self)(state)
//     }
// }

// impl SpecCallback for Box<dyn FnMut() -> Result<(), String>> {
//     type State = ();

//     fn call(&mut self, _state: &mut ()) -> Result<(), String> {
//         (self)()
//     }
// }

// // pub fn it<T>(
// //     name: impl Into<String>,
// //     callback: impl Fn(&mut T) -> Result<(), String> + 'static,
// // ) -> TypedSpec<T>
// // where
// //     T: Clone + Debug + 'static,
// // {
// //     TypedSpec::new(name.into(), Box::new(callback))
// // }

// // pub fn it_skip<T>(
// //     name: impl Into<String>,
// //     callback: impl FnMut(&mut T) -> Result<(), String> + 'static,
// // ) -> TypedSpec<T>
// // where
// //     T: Clone + Debug + 'static,
// // {
// //     let spec = TypedSpec::new(name.into(), Box::new(callback));
// //     spec.skip()
// // }

// // pub fn it_only<T>(
// //     name: &'static str,
// //     callback: impl FnMut(&mut T) -> Result<(), String> + 'static,
// // ) -> TypedSpec<T>
// // where
// //     T: Clone + Debug + 'static,
// // {
// //     TypedSpec::new(name.to_string(), Box::new(callback)).only()
// // }
