use pretty::{DocAllocator, DocPtr};

use crate::{
    render::{RenderConfig, RenderState},
    EmitBackendTrait,
};

use super::{cow_mut::CowMut, renderer::Renderer, Doc, Styled, StyledArena};

#[allow(dead_code)]
pub struct RenderContext<'arena, 'writer> {
    arena: StyledArena<'arena>,
    writer: CowMut<'writer, dyn std::io::Write>,
}

impl<'arena, 'writer> RenderContext<'arena, 'writer> {
    #[allow(unused)]
    pub fn new(writer: &'writer mut dyn std::io::Write) -> RenderContext<'arena, 'writer> {
        RenderContext {
            arena: StyledArena::new(),
            writer: CowMut::Borrowed(writer),
        }
    }

    #[allow(unused)]
    pub fn render(
        &'arena mut self,
        doc: &'arena dyn Doc,
        backend: impl EmitBackendTrait + 'arena,
        config: RenderConfig,
    ) -> Result<(), std::fmt::Error> {
        // let string = Buf::collect_string(|writer| {
        let mut renderer = Renderer::new(self.writer.to_mut(), backend);
        let doc = doc.render(&self.arena, RenderState::top(config));
        doc.into_doc()
            .render_raw(config.column_size, &mut renderer)?;

        Ok(())
        // })?;
    }
}

impl<'a, 'writer> DocAllocator<'a, Styled<'a>> for RenderContext<'a, 'writer> {
    type Doc = pretty::RefDoc<'a, Styled<'a>>;

    fn alloc(&'a self, doc: pretty::Doc<'a, Self::Doc, Styled<'a>>) -> Self::Doc {
        self.arena.alloc(doc)
    }

    fn alloc_column_fn(
        &'a self,
        f: impl Fn(usize) -> Self::Doc + 'a,
    ) -> <Self::Doc as DocPtr<'a, Styled<'a>>>::ColumnFn {
        self.arena.alloc_column_fn(f)
    }

    fn alloc_width_fn(
        &'a self,
        f: impl Fn(isize) -> Self::Doc + 'a,
    ) -> <Self::Doc as DocPtr<'a, Styled<'a>>>::WidthFn {
        self.arena.alloc_width_fn(f)
    }
}
