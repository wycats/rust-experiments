use std::fmt::Debug;

use derive_new::new;

use crate::{
    formatting::inline::PatternMatcher,
    formatting::traits::mismatch,
    traits::MatchResult,
    traits::{Described, Matcher},
};

#[derive(new)]
pub struct Equal<T>
where
    T: PartialEq + Clone + Debug + 'static,
{
    expected: Described<T>,
}

impl<T> Matcher for Equal<T>
where
    T: PartialEq + Clone + Debug + 'static,
{
    type Describe = PatternMatcher<T>;
    type Pattern = T;
    type Actual = T;

    fn new(expected: impl Into<Described<T>>) -> Self {
        Equal {
            expected: expected.into(),
        }
    }

    fn should(&self) -> PatternMatcher<T> {
        PatternMatcher::new(self.expected.clone(), "==".into())
    }

    fn should_not(&self) -> PatternMatcher<T> {
        PatternMatcher::new(self.expected.clone(), "!=".into())
    }

    fn matches(&self, actual: impl Into<T>) -> MatchResult {
        let actual = actual.into();
        if self.expected.value() == &actual {
            Ok(())
        } else {
            Err(mismatch(
                actual,
                PatternMatcher::new(self.expected.clone(), "==".to_string()),
            ))
        }
    }

    fn matches_not(&self, actual: impl Into<T>) -> MatchResult {
        let actual = actual.into();
        if self.expected.value() != &actual {
            Ok(())
        } else {
            Err(mismatch(
                actual,
                PatternMatcher::new(self.expected.clone(), "!=".to_string()),
            ))
        }
    }
}
