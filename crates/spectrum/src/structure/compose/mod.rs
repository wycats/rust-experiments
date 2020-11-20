/*!
 * A composable structure is a function from a document context to a piece of a pretty::Doc
 */

#[macro_use]
pub mod frag;

#[macro_use]
pub mod docs;

#[macro_use]
pub mod list;

#[macro_use]
mod join;

mod cow_mut;
pub mod render_context;
mod renderer;

use std::fmt::Debug;

use crate::{render::RenderState, Style};

pub use self::list::{DocList, Group};
pub use docs::*;
use pretty::DocAllocator;

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

pub trait Doc: Debug {
    fn render<'ctx>(
        &'ctx self,
        ctx: &'ctx StyledArena<'ctx>,
        state: RenderState,
    ) -> StyledDoc<'ctx>;

    fn boxed(self) -> BoxedDoc
    where
        Self: Sized + 'static,
    {
        BoxedDoc {
            doc: Box::new(self),
        }
    }
}

#[derive(Debug)]
pub struct BoxedDoc {
    doc: Box<dyn Doc + 'static>,
}

impl Doc for BoxedDoc {
    fn render<'ctx>(
        &'ctx self,
        ctx: &'ctx StyledArena<'ctx>,
        state: RenderState,
    ) -> StyledDoc<'ctx> {
        self.doc.render(ctx, state)
    }
}

impl Doc for &'static str {
    fn render<'ctx>(
        &'ctx self,
        ctx: &'ctx StyledArena<'ctx>,
        _state: RenderState,
    ) -> StyledDoc<'ctx> {
        ctx.text(*self).annotate(Styled::plain(self))
    }
}

#[cfg(test)]
mod tests {
    use crate::{emit::buf::Buf, prelude::test::*, render::RenderConfig, EmitPlain};

    use super::docs::empty;
    use super::{render_context::RenderContext, Doc, GAP};
    use textwrap::dedent;

    #[test]
    fn compose_smoke() -> TestResult {
        let expected_block = strip(
            r#"
            function HelloWorld({
              greeting = "hello",
              greeted = '"World"',
              silent = false,
              onMouseOver,
            }) {}
        "#,
        );

        let expected_inline = "function HelloWorld({ greeting = \"hello\", greeted = '\"World\"', silent = false, onMouseOver }) {}\n";

        let doc = list![
            "function ",
            "HelloWorld",
            group![
                "(",
                "{",
                group![
                    group!["greeting", " = ", r#""hello""#],
                    ",",
                    GAP(),
                    group!["greeted", " = ", r#"'"World"'"#],
                    ",",
                    GAP(),
                    group!["silent", " = ", "false"],
                    ",",
                    GAP(),
                    group!["onMouseOver"],
                    either! { inline: empty(), block: "," }
                ],
                "}",
                ")"
            ]
        ];

        assert_eq!(render(&doc, 80)?, expected_block);
        assert_eq!(render(&doc, 96)?, expected_inline);

        Ok(())
    }

    fn render(text: &impl Doc, page_size: usize) -> Result<String, std::fmt::Error> {
        Buf::collect_string(|writer| {
            let mut context = RenderContext::new(writer);
            context.render(text, EmitPlain, RenderConfig::width(page_size))?;

            Ok(())
        })
    }

    fn strip(input: &str) -> String {
        let lines: Vec<&str> = input.split('\n').collect();
        let string = lines[1..lines.len()].to_vec().join("\n");
        dedent(&string)
    }
}
