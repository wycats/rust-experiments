use derive_new::new;
use pretty::DocAllocator;

use crate::{
    render::{Nesting, RenderState},
    BoxedDoc,
};

use super::{Doc, StyledArena, StyledDoc};

#[derive(Debug, new)]
pub struct Group {
    docs: Vec<BoxedDoc>,
}

impl Doc for Group {
    fn render<'ctx>(
        &'ctx self,
        ctx: &'ctx StyledArena<'ctx>,
        state: RenderState,
    ) -> StyledDoc<'ctx> {
        let mut list = ctx.nil();

        for doc in &self.docs {
            list = list.append(doc.render(ctx, state));
        }

        list.group()
    }
}

#[derive(Debug, new)]
pub struct DocList {
    docs: Vec<BoxedDoc>,
}

impl Doc for DocList {
    fn render<'ctx>(
        &'ctx self,
        ctx: &'ctx StyledArena<'ctx>,
        state: RenderState,
    ) -> StyledDoc<'ctx> {
        let mut list = ctx.nil();

        for doc in &self.docs {
            list = list.append(doc.render(ctx, state));
        }

        list
    }
}

#[macro_export]
macro_rules! nest {
    (=> $start_gap:expr ; $structure:expr ; $end_gap:expr) => {
        $crate::structure::compose::list::Nested::new(
            $crate::structure::render::Nesting::Configured(1),
            Box::new($structure),
            Box::new($start_gap),
            Box::new($end_gap),
        )
    };
}

#[derive(Debug, new)]
pub struct Nested {
    indent: Nesting,
    structure: Box<dyn Doc>,
    start_gap: Box<dyn Doc>,
    end_gap: Box<dyn Doc>,
}

impl Doc for Nested {
    fn render<'ctx>(
        &'ctx self,
        ctx: &'ctx StyledArena<'ctx>,
        state: RenderState,
    ) -> StyledDoc<'ctx> {
        let Self {
            indent,
            structure,
            start_gap,
            end_gap,
        } = self;

        ctx.nil().append(start_gap.render(ctx, state)).append(
            structure
                .render(ctx, state)
                .nest(state.size(*indent))
                .append(end_gap.render(ctx, state).group()),
        )
    }
}
