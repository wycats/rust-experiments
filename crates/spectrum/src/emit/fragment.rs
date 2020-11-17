use std::{
    fmt,
    fmt::{Debug, Formatter},
    io::Write,
};

use crate::string::copy_string::{CopyString, StringContext};

use super::{
    buf::Buf,
    error::{EmitError, EmitResult},
    style::Style,
};

#[derive(Debug)]
pub enum StyledFragment<Ctx = ()>
where
    Ctx: StringContext,
{
    String(StyledString<Ctx>),
    Line(StyledLine<Ctx>),
    Newline,
}

impl<Ctx> Clone for StyledFragment<Ctx>
where
    Ctx: StringContext,
{
    fn clone(&self) -> Self {
        match self {
            StyledFragment::String(s) => StyledFragment::String(s.clone()),
            StyledFragment::Newline => StyledFragment::Newline,
            StyledFragment::Line(line) => StyledFragment::Line(line.clone()),
        }
    }
}

impl<Ctx> Into<StyledFragment<Ctx>> for &'static str
where
    Ctx: StringContext,
{
    fn into(self) -> StyledFragment<Ctx> {
        StyledFragment::String(self.into())
    }
}

impl<Ctx> StyledFragment<Ctx>
where
    Ctx: StringContext,
{
    pub fn emit_plain(&self, ctx: &Ctx) -> String {
        self.emit_into_string_with(EmitPlain, ctx).unwrap()
    }

    pub fn plain(&self) -> String
    where
        Ctx: StringContext<CustomRepr = ()>,
    {
        self.emit_plain(&Ctx::default())
    }

    pub fn emit_into_formatter(
        &self,
        f: &mut Formatter<'_>,
        backend: &EmitBackend<'_>,
        ctx: &Ctx,
    ) -> EmitResult {
        match self {
            StyledFragment::String(s) => backend.emit(f, &ctx.as_string(s.string.repr), &s.style),
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

    pub fn emit_into_with(
        &self,
        write: &mut dyn std::io::Write,
        backend: &EmitBackend<'_>,
        ctx: &Ctx,
    ) -> EmitResult {
        let formatted = format::Display(move |f| Ok(self.emit_into_formatter(f, backend, ctx)?));
        Ok(write!(write, "{}", formatted)?)
    }

    pub fn emit_into(&self, write: &mut dyn std::io::Write, backend: &EmitBackend<'_>) -> EmitResult
    where
        Ctx: StringContext<CustomRepr = ()>,
    {
        self.emit_into_with(write, backend, &Ctx::default())
    }

    pub fn emit_into_string_with(
        &self,
        backend: impl EmitBackendTrait,
        ctx: &Ctx,
    ) -> EmitResult<String> {
        Ok(Buf::collect_string(|write| {
            Ok(self.emit_into_with(write, &backend.emitter(), ctx)?)
        })?)
    }

    pub fn emit_into_string(&self, backend: impl EmitBackendTrait) -> EmitResult<String>
    where
        Ctx: StringContext<CustomRepr = ()>,
    {
        self.emit_into_string_with(backend, &Ctx::default())
    }
}

#[derive(Debug)]
pub struct StyledNewline;

/// A `StyledString` is the simplest implementation of `StyledFragment`, holding a `String` and a
/// `Style`.
#[derive(Debug, Copy)]
pub struct StyledString<Ctx>
where
    Ctx: StringContext,
{
    string: CopyString<Ctx>,
    style: Style,
}

impl<Ctx> Clone for StyledString<Ctx>
where
    Ctx: StringContext,
{
    fn clone(&self) -> Self {
        StyledString {
            string: self.string,
            style: self.style,
        }
    }
}

impl<Ctx> Into<StyledFragment<Ctx>> for StyledString<Ctx>
where
    Ctx: StringContext,
{
    fn into(self) -> StyledFragment<Ctx> {
        StyledFragment::String(self)
    }
}

impl<Ctx> StyledString<Ctx>
where
    Ctx: StringContext,
{
    pub fn custom(s: Ctx::CustomRepr, style: impl Into<Style>) -> StyledString<Ctx> {
        StyledString {
            string: CopyString::custom(s),
            style: style.into(),
        }
    }

    pub fn str(s: &'static str, style: impl Into<Style>) -> StyledString<Ctx> {
        StyledString {
            string: CopyString::str(s),
            style: style.into(),
        }
    }
}

impl<Ctx> Into<StyledString<Ctx>> for &'static str
where
    Ctx: StringContext,
{
    fn into(self) -> StyledString<Ctx> {
        StyledString {
            string: self.into(),
            style: Style::default(),
        }
    }
}

#[derive(Debug)]
pub struct StyledLine<Ctx>
where
    Ctx: StringContext,
{
    line: Vec<StyledFragment<Ctx>>,
}

impl<Ctx> Into<StyledFragment<Ctx>> for StyledLine<Ctx>
where
    Ctx: StringContext,
{
    fn into(self) -> StyledFragment<Ctx> {
        StyledFragment::Line(self)
    }
}

impl<Ctx> Clone for StyledLine<Ctx>
where
    Ctx: StringContext,
{
    fn clone(&self) -> Self {
        StyledLine {
            line: self.line.clone(),
        }
    }
}

impl<Ctx> StyledLine<Ctx>
where
    Ctx: StringContext,
{
    pub fn new(fragments: Vec<StyledFragment<Ctx>>) -> StyledLine<Ctx> {
        StyledLine { line: fragments }
    }
}

/// An implementation of `EmitBackendTrait` takes a piece of styled text and emits it into the
/// supplied [std::fmt::Formatter].
pub trait EmitBackendTrait {
    fn emit(&self, f: &mut Formatter<'_>, fragment: &str, style: &Style) -> EmitResult;

    fn emitter(&self) -> EmitBackend<'_>
    where
        Self: Sized,
    {
        EmitBackend { backend: self }
    }
}

impl<'a, T> From<&'a T> for EmitBackend<'a>
where
    T: EmitBackendTrait + 'a,
{
    fn from(backend: &'a T) -> Self {
        backend.emitter()
    }
}

pub struct EmitBackend<'a> {
    backend: &'a dyn EmitBackendTrait,
}

impl<'a> EmitBackend<'a> {
    fn emit(&self, f: &mut Formatter<'_>, fragment: &str, style: &Style) -> EmitResult {
        self.backend.emit(f, fragment, style)
    }
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
    use crate::EmitForTest;

    use super::*;
    use console::Color;

    #[test]
    fn emit_test() -> EmitResult {
        let styled: StyledFragment =
            StyledString::str("hello emitter world", Style::new().fg(Color::Red)).into();
        let string = styled.emit_into_string(EmitForTest)?;

        assert_eq!(&string, "[Red:hello emitter world]");

        Ok(())
    }

    #[test]
    fn emit_plain() -> EmitResult {
        let styled: StyledFragment =
            StyledString::str("hello emitter world", Style::new().fg(Color::Red)).into();
        let string = styled.emit_into_string(EmitPlain)?;

        assert_eq!(&string, "hello emitter world");

        Ok(())
    }

    #[test]
    fn emit_colored() -> EmitResult {
        let styled: StyledFragment =
            StyledString::str("hello emitter world", Style::new().fg(Color::Red)).into();
        let string = styled.emit_into_string(EmitColored)?;

        assert_eq!(&string, "\u{1b}[31mhello emitter world\u{1b}[0m");

        Ok(())
    }
}
