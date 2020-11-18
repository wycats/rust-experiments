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
pub struct Repr<'a, Ctx>(Ctx::CustomRepr)
where
    Ctx: StringContext<'a>;

impl<'a, Ctx> Repr<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    pub(crate) fn value(self) -> Ctx::CustomRepr {
        self.0
    }
}

impl<'a, Ctx> Into<Structure<'a, Ctx>> for Repr<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    fn into(self) -> Structure<'a, Ctx> {
        StyledFragment::String(self.into()).into()
    }
}

impl<'a, Ctx> Into<StyledFragment<'a, Ctx>> for Repr<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    fn into(self) -> StyledFragment<'a, Ctx> {
        StyledFragment::String(self.into())
    }
}

impl<'a, Ctx> Into<StyledString<'a, Ctx>> for Repr<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    fn into(self) -> StyledString<'a, Ctx> {
        StyledString::repr(self.0, Style::default())
    }
}

pub trait StringContext<'a>: Debug + Default {
    /// CustomRepr must implement From<&'static str> so that static strings are generically allowed
    /// as string representations
    type CustomRepr: From<&'a str> + Copy + 'a + Debug;
    type ValidInput;

    fn take(&mut self, value: impl Into<Self::ValidInput> + 'a) -> Repr<'a, Self>;

    fn as_repr(value: impl Into<Self::CustomRepr> + 'a) -> Repr<'a, Self> {
        Repr::new(value.into())
    }

    fn repr_as_string<'b>(&'b self, string: Repr<'a, Self>) -> Cow<'b, str>
    where
        'a: 'b,
        Self::CustomRepr: 'a;

    fn styled(
        input: impl Into<Self::CustomRepr> + 'a,
        style: impl Into<Style>,
    ) -> StyledString<'a, Self> {
        Self::styled_repr(Repr(input.into()), style)
    }

    fn styled_repr(input: Repr<'a, Self>, style: impl Into<Style>) -> StyledString<'a, Self> {
        StyledString::repr(input.0, style)
    }

    fn plain(input: impl Into<Self::CustomRepr> + 'a) -> StyledString<'a, Self> {
        Self::styled_repr(Repr(input.into()), Style::default())
    }

    fn plain_repr(input: Repr<'a, Self>) -> StyledString<'a, Self> {
        StyledString::repr(input.0, Style::default())
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct SimpleContext;

impl<'a> StringContext<'a> for SimpleContext {
    type CustomRepr = &'a str;
    type ValidInput = &'a str;

    fn repr_as_string<'b>(&'b self, string: Repr<'a, Self>) -> Cow<'b, str>
    where
        'a: 'b,
    {
        Cow::Borrowed(string.0)
    }

    fn take(&mut self, value: impl Into<&'a str>) -> Repr<'a, Self> {
        Repr::new(value.into())
    }
}
