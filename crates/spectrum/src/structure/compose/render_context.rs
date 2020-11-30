use derive_new::new;

use pretty::{DocAllocator, DocPtr};

use crate::{
    render::{RenderConfig, RenderState},
    string::intern::Intern,
    EmitBackendTrait,
};

use super::{renderer::Renderer, Doc, Fragment, StyledArena};

#[allow(dead_code)]
#[derive(new)]
pub struct RenderContext<'arena> {
    arena: StyledArena<'arena>,
}

impl<'arena> RenderContext<'arena> {
    #[allow(unused)]
    pub fn render(
        &'arena mut self,
        doc: &dyn Doc,
        backend: impl EmitBackendTrait,
        writer: &mut dyn std::io::Write,
        config: RenderConfig,
    ) -> Result<(), std::fmt::Error> {
        // let string = Buf::collect_string(|writer| {
        let intern = Intern::default();
        let mut renderer = Renderer::new(writer, &intern, backend);
        let doc = doc.render(&self.arena, RenderState::top(config));
        doc.into_doc()
            .render_raw(config.column_size, &mut renderer)?;

        Ok(())
    }
}

impl<'a> DocAllocator<'a, Fragment> for RenderContext<'a> {
    type Doc = pretty::RefDoc<'a, Fragment>;

    fn alloc(&'a self, doc: pretty::Doc<'a, Self::Doc, Fragment>) -> Self::Doc {
        self.arena.alloc(doc)
    }

    fn alloc_column_fn(
        &'a self,
        f: impl Fn(usize) -> Self::Doc + 'a,
    ) -> <Self::Doc as DocPtr<'a, Fragment>>::ColumnFn {
        self.arena.alloc_column_fn(f)
    }

    fn alloc_width_fn(
        &'a self,
        f: impl Fn(isize) -> Self::Doc + 'a,
    ) -> <Self::Doc as DocPtr<'a, Fragment>>::WidthFn {
        self.arena.alloc_width_fn(f)
    }
}
