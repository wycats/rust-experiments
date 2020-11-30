use format::Display;

use crate::{string::intern::Intern, EmitBackendTrait, Fragment, Style};

pub struct Renderer<'write, 'intern> {
    ann: bool,
    write: &'write mut dyn std::io::Write,
    intern: &'intern Intern,
    backend: Box<dyn EmitBackendTrait + 'static>,
}

impl<'write, 'intern> Renderer<'write, 'intern> {
    #[allow(unused)]
    pub fn new(
        write: &'write mut dyn std::io::Write,
        intern: &'intern Intern,
        backend: impl EmitBackendTrait,
    ) -> Renderer<'write, 'intern> {
        Renderer {
            ann: false,
            intern,
            write,
            backend: Box::new(backend),
        }
    }
}

impl pretty::Render for Renderer<'_, '_> {
    type Error = std::fmt::Error;

    fn write_str(&mut self, s: &str) -> Result<usize, Self::Error> {
        if self.ann {
            self.ann = false;
        } else {
            let Self { write, backend, .. } = self;
            write!(
                write,
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

impl<'a> pretty::RenderAnnotated<'_, Fragment> for Renderer<'_, '_> {
    fn push_annotation(&mut self, annotation: &Fragment) -> Result<(), Self::Error> {
        self.ann = true;

        let Self {
            write,
            backend,
            intern,
            ..
        } = self;

        write!(
            write,
            "{}",
            Display(move |f| backend
                .emit(f, intern.get(annotation.id()), annotation.style())
                .map_err(|_| std::fmt::Error))
        )
        .map_err(|_| std::fmt::Error)?;

        Ok(())
    }

    fn pop_annotation(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
