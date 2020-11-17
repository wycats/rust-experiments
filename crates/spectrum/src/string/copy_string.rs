use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

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

pub trait StringContext: Debug + Default {
    type CustomRepr: Copy + Debug;

    fn as_string(&self, string: StringRepr<Self::CustomRepr>) -> Cow<'_, str> {
        match string {
            StringRepr::String(string) => Cow::Borrowed(string),
            StringRepr::Other(repr) => self.from_custom(repr),
        }
    }
    fn from_custom(&self, string: Self::CustomRepr) -> Cow<'_, str>;
}

impl StringContext for () {
    type CustomRepr = ();

    fn as_string(&self, string: StringRepr) -> Cow<'_, str> {
        match string {
            StringRepr::String(string) => Cow::Borrowed(string),
            StringRepr::Other(()) => Cow::Borrowed(""),
        }
    }

    fn from_custom(&self, _string: Self::CustomRepr) -> Cow<'_, str> {
        Cow::Borrowed("")
    }
}

#[derive(Debug)]
pub struct CopyString<Ctx>
where
    Ctx: StringContext,
{
    pub(crate) repr: StringRepr<Ctx::CustomRepr>,
    ctx: PhantomData<Ctx>,
}

impl<Ctx> CopyString<Ctx>
where
    Ctx: StringContext,
{
    pub fn str(repr: &'static str) -> CopyString<Ctx> {
        CopyString {
            repr: StringRepr::String(repr),
            ctx: PhantomData,
        }
    }

    pub fn custom(repr: Ctx::CustomRepr) -> CopyString<Ctx> {
        CopyString {
            repr: StringRepr::Other(repr),
            ctx: PhantomData,
        }
    }
}

impl<Ctx> Into<CopyString<Ctx>> for &'static str
where
    Ctx: StringContext,
{
    fn into(self) -> CopyString<Ctx> {
        CopyString {
            repr: StringRepr::String(self),
            ctx: PhantomData,
        }
    }
}

impl<Ctx> Clone for CopyString<Ctx>
where
    Ctx: StringContext,
{
    fn clone(&self) -> Self {
        CopyString {
            repr: self.repr,
            ctx: PhantomData,
        }
    }
}

impl<Ctx> Copy for CopyString<Ctx> where Ctx: StringContext {}
