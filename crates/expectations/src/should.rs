use std::fmt::Debug;

use crate::{
    builtins::eq::Equal,
    traits::Matcher,
    traits::{Described, MatchResult},
};

// use std::ops::Eq;

pub struct Should<T>
where
    T: Clone + Debug,
{
    actual: T,
}

impl<T> Should<T>
where
    T: Clone + Debug,
{
    pub fn eq(self, pattern: impl Into<T>) -> MatchResult
    where
        T: PartialEq + 'static,
    {
        let matcher = Equal::<T>::new(Described::new("expected", pattern.into()));
        matcher.matches(self.actual)

        // Expectation::<Equal<T>>::new(
        //     Described::new("actual", self.actual),
        //     Described::new("expected", pattern.into()),
        // )
        // .check()
    }
}

pub trait ShouldSugar: Clone + Debug {
    fn should(self) -> Should<Self> {
        Should { actual: self }
    }
}

impl<T> ShouldSugar for T where T: Clone + Debug {}
