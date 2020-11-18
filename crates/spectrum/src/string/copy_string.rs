use derive_new::new;
use std::{borrow::Cow, fmt::Debug};

use crate::{Structure, Style, StyledFragment, StyledString};

#[derive(Debug, Clone)]
pub enum StringRepr<T = ()>
where
    T: Copy,
{
    String(&'static str),
    Other(T),
}

impl<T> Into<StringRepr<T>> for &'static str
where
    T: Copy,
{
    fn into(self) -> StringRepr<T> {
        StringRepr::String(self)
    }
}

impl<T> Copy for StringRepr<T> where T: Copy {}

/// This newtype makes it possible to implement Into<Structure> and Into<Fragment> on the string
/// context's repr
#[derive(new)]
pub struct Repr<Ctx>(Ctx::CustomRepr)
where
    Ctx: StringContext;

impl<Ctx> Into<Structure<Ctx>> for Repr<Ctx>
where
    Ctx: StringContext,
{
    fn into(self) -> Structure<Ctx> {
        StyledFragment::String(self.into()).into()
    }
}

impl<Ctx> Into<StyledFragment<Ctx>> for Repr<Ctx>
where
    Ctx: StringContext,
{
    fn into(self) -> StyledFragment<Ctx> {
        StyledFragment::String(self.into())
    }
}

impl<Ctx> Into<StyledString<Ctx>> for Repr<Ctx>
where
    Ctx: StringContext,
{
    fn into(self) -> StyledString<Ctx> {
        StyledString::repr(self.0, Style::default())
    }
}

pub trait StringContext: Debug + Default {
    /// CustomRepr must implement From<&'static str> so that static strings are generically allowed
    /// as string representations
    type CustomRepr: From<&'static str> + Copy + Debug;
    type InputCustomRepr: From<&'static str>;

    fn as_repr(&mut self, input: Self::InputCustomRepr) -> Repr<Self>;
    fn repr_as_string(&self, string: Self::CustomRepr) -> Cow<'_, str>;

    fn as_string(&mut self, input: Self::InputCustomRepr) -> Cow<'_, str> {
        let repr = self.as_repr(input);
        self.repr_as_string(repr.0)
    }

    fn styled_repr(input: Self::CustomRepr, style: impl Into<Style>) -> StyledString<Self> {
        StyledString::repr(input, style)
    }

    fn plain_repr(input: Self::CustomRepr) -> StyledString<Self> {
        StyledString::repr(input, Style::default())
    }

    fn styled(
        &mut self,
        input: Self::InputCustomRepr,
        style: impl Into<Style>,
    ) -> StyledString<Self> {
        let repr = self.as_repr(input).0;
        StyledString::repr(repr, style.into())
    }

    fn plain(&mut self, input: Self::InputCustomRepr) -> StyledString<Self> {
        let repr = self.as_repr(input).0;
        StyledString::repr(repr, Style::default())
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct SimpleContext;

impl StringContext for SimpleContext {
    type CustomRepr = &'static str;
    type InputCustomRepr = &'static str;

    fn as_repr(&mut self, input: &'static str) -> Repr<Self> {
        Repr(input)
    }

    fn repr_as_string(&self, string: &'static str) -> Cow<'_, str> {
        Cow::Borrowed(string)
    }
}
