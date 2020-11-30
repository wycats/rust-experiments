/*!
 * A composable structure is a function from a document context to a piece of a pretty::Doc
 */

#[macro_use]
pub mod macros;

#[macro_use]
pub mod frag;

#[macro_use]
pub mod docs;

#[macro_use]
pub mod list;

#[macro_use]
mod join;

pub mod render_context;
mod renderer;
mod subdoc;

use derive_new::new;
use std::fmt::Debug;

use crate::string::intern::{DerefInternedString, Intern, StringId};
use crate::{render::RenderState, Style};

pub use self::list::{DocList, Group};
pub use docs::*;
use pretty::{DocAllocator, DocPtr, RefDoc};

#[derive(Debug, Copy, Clone, new)]
pub struct Fragment {
    fragment: StringId,
    style: Style,
}

impl DerefInternedString for Fragment {
    fn fmt_interned(&self, f: &mut std::fmt::Formatter<'_>, intern: &Intern) -> std::fmt::Result {
        write!(f, "{}", intern.get(self.fragment))
    }
}

impl Fragment {
    pub(crate) fn id(self) -> StringId {
        self.fragment
    }

    pub fn plain(id: StringId) -> Fragment {
        Fragment::new(id, Style::default())
    }

    pub fn style(self) -> Style {
        self.style
    }
}

impl Doc for Fragment {
    fn render<'a>(&self, ctx: &'a StyledArena<'a>, _state: RenderState) -> StyledDoc<'a> {
        let str = ctx.get_str(self.fragment);
        // let doc = str.annotate(*self);

        let text = ctx.text(str);
        text.annotate(*self)
        // ctx.clone().text(ctx.get_str(self.fragment)).annotate(*self)
    }
}

pub struct StyledArenaInternal<'arena> {
    arena: pretty::Arena<'arena, Fragment>,
    intern: &'arena Intern,
}

pub struct StyledArena<'arena> {
    arena: StyledArenaInternal<'arena>,
}

impl<'arena> StyledArena<'arena> {
    pub fn new(intern: &'arena Intern) -> StyledArena<'arena> {
        StyledArena {
            arena: StyledArenaInternal {
                arena: pretty::Arena::new(),
                intern,
            },
        }
    }

    pub fn get_str(&self, id: StringId) -> &str {
        self.arena.intern.get(id)
    }
}

impl<'a> DocAllocator<'a, Fragment> for StyledArena<'a> {
    type Doc = RefDoc<'a, Fragment>;

    fn alloc(&'a self, doc: pretty::Doc<'a, Self::Doc, Fragment>) -> Self::Doc {
        self.arena.arena.alloc(doc)
    }

    fn alloc_column_fn(
        &'a self,
        f: impl Fn(usize) -> Self::Doc + 'a,
    ) -> <Self::Doc as DocPtr<'a, Fragment>>::ColumnFn {
        self.arena.arena.alloc_column_fn(f)
    }

    fn alloc_width_fn(
        &'a self,
        f: impl Fn(isize) -> Self::Doc + 'a,
    ) -> <Self::Doc as DocPtr<'a, Fragment>>::WidthFn {
        self.arena.arena.alloc_width_fn(f)
    }
}

pub type StyledDoc<'a> = pretty::DocBuilder<'a, StyledArena<'a>, Fragment>;

pub trait Doc: Debug + 'static {
    fn render<'a>(&self, ctx: &'a StyledArena<'a>, state: RenderState) -> StyledDoc<'a>;

    fn boxed(self) -> BoxedDoc
    where
        Self: Sized,
    {
        BoxedDoc {
            doc: Box::new(self),
        }
    }
}

#[derive(Debug)]
pub struct BoxedDoc {
    doc: Box<dyn Doc>,
}

pub trait InternedBoxedDoc {
    fn intern(self, intern: &mut Intern) -> BoxedDoc;
}

impl Doc for BoxedDoc {
    fn render<'a>(&self, ctx: &'a StyledArena<'a>, state: RenderState) -> StyledDoc<'a> {
        self.doc.render(ctx, state)
    }
}

impl Doc for &'static str {
    fn render<'a>(&self, ctx: &'a StyledArena<'a>, _state: RenderState) -> StyledDoc<'a> {
        let text = StringId::Literal(self);

        ctx.text(*self).annotate(Fragment::plain(text))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        emit::buf::Buf, prelude::test::*, render::RenderConfig, string::intern::Intern, EmitPlain,
    };

    use super::{docs::empty, StyledArena};
    use super::{render_context::RenderContext, Doc, GAP};
    use textwrap::dedent;

    #[test]
    fn compose_smoke_spectrum() -> TestResult {
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

        let expected_inline = "function HelloWorld({ greeting = \"hello\", greeted = '\"World\"', silent = false, onMouseOver }) {}";

        let doc = list![
            "function ",
            "HelloWorld",
            group![
                "(",
                "{",
                nest![
                    {
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
                    }
                    before = GAP();
                    after = GAP();
                ],
                "}",
                ")",
                " ",
                "{}"
            ]
        ];

        assert_eq!(render(&doc, 80)?, expected_block);
        assert_eq!(render(&doc, 96)?, expected_inline);

        Ok(())
    }

    fn render(text: &impl Doc, page_size: usize) -> Result<String, std::fmt::Error> {
        Buf::collect_string(|writer| {
            let intern = Intern::new();
            let arena = StyledArena::new(&intern);
            let mut context = RenderContext::new(arena);
            context.render(text, EmitPlain, writer, RenderConfig::width(page_size))?;

            Ok(())
        })
    }

    fn strip(input: &str) -> String {
        let lines: Vec<&str> = input.split('\n').collect();
        let string = lines[1..lines.len() - 1].to_vec().join("\n");
        dedent(&string)
    }
}
