use getset::Getters;
use std::fmt::Debug;

use crate::formatting::traits::{DescribeMatcher, MatchError};

pub type MatchResult = Result<(), MatchError>;

#[derive(Debug, Clone, Getters)]
pub struct Described<T>
where
    T: Clone + Debug,
{
    #[get = "pub"]
    pub description: String,
    #[get = "pub"]
    pub value: T,
}

impl<T> Described<T>
where
    T: Clone + Debug,
{
    pub fn new(desc: impl Into<String>, value: impl Into<T>) -> Described<T> {
        Described {
            description: desc.into(),
            value: value.into(),
        }
    }

    pub fn to_string(&self) -> Described<String> {
        Described {
            description: self.description.clone(),
            value: format!("{:?}", self.value),
        }
    }

    pub fn get(self) -> (String, T) {
        (self.description, self.value)
    }
}

pub struct Not<M>
where
    M: Matcher,
{
    matcher: M,
}

impl<M> Matcher for Not<M>
where
    M: Matcher,
{
    type Describe = M::Describe;
    type Pattern = M::Pattern;
    type Actual = M::Actual;

    fn should(&self) -> Self::Describe {
        self.matcher.should_not()
    }

    fn should_not(&self) -> Self::Describe {
        self.matcher.should()
    }

    fn matches(&self, actual: impl Into<Self::Actual>) -> MatchResult {
        self.matcher.matches_not(actual.into())
    }

    fn matches_not(&self, actual: impl Into<Self::Actual>) -> MatchResult {
        self.matcher.matches(actual.into())
    }

    fn new(expected: impl Into<Described<Self::Pattern>>) -> Self {
        Not {
            matcher: M::new(expected.into()),
        }
    }
}

pub trait Matcher {
    type Describe: DescribeMatcher<Self::Actual>;
    type Pattern: Clone + Debug;
    type Actual: Clone + Debug;

    fn new(expected: impl Into<Described<Self::Pattern>>) -> Self;

    fn should(&self) -> Self::Describe;
    fn should_not(&self) -> Self::Describe;

    fn matches(&self, actual: impl Into<Self::Actual>) -> MatchResult;

    fn matches_not(&self, actual: impl Into<Self::Actual>) -> MatchResult;

    fn not(self) -> Not<Self>
    where
        Self: Sized,
    {
        Not { matcher: self }
    }
}
