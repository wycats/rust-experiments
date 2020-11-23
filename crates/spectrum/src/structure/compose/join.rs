use derive_new::new;
use pretty::DocAllocator;

use crate::NonemptyList;

use super::Doc;

#[derive(Debug, new)]
pub struct JoinList {
    delimiter: Box<dyn Doc>,
    items: NonemptyList<Box<dyn Doc>>,
    trailing: bool,
}

impl Doc for JoinList {
    fn render<'ctx>(
        &'ctx self,
        ctx: &'ctx super::StyledArena<'ctx>,
        state: crate::render::RenderState,
    ) -> super::StyledDoc<'ctx> {
        let mut list = ctx.nil();

        for item in self.items.iter() {
            let is_last = item.is_last();

            list = list.append(item.value().render(ctx, state));

            if !is_last || self.trailing {
                list = list.append(self.delimiter.render(ctx, state));
            }
        }

        list
    }
}
