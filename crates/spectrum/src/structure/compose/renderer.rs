use crate::{EmitBackendTrait, Style};

use super::{cow_mut::CowMut, Styled};

pub struct Renderer<'ctx> {
    ann: bool,
    write: CowMut<'ctx, dyn std::io::Write>,
    backend: Box<dyn EmitBackendTrait + 'ctx>,
}

impl<'ctx> Renderer<'ctx> {
    #[allow(unused)]
    pub fn new(
        write: &'ctx mut dyn std::io::Write,
        backend: impl EmitBackendTrait + 'ctx,
    ) -> Renderer<'ctx> {
        Renderer {
            ann: false,
            write: CowMut::Borrowed(write),
            backend: Box::new(backend),
        }
    }
}

impl<'ctx> pretty::Render for Renderer<'ctx> {
    type Error = std::fmt::Error;

    fn write_str(&mut self, s: &str) -> Result<usize, Self::Error> {
        if self.ann {
            self.ann = false;
        } else {
            let Self { write, backend, .. } = self;
            write!(
                write.to_mut(),
                "{}",
                format::lazy_format!(|f| backend
                    .emit(f, s, Style::default())
                    .map_err(|_| std::fmt::Error))
            )
            .map_err(|_| std::fmt::Error)?;
        }

        Ok(s.len())
    }

    fn fail_doc(&self) -> Self::Error {
        std::fmt::Error
    }
}

impl<'ctx, 'a> pretty::RenderAnnotated<'a, Styled<'ctx>> for Renderer<'ctx> {
    fn push_annotation(&mut self, annotation: &'a Styled<'ctx>) -> Result<(), Self::Error> {
        self.ann = true;

        let Self { write, backend, .. } = self;
        let (str, style) = annotation.as_pair();

        write!(
            write.to_mut(),
            "{}",
            format::lazy_format!(|f| backend.emit(f, str, style).map_err(|_| std::fmt::Error))
        )
        .map_err(|_| std::fmt::Error)?;

        Ok(())
    }

    fn pop_annotation(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
