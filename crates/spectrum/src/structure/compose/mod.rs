/*!
 * A composable structure is a function from a document context to a piece of a pretty::Doc
 */

mod cow_mut;
mod docs;
mod render_context;
mod renderer;

use derive_new::new;
use pretty::DocAllocator;

use crate::Style;

pub enum Styled<'ctx> {
    Fragment { fragment: &'ctx str, style: Style },
}

impl<'ctx> Styled<'ctx> {
    pub fn str(fragment: &'ctx str, style: Style) -> Styled<'ctx> {
        Styled::Fragment { fragment, style }
    }

    pub fn plain(fragment: &'ctx str) -> Styled<'ctx> {
        Styled::Fragment {
            fragment,
            style: Style::default(),
        }
    }

    pub fn as_pair(&self) -> (&'ctx str, Style) {
        match *self {
            Styled::Fragment { fragment, style } => (fragment, style),
        }
    }
}

pub type StyledArena<'ctx> = pretty::Arena<'ctx, Styled<'ctx>>;
pub type StyledDoc<'ctx> = pretty::DocBuilder<'ctx, StyledArena<'ctx>, Styled<'ctx>>;

pub trait Doc {
    fn render<'ctx>(&'ctx self, ctx: &'ctx StyledArena<'ctx>) -> StyledDoc<'ctx>;
}

#[derive(new)]
pub struct DocList {
    docs: Vec<Box<dyn Doc>>,
}

impl Doc for DocList {
    fn render<'ctx>(&'ctx self, ctx: &'ctx StyledArena<'ctx>) -> StyledDoc<'ctx> {
        let mut list = ctx.nil();

        for doc in &self.docs {
            list = list.append(doc.render(ctx));
        }

        list
    }
}
