use derive_new::new;
use pretty::DocAllocator;

use crate::{BoxedDoc, NonemptyList};

use super::{Doc, StyledArena, StyledDoc};

#[derive(Debug, new)]
pub struct JoinList {
    delimiter: BoxedDoc,
    items: NonemptyList<BoxedDoc>,
    trailing: bool,
}

impl Doc for JoinList {
    fn render<'a>(
        &self,
        ctx: &'a StyledArena<'a>,
        state: crate::render::RenderState,
    ) -> StyledDoc<'a> {
        let mut list = ctx.nil();

        for item in self.items.iter() {
            let is_last = item.is_last();

            list = list.append(item.value().render(&ctx, state));

            if !is_last || self.trailing {
                list = list.append(self.delimiter.render(&ctx, state));
            }
        }

        list
    }
}
