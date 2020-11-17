use std::{
    error::Error,
    io::{self, stdout},
};

use pretty::{Render, RenderAnnotated};

use crate::{string::copy_string::StringContext, EmitBackend, StyledFragment};

pub struct StyledRenderer<'a, Ctx>
where
    Ctx: StringContext,
{
    write: Box<dyn io::Write + 'a>,
    backend: EmitBackend<'a>,
    ctx: Ctx,
    /// remember whether we saw an annotation, which means we don't need to emit the string when
    /// write_str is called.
    ann: bool,
}

impl<'a, Ctx> StyledRenderer<'a, Ctx>
where
    Ctx: StringContext,
{
    #[allow(unused)]
    pub fn new(
        write: impl io::Write + 'a,
        backend: impl Into<EmitBackend<'a>>,
        context: Ctx,
    ) -> StyledRenderer<'a, Ctx>
    where
        Ctx: StringContext,
    {
        StyledRenderer {
            write: Box::new(write),
            backend: backend.into(),
            ann: false,
            ctx: context,
        }
    }

    #[allow(unused)]
    pub fn stdout(backend: impl Into<EmitBackend<'a>>, context: Ctx) -> StyledRenderer<'a, Ctx> {
        StyledRenderer {
            write: Box::new(stdout()),
            backend: backend.into(),
            ann: false,
            ctx: context,
        }
    }
}

impl<'a, Ctx> Render for StyledRenderer<'a, Ctx>
where
    Ctx: StringContext,
{
    type Error = Box<dyn Error>;

    fn write_str(&mut self, s: &str) -> Result<usize, Self::Error> {
        if self.ann {
            self.ann = false;
        } else {
            write!(self.write, "{}", s)?;
        }

        Ok(s.len())
    }

    fn fail_doc(&self) -> Self::Error {
        Box::new(io::Error::new(
            io::ErrorKind::Other,
            "Document failed to render",
        ))
    }
}

impl<'a, Ctx> RenderAnnotated<'a, StyledFragment<Ctx>> for StyledRenderer<'_, Ctx>
where
    Ctx: StringContext,
{
    fn push_annotation(&mut self, annotation: &'a StyledFragment<Ctx>) -> Result<(), Self::Error> {
        self.ann = true;
        Ok(annotation.emit_into_with(&mut *self.write, &self.backend, &self.ctx)?)
    }

    fn pop_annotation(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use console::{Attribute, Color};
    use format::lazy_format;
    use pretty::{Arena, DocAllocator};

    use super::*;
    use crate::render::Render;
    use crate::{
        emit::buf::Buf, structure::test::render, structure::Structure, EmitForTest, EmitPlain,
        Style, StyledString, GAP,
    };

    #[test]
    fn basic_render() -> Result<(), Box<dyn Error>> {
        let structure: Structure<()> =
            Structure::fragment(StyledString::str("hello", Style::default().fg(Color::Red)))
                .append(Structure::hardline());

        let pretty = structure.render();

        let string = Buf::collect_string(|write| {
            let mut renderer = StyledRenderer::new(write, &EmitPlain, ());
            pretty
                .render_raw(100, &mut renderer)
                .map_err(|_| std::fmt::Error)
        })?;

        assert_eq!(string, "hello\n");

        Ok(())
    }

    #[test]
    fn colored_render() -> Result<(), Box<dyn Error>> {
        let structure: Structure<()> =
            Structure::fragment(StyledString::str("hello", Style::default().fg(Color::Red)))
                .append(Structure::hardline());

        let pretty = structure.render();

        eprintln!("{:#?}", pretty);

        let string = Buf::collect_string(|write| {
            let mut renderer = StyledRenderer::new(write, &EmitForTest, ());
            pretty
                .render_raw(100, &mut renderer)
                .map_err(|_| std::fmt::Error)
        })?;

        assert_eq!(string, "[Red:hello]\n");

        Ok(())
    }

    #[test]
    fn prettyrs_example() -> Result<(), Box<dyn Error>> {
        let red = Structure::fragment(StyledString::str(
            "it-is-red",
            Style::default().fg(Color::Red),
        ));

        let blue = Structure::fragment(StyledString::str(
            "it-is-blue",
            Style::default().fg(Color::Blue),
        ));

        let bold = Structure::fragment(StyledString::str(
            "it-is-bold",
            Style::default().attr(Attribute::Bold),
        ));

        let structure = red
            .append(GAP())
            .append(blue)
            .append(GAP())
            .append(bold)
            .group();

        assert_eq!(
            render(&structure, &EmitForTest, 100)?,
            "[Red:it-is-red][normal: ][Blue:it-is-blue][normal: ][normal,bold:it-is-bold]"
        );

        assert_eq!(
            render(&structure, &EmitForTest, 5)?,
            "[Red:it-is-red]\n[Blue:it-is-blue]\n[normal,bold:it-is-bold]"
        );

        Ok(())
    }

    #[test]
    fn prettyrs_original() -> Result<(), Box<dyn Error>> {
        let arena = Arena::<'_, ()>::new();
        let red = arena.text("red");

        let blue = arena.text("blue");

        let bold = arena.text("bold");

        let intense = arena.text("intense");

        let red = red
            .append(arena.line())
            .append(blue)
            .append(arena.line())
            .append(bold)
            .append(arena.line())
            .append(intense)
            .group()
            .1;

        assert_eq!(
            &format!("{}", lazy_format!(|f| red.render_fmt(50, f))),
            "red blue bold intense"
        );

        assert_eq!(
            &format!("{}", lazy_format!(|f| red.render_fmt(5, f))),
            "red\nblue\nbold\nintense"
        );

        Ok(())
    }
}
