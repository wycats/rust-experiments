use derive_new::new;
use pretty::DocAllocator;

use crate::{
    render::{Nesting, RenderState},
    BoxedDoc, Doc, GAP,
};

use super::{StyledArena, StyledDoc};

#[derive(Debug, new)]
pub struct Group {
    docs: Vec<BoxedDoc>,
}

impl Doc for Group {
    fn render<'a>(&self, ctx: &'a StyledArena<'a>, state: RenderState) -> StyledDoc<'a> {
        let mut list = ctx.nil();

        for doc in self.docs.iter() {
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
    fn render<'a>(&self, ctx: &'a StyledArena<'a>, state: RenderState) -> StyledDoc<'a> {
        let mut list = ctx.nil();

        for doc in self.docs.iter() {
            list = list.append(doc.render(ctx, state));
        }

        list
    }
}

#[derive(Debug, new)]
pub struct Nested {
    indent: Nesting,
    structure: BoxedDoc,
    start_gap: BoxedDoc,
    end_gap: BoxedDoc,
}

impl Nested {
    pub fn once(structure: impl Doc, start_gap: impl Doc, end_gap: impl Doc) -> Nested {
        Nested {
            indent: Nesting::Configured(1),
            structure: structure.boxed(),
            start_gap: start_gap.boxed(),
            end_gap: end_gap.boxed(),
        }
    }

    pub fn basic(structure: impl Doc) -> Nested {
        Nested {
            indent: Nesting::Configured(1),
            structure: structure.boxed(),
            start_gap: GAP().boxed(),
            end_gap: GAP().boxed(),
        }
    }
}

impl Doc for Nested {
    fn render<'a>(&self, ctx: &'a StyledArena<'a>, state: RenderState) -> StyledDoc<'a> {
        let Self {
            indent,
            structure,
            start_gap,
            end_gap,
        } = self;

        start_gap
            .render(ctx, state)
            .append(structure.render(ctx, state))
            .nest(state.size(*indent))
            .append(end_gap.render(ctx, state))
    }
}
