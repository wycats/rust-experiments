// #[doc(hidden)]
// #[macro_use]
// pub mod emit;

// pub mod builtins;
// mod formatting;
// mod should;
// pub mod traits;

// use std::{cmp::PartialEq, fmt::Debug};

// use builtins::eq::Equal;
// use traits::{Described, Matcher};

// #[doc(hidden)]
// pub use ansi_term;

// pub use ansi_term::Style;
// pub use emit::{
//     write::emitter, write::EmitWriter, write::StyledEmitter, write::StyledEmitterTrait, Styled,
// };
// pub use formatting::traits::MatchError;
// pub use should::ShouldSugar;
// pub use traits::MatchResult;

// pub struct Expect<T>
// where
//     T: Debug + Clone,
// {
//     pub expected: T,
// }
// impl<T> Expect<T>
// where
//     T: Debug + Clone,
// {
//     pub fn new(expect: T) -> Expect<T> {
//         Expect { expected: expect }
//     }
//     pub fn expect(result: T) -> Expect<T> {
//         Expect { expected: result }
//     }
//     pub fn equals(self, actual: T) -> MatchResult
//     where
//         T: PartialEq + 'static,
//     {
//         let matcher = Equal::<T>::new(Described::new("expected", self.expected));
//         matcher.matches(actual)

//         // let actual = Described::new("actual", actual)
//         // Expectation::<Equal<T>>::new(
//         //     Described::new("actual", actual),
//         //     ,
//         // )
//         // .check_string()
//     }
//     #[allow(clippy::wrong_self_convention)]
//     pub fn to_equal(self, control: T) -> MatchResult
//     where
//         T: PartialEq + 'static,
//     {
//         self.equals(control)
//     }

//     #[allow(clippy::wrong_self_convention)]
//     pub fn to_be(self, control: T) -> MatchResult
//     where
//         T: PartialEq + 'static,
//     {
//         self.equals(control)
//     }

//     // pub fn to_not_equal(&self, control: T) -> Result<(), String> {
//     //     if self.expected != control {
//     //         Ok(())
//     //     } else {
//     //         Err(format!(
//     //             "Expected {:#?} not to equal {:#?}",
//     //             self.expected, control
//     //         ))
//     //     }
//     // }
//     // pub fn to_not_be(&self, control: T) -> Result<(), String> {
//     //     if self.expected != control {
//     //         Ok(())
//     //     } else {
//     //         Err(format!(
//     //             "Expected {:#?} not to be {:#?}",
//     //             self.expected, control
//     //         ))
//     //     }
//     // }
// }

// pub fn expect<T>(result: T) -> Expect<T>
// where
//     T: Debug + Clone,
// {
//     Expect::new(result)
// }
