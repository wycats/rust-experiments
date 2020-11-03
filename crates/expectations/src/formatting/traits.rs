use std::{fmt::Debug, ops::Deref};

use crate::{
    emit::write::StyledEmitter, emit::StyledFragment, emit::StyledFragmentTrait, traits::Described,
    EmitWriter,
};

pub fn mismatch<T>(actual: T, describe: impl DescribeMatcher<T> + 'static) -> MatchError
where
    T: Clone + Debug + 'static,
{
    MatchError::new(NoMatch {
        actual,
        describe: Box::new(describe),
    })
}

pub struct NoMatch<T>
where
    T: Clone + Debug,
{
    actual: T,
    describe: Box<dyn DescribeMatcher<T>>,
}

pub struct MatchError {
    error: Box<dyn MatchErrorTrait>,
}

impl Debug for MatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let terse = self.as_terse();
        let writer = EmitWriter::borrowed_fmt(f);

        terse
            .emit_into(&mut StyledEmitter::unstyled(writer))
            .map_err(|_| std::fmt::Error)
    }
}

impl Clone for MatchError {
    fn clone(&self) -> MatchError {
        MatchErrorTrait::clone(&*self.error)
    }
}

impl Deref for MatchError {
    type Target = dyn MatchErrorTrait;

    fn deref(&self) -> &Self::Target {
        Deref::deref(&self.error)
    }
}

impl MatchError {
    pub fn new<T>(err: NoMatch<T>) -> MatchError
    where
        T: Clone + Debug + 'static,
    {
        MatchError {
            error: Box::new(err),
        }
    }
}

impl<T> Debug for NoMatch<T>
where
    T: Clone + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MatchError")
            .field("actual", &self.actual)
            .field("describe", &self.describe.name())
            .finish()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Verbosity {
    Terse,
    Detailed,
}

pub struct WritableError {
    error: MatchError,
    verbosity: Verbosity,
}

impl StyledFragmentTrait for WritableError {
    fn emit_into(&self, writer: &mut StyledEmitter) -> std::io::Result<()> {
        self.error.emit(self.verbosity, writer)
    }

    fn boxed_fragment(self, _style: Option<ansi_term::Style>) -> StyledFragment {
        StyledFragment::new(self)
    }
}

pub trait MatchErrorTrait: Debug {
    fn clone(&self) -> MatchError;

    fn emit(&self, verbosity: Verbosity, styled: &mut StyledEmitter) -> std::io::Result<()>;
}

impl MatchError {
    pub fn as_terse(&self) -> WritableError {
        WritableError {
            error: self.clone(),
            verbosity: Verbosity::Terse,
        }
    }
}

impl<T> MatchErrorTrait for NoMatch<T>
where
    T: Clone + Debug + 'static,
{
    // fn as_terse(&self, styled: &mut dyn EmitStyled) -> std::io::Result<()> {
    //     self.describe.terse(&self.actual, styled)
    // }

    // fn as_detailed(&self, styled: &mut dyn EmitStyled) -> std::io::Result<()> {
    //     todo!()
    // }

    fn emit(&self, verbosity: Verbosity, styled: &mut StyledEmitter) -> std::io::Result<()> {
        match verbosity {
            Verbosity::Terse => self.describe.terse(&self.actual, styled),
            Verbosity::Detailed => todo!(),
        }
    }

    fn clone(&self) -> MatchError {
        MatchError::new(NoMatch {
            actual: self.actual.clone(),
            describe: self.describe.clone(),
        })
    }
}

pub trait DescribeMatcher<Actual>
where
    Actual: Clone + Debug,
{
    fn name(&self) -> String {
        std::any::type_name::<Self>().to_string()
    }

    fn clone_matcher(&self) -> Box<dyn DescribeMatcher<Actual>>;

    fn expected(&self, styled: &mut StyledEmitter) -> std::io::Result<()>;
    fn terse(&self, actual: &Actual, styled: &mut StyledEmitter) -> std::io::Result<()>;
    fn detailed(
        &self,
        actual: Described<Actual>,
        styled: &mut StyledEmitter,
    ) -> std::io::Result<()>;
}

impl<Actual> Clone for Box<dyn DescribeMatcher<Actual>>
where
    Actual: Clone + Debug,
{
    fn clone(&self) -> Self {
        self.clone_matcher()
    }
}
