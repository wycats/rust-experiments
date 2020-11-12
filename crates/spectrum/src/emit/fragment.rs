use std::{fmt, fmt::Formatter, io::Write};

use super::{
    buf::Buf,
    error::{EmitError, EmitResult},
    style::Style,
};

/// A [StyledFragmentTrait] represents a fragment of content that can be emitted into an output
/// stream (through an EmitBackend).
///
/// An implementation of StyledFragmentTrait must implement `emit_into_formatter`, and gets default
/// implementations of `emit_into`, which takes a `fmt::Write` and a backend and writes into the
/// `fmt::Write`, and `emit_into_string`, which takes a backend and produces a `String`.
///
/// In general, you should implement [StyledFragmentTrait] and store [StyledFragment].
pub trait StyledFragmentTrait {
    fn clone_frag(&self) -> StyledFragment;

    fn emit_into_formatter(&self, f: &mut Formatter<'_>, backend: &EmitBackend<'_>) -> EmitResult;
}

impl<T> From<T> for StyledFragment
where
    T: StyledFragmentTrait + 'static,
{
    fn from(value: T) -> StyledFragment {
        StyledFragment {
            fragment: Box::new(value),
        }
    }
}

/// A [StyledFragment] is a concrete value that represents an implementation of
/// [StyledFragmentTrait].
pub struct StyledFragment {
    fragment: Box<dyn StyledFragmentTrait + 'static>,
}

impl Clone for StyledFragment {
    fn clone(&self) -> Self {
        self.clone_frag()
    }
}

impl StyledFragment {
    pub fn new(frag: impl StyledFragmentTrait + 'static) -> StyledFragment {
        StyledFragment {
            fragment: Box::new(frag),
        }
    }

    pub fn clone_frag(&self) -> StyledFragment {
        self.fragment.clone_frag()
    }

    pub fn plain(&self) -> String {
        self.emit_into_string(EmitPlain).unwrap()
    }

    pub fn emit_into_formatter(
        &self,
        f: &mut Formatter<'_>,
        backend: &EmitBackend<'_>,
    ) -> EmitResult {
        self.fragment.emit_into_formatter(f, backend)
    }

    pub fn emit_into(
        &self,
        write: &mut dyn std::io::Write,
        backend: &EmitBackend<'_>,
    ) -> EmitResult {
        let formatted = format::Display(move |f| Ok(self.emit_into_formatter(f, backend)?));
        Ok(write!(write, "{}", formatted)?)
    }

    pub fn emit_into_string(&self, backend: impl EmitBackendTrait) -> EmitResult<String> {
        Ok(Buf::collect_string(|write| {
            Ok(self.emit_into(write, &backend.emitter())?)
        })?)
    }
}

pub struct StyledNewline;

impl StyledFragmentTrait for StyledNewline {
    fn emit_into_formatter(&self, f: &mut Formatter<'_>, backend: &EmitBackend<'_>) -> EmitResult {
        backend.emit(f, "\n", &Style::default())
    }

    fn clone_frag(&self) -> StyledFragment {
        StyledFragment::new(StyledNewline)
    }
}

/// A `StyledString` is the simplest implementation of `StyledFragment`, holding a `String` and a
/// `Style`.
#[derive(Debug, Clone)]
pub struct StyledString {
    string: String,
    style: Style,
}

impl StyledString {
    pub fn new(string: impl Into<String>, style: impl Into<Style>) -> StyledString {
        StyledString {
            string: string.into(),
            style: style.into(),
        }
    }
}

impl StyledFragmentTrait for StyledString {
    fn emit_into_formatter(&self, f: &mut Formatter<'_>, backend: &EmitBackend<'_>) -> EmitResult {
        backend.emit(f, &self.string[..], &self.style)
    }

    fn clone_frag(&self) -> StyledFragment {
        StyledFragment::new(self.clone())
    }
}

/// A [StyledLine] is a list of [StyledFragment]s, intended to be laid out on a single line
pub struct StyledLine {
    line: Vec<StyledFragment>,
}

impl StyledLine {
    pub fn new(fragments: Vec<StyledFragment>) -> StyledLine {
        StyledLine { line: fragments }
    }
}

impl StyledFragmentTrait for StyledLine {
    fn emit_into_formatter(&self, f: &mut Formatter<'_>, backend: &EmitBackend<'_>) -> EmitResult {
        for fragment in &self.line {
            fragment.emit_into_formatter(f, backend)?
        }

        Ok(())
    }

    fn clone_frag(&self) -> StyledFragment {
        StyledFragment::new(StyledLine {
            line: self.line.to_vec(),
        })
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
            StyledString::new("hello emitter world", Style::new().fg(Color::Red)).into();
        let string = styled.emit_into_string(EmitForTest)?;

        assert_eq!(&string, "[Red:hello emitter world]");

        Ok(())
    }

    #[test]
    fn emit_plain() -> EmitResult {
        let styled: StyledFragment =
            StyledString::new("hello emitter world", Style::new().fg(Color::Red)).into();
        let string = styled.emit_into_string(EmitPlain)?;

        assert_eq!(&string, "hello emitter world");

        Ok(())
    }

    #[test]
    fn emit_colored() -> EmitResult {
        let styled: StyledFragment =
            StyledString::new("hello emitter world", Style::new().fg(Color::Red)).into();
        let string = styled.emit_into_string(EmitColored)?;

        assert_eq!(&string, "\u{1b}[31mhello emitter world\u{1b}[0m");

        Ok(())
    }
}
