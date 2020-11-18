use std::{
    error::Error,
    io::{self, stdout},
    marker::PhantomData,
};

use pretty::{Render, RenderAnnotated};

use crate::{string::copy_string::StringContext, EmitBackendTrait, StyledFragment};

enum CowMut<'a, T>
where
    T: ?Sized,
{
    Owned(Box<T>),
    Borrowed(&'a mut T),
}

impl<'a, T> CowMut<'a, T>
where
    T: ?Sized,
{
    fn to_mut(&mut self) -> &mut T {
        match self {
            CowMut::Owned(owned) => owned,
            CowMut::Borrowed(borrowed) => borrowed,
        }
    }
}

pub struct StyledRenderer<'borrow, 'backend, 'write, 'ctx, Ctx>
where
    Ctx: StringContext<'ctx> + 'ctx,
    'backend: 'borrow,
    'write: 'borrow,
    'ctx: 'borrow,
{
    write: CowMut<'borrow, dyn io::Write + 'write>,
    backend: &'borrow (dyn EmitBackendTrait + 'backend),
    ctx: &'borrow mut Ctx,
    ctx_lt: PhantomData<&'ctx ()>,
    /// remember whether we saw an annotation, which means we don't need to emit the string when
    /// write_str is called.
    ann: bool,
}

impl<'borrow, 'backend, 'write, 'ctx, Ctx> StyledRenderer<'borrow, 'backend, 'write, 'ctx, Ctx>
where
    Ctx: StringContext<'ctx> + 'ctx,
    'backend: 'borrow,
    'write: 'borrow,
    'ctx: 'borrow,
{
    #[allow(unused)]
    pub fn new(
        write: &'borrow mut (dyn io::Write + 'write),
        backend: &'borrow (dyn EmitBackendTrait + 'backend),
        context: &'borrow mut Ctx,
    ) -> StyledRenderer<'borrow, 'backend, 'write, 'ctx, Ctx> {
        StyledRenderer {
            write: CowMut::Borrowed(write),
            backend,
            ann: false,
            ctx: context,
            ctx_lt: PhantomData,
        }
    }

    #[allow(unused)]
    pub fn stdout(
        backend: &'borrow (dyn EmitBackendTrait + 'backend),
        context: &'borrow mut Ctx,
    ) -> StyledRenderer<'borrow, 'backend, 'static, 'ctx, Ctx> {
        StyledRenderer {
            write: CowMut::Owned(Box::new(stdout())),
            backend,
            ann: false,
            ctx: context,
            ctx_lt: PhantomData,
        }
    }
}

impl<'borrow, 'backend, 'write, 'ctx, Ctx> Render
    for StyledRenderer<'borrow, 'backend, 'write, 'ctx, Ctx>
where
    Ctx: StringContext<'ctx> + 'ctx,
    'backend: 'borrow,
    'write: 'borrow,
    'ctx: 'borrow,
{
    type Error = Box<dyn Error>;

    fn write_str(&mut self, s: &str) -> Result<usize, Self::Error> {
        if self.ann {
            self.ann = false;
        } else {
            write!(self.write.to_mut(), "{}", s)?;
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

impl<'borrow, 'backend, 'write, 'ctx, 'inner, Ctx>
    RenderAnnotated<'inner, StyledFragment<'ctx, Ctx>>
    for StyledRenderer<'borrow, 'backend, 'write, 'ctx, Ctx>
where
    Ctx: StringContext<'ctx> + 'ctx,
    'backend: 'borrow,
    'write: 'borrow,
    'ctx: 'borrow,
{
    fn push_annotation(
        &mut self,
        annotation: &'inner StyledFragment<'ctx, Ctx>,
    ) -> Result<(), Self::Error> {
        self.ann = true;
        Ok(annotation.emit_into_with(self.write.to_mut(), self.backend, &self.ctx)?)
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
    use crate::{
        emit::buf::Buf, structure::test::render, structure::Structure, EmitForTest, EmitPlain, GAP,
    };
    use crate::{render::Render, string::copy_string::SimpleContext};

    #[test]
    fn basic_render() -> Result<(), Box<dyn Error>> {
        let structure: Structure<SimpleContext> =
            Structure::fragment(SimpleContext::styled("hello", Color::Red))
                .append(Structure::hardline());

        let string = Buf::collect_string(move |write| {
            let pretty = structure.render();
            let ctx = &mut SimpleContext;
            let mut renderer = StyledRenderer::new(write, &EmitPlain, ctx);
            pretty
                .render_raw(100, &mut renderer)
                .map_err(|_| std::fmt::Error)
        })?;

        assert_eq!(string, "hello\n");

        Ok(())
    }

    #[test]
    fn colored_render() -> Result<(), Box<dyn Error>> {
        let structure: Structure<SimpleContext> =
            Structure::fragment(SimpleContext::styled("hello", Color::Red))
                .append(Structure::hardline());

        let pretty = structure.render();

        eprintln!("{:#?}", pretty);

        let string = Buf::collect_string(|write| {
            let ctx = &mut SimpleContext;
            let mut renderer = StyledRenderer::new(write, &EmitForTest, ctx);
            pretty
                .render_raw(100, &mut renderer)
                .map_err(|_| std::fmt::Error)
        })?;

        assert_eq!(string, "[Red:hello]\n");

        Ok(())
    }

    #[test]
    fn prettyrs_example() -> Result<(), Box<dyn Error>> {
        let red = Structure::fragment(SimpleContext::styled("it-is-red", Color::Red));

        let blue = Structure::fragment(SimpleContext::styled("it-is-blue", Color::Blue));

        let bold = Structure::fragment(SimpleContext::styled("it-is-bold", Attribute::Bold));

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
