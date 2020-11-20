use std::{fmt, fmt::Formatter, io::Write};

use crate::{compose::Doc, RenderConfig, RenderContext};

use super::{
    buf::Buf,
    error::{EmitError, EmitResult},
    style::Style,
};

/// An implementation of `EmitBackendTrait` takes a piece of styled text and emits it into the
/// supplied [std::fmt::Formatter].
pub trait EmitBackendTrait {
    fn emit(&self, f: &mut Formatter<'_>, fragment: &str, style: Style) -> EmitResult;

    fn emit_string(&self, fragment: &str, style: Style) -> String {
        Buf::collect_string(|write| {
            write!(
                write,
                "{}",
                format::Display(move |f| Ok(self.emit(f, fragment, style)?))
            )
            .map_err(|_| std::fmt::Error)
            // format::lazy_format!(|f| self.emit(f, fragment, style).map_err(|_| std::fmt::Error))
        })
        .unwrap()
    }

    fn render(self, text: &impl Doc, page_size: usize) -> Result<String, std::fmt::Error>
    where
        Self: Sized,
    {
        Buf::collect_string(|writer| {
            let mut context = RenderContext::new(writer);
            context.render(text, self, RenderConfig::width(page_size))?;

            Ok(())
        })
    }
}

pub struct EmitColored;

impl EmitBackendTrait for EmitColored {
    fn emit(&self, f: &mut Formatter<'_>, fragment: &str, style: Style) -> EmitResult {
        write!(f, "{}", style.apply_to(fragment)).map_err(EmitError::new)
    }
}

pub struct EmitPlain;

impl EmitBackendTrait for EmitPlain {
    fn emit(&self, f: &mut Formatter<'_>, fragment: &str, _style: Style) -> EmitResult {
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
        assert_eq!(
            EmitForTest.emit_string("hello emitter world", Color::Red.into()),
            "[Red:hello emitter world]"
        );

        Ok(())
    }

    #[test]
    fn emit_plain() -> EmitResult {
        assert_eq!(
            EmitPlain.emit_string("hello emitter world", Color::Red.into()),
            "hello emitter world"
        );

        Ok(())
    }

    #[test]
    fn emit_colored() -> EmitResult {
        assert_eq!(
            EmitColored.emit_string("hello emitter world", Color::Red.into()),
            "\u{1b}[31mhello emitter world\u{1b}[0m"
        );

        Ok(())
    }
}
