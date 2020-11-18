use std::{
    fmt,
    fmt::{Debug, Formatter},
    io::Write,
};

use crate::{string::copy_string::Repr, string::copy_string::StringContext, Primitive, Structure};

use super::{
    buf::Buf,
    error::{EmitError, EmitResult},
    style::Style,
};

#[derive(Debug)]
pub enum StyledFragment<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    String(StyledString<'a, Ctx>),
    Line(StyledLine<'a, Ctx>),
    Newline,
}

impl<'a, Ctx> Into<StyledString<'a, Ctx>> for &'static str
where
    Ctx: StringContext<'a>,
{
    fn into(self) -> StyledString<'a, Ctx> {
        Ctx::plain_repr(Repr::new(self.into()))
    }
}

impl<'a, Ctx> Into<StyledFragment<'a, Ctx>> for &'static str
where
    Ctx: StringContext<'a>,
{
    fn into(self) -> StyledFragment<'a, Ctx> {
        StyledFragment::String(self.into())
    }
}

impl<'a, Ctx> Into<Structure<'a, Ctx>> for StyledFragment<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    fn into(self) -> Structure<'a, Ctx> {
        Structure::Primitive(self.into())
    }
}

impl<'a, Ctx> Into<Primitive<'a, Ctx>> for StyledFragment<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    fn into(self) -> Primitive<'a, Ctx> {
        Primitive::Fragment(self)
    }
}

impl<'a, Ctx> Clone for StyledFragment<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    fn clone(&self) -> Self {
        match self {
            StyledFragment::String(s) => StyledFragment::String(s.clone()),
            StyledFragment::Newline => StyledFragment::Newline,
            StyledFragment::Line(line) => StyledFragment::Line(line.clone()),
        }
    }
}

impl<'a, Ctx> StyledFragment<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    pub fn emit_plain(&self, ctx: &Ctx) -> String {
        self.emit_into_string_with(&EmitPlain, ctx).unwrap()
    }

    pub fn plain(&self) -> String
    where
        Ctx: StringContext<'a, CustomRepr = ()>,
    {
        self.emit_plain(&Ctx::default())
    }

    pub fn emit_into_formatter(
        &self,
        f: &mut Formatter<'_>,
        backend: &dyn EmitBackendTrait,
        ctx: &Ctx,
    ) -> EmitResult {
        match self {
            StyledFragment::String(s) => {
                backend.emit(f, &ctx.repr_as_string(Repr::new(s.string)), &s.style)
            }
            StyledFragment::Newline => backend.emit(f, "\n", &Style::default()),
            StyledFragment::Line(line) => {
                for fragment in line.line.iter() {
                    fragment.emit_into_formatter(f, backend, ctx)?
                }

                Ok(())
            }
        }
        // self.fragment.emit_into_formatter(f, backend)
    }

    pub fn emit_into_with<'b>(
        &self,
        write: &mut dyn std::io::Write,
        backend: &dyn EmitBackendTrait,
        ctx: &'b Ctx,
    ) -> EmitResult
    where
        'a: 'b,
    {
        let formatted = format::Display(move |f| Ok(self.emit_into_formatter(f, backend, ctx)?));
        Ok(write!(write, "{}", formatted)?)
    }

    pub fn emit_into(
        &self,
        write: &mut dyn std::io::Write,
        backend: &dyn EmitBackendTrait,
    ) -> EmitResult
    where
        Ctx: StringContext<'a, CustomRepr = ()>,
    {
        let ctx = Ctx::default();
        self.emit_into_with(write, backend, &ctx)
    }

    pub fn emit_into_string_with(
        &self,
        backend: &dyn EmitBackendTrait,
        ctx: &Ctx,
    ) -> EmitResult<String> {
        Ok(Buf::collect_string(|write| {
            Ok(self.emit_into_with(write, backend, ctx)?)
        })?)
    }

    pub fn emit_into_string(&self, backend: &dyn EmitBackendTrait) -> EmitResult<String>
    where
        Ctx: StringContext<'a>,
    {
        self.emit_into_string_with(backend, &Ctx::default())
    }
}

#[derive(Debug)]
pub struct StyledNewline;

/// A `StyledString` is the simplest implementation of `StyledFragment`, holding a `String` and a
/// `Style`.
#[derive(Debug, Copy)]
pub struct StyledString<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    string: Ctx::CustomRepr,
    style: Style,
}

impl<'a, Ctx> Into<Structure<'a, Ctx>> for StyledString<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    fn into(self) -> Structure<'a, Ctx> {
        Structure::fragment(self)
    }
}

impl<'a, Ctx> Clone for StyledString<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    fn clone(&self) -> Self {
        StyledString {
            string: self.string,
            style: self.style,
        }
    }
}

impl<'a, Ctx> Into<StyledFragment<'a, Ctx>> for StyledString<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    fn into(self) -> StyledFragment<'a, Ctx> {
        StyledFragment::String(self)
    }
}

impl<'a, Ctx> StyledString<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    // pub fn str(s: Ctx::InputCustomRepr, style: impl Into<Style>) -> StyledString<Ctx>
    // where
    //     Ctx: StringContext,
    // {
    //     StyledString {
    //         string: Ctx::as_repr(s),
    //         style: style.into(),
    //     }
    // }

    pub fn repr(s: Ctx::CustomRepr, style: impl Into<Style>) -> StyledString<'a, Ctx>
    where
        Ctx: StringContext<'a>,
    {
        StyledString {
            string: s,
            style: style.into(),
        }
    }
}

#[derive(Debug)]
pub struct StyledLine<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    line: Vec<StyledFragment<'a, Ctx>>,
}

impl<'a, Ctx> Into<StyledFragment<'a, Ctx>> for StyledLine<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    fn into(self) -> StyledFragment<'a, Ctx> {
        StyledFragment::Line(self)
    }
}

impl<'a, Ctx> Clone for StyledLine<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    fn clone(&self) -> Self {
        StyledLine {
            line: self.line.clone(),
        }
    }
}

impl<'a, Ctx> StyledLine<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    pub fn new(fragments: Vec<StyledFragment<'a, Ctx>>) -> StyledLine<'a, Ctx> {
        StyledLine { line: fragments }
    }
}

/// An implementation of `EmitBackendTrait` takes a piece of styled text and emits it into the
/// supplied [std::fmt::Formatter].
pub trait EmitBackendTrait {
    fn emit(&self, f: &mut Formatter<'_>, fragment: &str, style: &Style) -> EmitResult;
}

pub struct EmitColored;

impl EmitBackendTrait for EmitColored {
    fn emit(&self, f: &mut Formatter<'_>, fragment: &str, style: &Style) -> EmitResult {
        write!(f, "{}", style.apply_to(fragment)).map_err(EmitError::new)
    }
}

pub struct EmitPlain;

impl EmitBackendTrait for EmitPlain {
    fn emit(&self, f: &mut Formatter<'_>, fragment: &str, _style: &Style) -> EmitResult {
        write!(f, "{}", fragment).map_err(EmitError::new)
    }
}

pub fn write_into(
    write: &mut impl Write,
    callback: impl Fn(&mut Formatter<'_>) -> fmt::Result,
) -> EmitResult {
    let formatted = format::Display(move |f| callback(f));
    Ok(write!(write, "{}", formatted)?)
}

#[cfg(test)]
mod tests {
    use crate::{string::copy_string::SimpleContext, EmitForTest};

    use super::*;
    use console::Color;

    #[test]
    fn emit_test() -> EmitResult {
        let styled: StyledFragment<SimpleContext> =
            SimpleContext::styled("hello emitter world", Color::Red).into();
        let string = styled.emit_into_string(&EmitForTest)?;

        assert_eq!(&string, "[Red:hello emitter world]");

        Ok(())
    }

    #[test]
    fn emit_plain() -> EmitResult {
        let styled: StyledFragment<SimpleContext> =
            SimpleContext::styled("hello emitter world", Color::Red).into();
        let string = styled.emit_into_string(&EmitPlain)?;

        assert_eq!(&string, "hello emitter world");

        Ok(())
    }

    #[test]
    fn emit_colored() -> EmitResult {
        let styled: StyledFragment<SimpleContext> =
            SimpleContext::styled("hello emitter world", Color::Red).into();
        let string = styled.emit_into_string(&EmitColored)?;

        assert_eq!(&string, "\u{1b}[31mhello emitter world\u{1b}[0m");

        Ok(())
    }
}
