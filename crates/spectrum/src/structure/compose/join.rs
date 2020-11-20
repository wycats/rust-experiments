use derive_new::new;
use pretty::DocAllocator;

use crate::NonemptyList;

use super::Doc;

#[macro_export]
macro_rules! join {
    ($items:expr, with $delimiter:expr, trail) => {
        join!(generate $items, with $delimiter, trail = true)
    };

    ($items:expr, with $delimiter:expr) => {
        join!(generate $items, with $delimiter, trail = false)
    };

    (generate $items:expr, with $delimiter:expr, trail = $trail:tt) => {
        $crate::structure::compose::join::JoinList::new(
            Box::new($delimiter),
            $crate::structure::nonempty::NonemptyList::new($items),
            $trail,
        )
    };
}

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
