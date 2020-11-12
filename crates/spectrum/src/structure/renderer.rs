use std::{
    error::Error,
    io::{self, stdout},
};

use pretty::{Render, RenderAnnotated};

use crate::{EmitBackend, StyledFragment};

pub struct StyledRenderer<'a> {
    write: Box<dyn io::Write + 'a>,
    backend: EmitBackend<'a>,
}

impl<'a> StyledRenderer<'a> {
    #[allow(unused)]
    pub fn new(
        write: impl io::Write + 'a,
        backend: impl Into<EmitBackend<'a>>,
    ) -> StyledRenderer<'a> {
        StyledRenderer {
            write: Box::new(write),
            backend: backend.into(),
        }
    }

    #[allow(unused)]
    pub fn stdout(backend: impl Into<EmitBackend<'a>>) -> StyledRenderer<'a> {
        StyledRenderer {
            write: Box::new(stdout()),
            backend: backend.into(),
        }
    }
}

impl<'a> Render for StyledRenderer<'a> {
    type Error = Box<dyn Error>;

    fn write_str(&mut self, s: &str) -> Result<usize, Self::Error> {
        Ok(s.len())
    }

    fn fail_doc(&self) -> Self::Error {
        Box::new(io::Error::new(
            io::ErrorKind::Other,
            "Document failed to render",
        ))
    }
}

impl<'a> RenderAnnotated<'a, StyledFragment> for StyledRenderer<'_> {
    fn push_annotation(&mut self, annotation: &'a StyledFragment) -> Result<(), Self::Error> {
        Ok(annotation.emit_into(&mut *self.write, &self.backend)?)
    }

    fn pop_annotation(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use console::Color;

    use super::*;
    use crate::{emit::buf::Buf, structure::*, EmitForTest, EmitPlain, Style, StyledString};

    #[test]
    fn basic_render() -> Result<(), Box<dyn Error>> {
        let structure =
            Structure::fragment(StyledString::new("hello", Style::default().fg(Color::Red)))
                .append(Structure::Hardline);

        let pretty = structure.render();

        let string = Buf::collect_string(|write| {
            let mut renderer = StyledRenderer::new(write, &EmitPlain);
            pretty
                .render_raw(100, &mut renderer)
                .map_err(|_| std::fmt::Error)
        })?;

        assert_eq!(string, "hello\n");

        Ok(())
    }

    #[test]
    fn colored_render() -> Result<(), Box<dyn Error>> {
        let structure =
            Structure::fragment(StyledString::new("hello", Style::default().fg(Color::Red)))
                .append(Structure::Hardline);

        let pretty = structure.render();

        let string = Buf::collect_string(|write| {
            let mut renderer = StyledRenderer::new(write, &EmitForTest);
            pretty
                .render_raw(100, &mut renderer)
                .map_err(|_| std::fmt::Error)
        })?;

        assert_eq!(string, "[Red:hello]\n");

        Ok(())
    }
}
