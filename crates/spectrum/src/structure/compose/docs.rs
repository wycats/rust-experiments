use derive_new::new;
use pretty::DocAllocator;

use crate::{render::RenderState, string::intern::Intern, string::intern::StringId, Style};

use super::{BoxedDoc, Doc, Fragment};
use super::{StyledArena, StyledDoc};

#[derive(Debug)]
pub struct Plain {
    string: StringId,
}

impl Doc for Plain {
    fn render<'a>(&self, ctx: &'a StyledArena<'a>, _state: RenderState) -> StyledDoc<'a> {
        ctx.text(ctx.get_str(self.string))
            .annotate(Fragment::plain(self.string))
    }
}

pub fn plain(string: impl Into<StringId>) -> Plain {
    Plain {
        string: string.into(),
    }
}

// document! {
//     plain as Plain<'a> { string: &'a str }
//     |plain, ctx, _| ctx.text(plain.string).annotate(Styled::plain(plain.string))
// }

pub trait InternedStyledFragment {
    fn intern(self, intern: &mut Intern) -> StyledFragment;
}

#[derive(Debug, new)]
pub struct StyledFragment {
    string: StringId,
    style: Style,
}

impl Doc for StyledFragment {
    fn render<'a>(&self, ctx: &'a StyledArena<'a>, _state: RenderState) -> StyledDoc<'a> {
        ctx.text(ctx.get_str(self.string))
            .annotate(Fragment::new(self.string, self.style))
    }
}

pub fn styled(string: impl Into<StringId>, style: Style) -> StyledFragment {
    StyledFragment {
        string: string.into(),
        style,
    }
}

// document! {
//     styled as StyledFragment<'a> { string: &'a str, style: Style }
//     |styled, ctx, _| ctx.text(styled.string).annotate(Styled::str(styled.string, styled.style))
// }

#[derive(Debug)]
struct HardLine;

impl Doc for HardLine {
    fn render<'a>(&self, ctx: &'a StyledArena<'a>, _state: RenderState) -> StyledDoc<'a> {
        ctx.hardline()
    }
}

#[derive(Debug)]
pub struct Either {
    inline: BoxedDoc,
    block: BoxedDoc,
}

impl Doc for Either {
    fn render<'a>(&self, ctx: &'a StyledArena<'a>, state: RenderState) -> StyledDoc<'a> {
        self.block
            .render(ctx, state)
            .flat_alt(self.inline.render(ctx, state))
    }
}

pub fn either(inline: BoxedDoc, block: BoxedDoc) -> Either {
    Either { inline, block }
}

impl Doc for Box<dyn Doc> {
    fn render<'a>(&self, ctx: &'a StyledArena<'a>, state: RenderState) -> StyledDoc<'a> {
        Doc::render(&**self, ctx, state)
    }
}

#[derive(Debug)]
pub struct Empty;

impl Doc for Empty {
    fn render<'a>(&self, ctx: &'a StyledArena<'a>, _state: RenderState) -> StyledDoc<'a> {
        ctx.nil()
    }
}

pub fn empty() -> Empty {
    Empty
}

// document! {
//     empty as Empty
//     |ctx, _| ctx.nil()
// }

/// A GAP is a space when laid out inline, or a hard line when laid out as a block
pub fn GAP() -> BoxedDoc {
    either!(inline: plain(" "), block: HardLine).boxed()
}

/// A GAP_HINT is a newline if the next element would overflow the page size, or a space otherwise.
pub fn GAP_HINT() -> BoxedDoc {
    group![GAP()].boxed()
}

/// A BOUNDARY is nothing when laid out inline, or a hard line when laid out as a block
pub fn BOUNDARY() -> BoxedDoc {
    either!(inline: empty(), block: HardLine).boxed()
}

/// A BOUNDARY_HINT is a newline if the next element would overflow the page size, or nothing
/// otherwise.
pub fn BOUNDARY_HINT() -> BoxedDoc {
    group![either!(inline: empty(), block: HardLine)].boxed()
}

#[cfg(test)]
mod tests {

    use console::Color;

    use crate::{
        emit::buf::Buf, prelude::test::*, render::RenderConfig, string::intern::Intern,
        structure::compose::render_context::RenderContext, structure::compose::Doc, EmitForTest,
    };

    use super::*;

    #[test]
    fn test_plain() -> TestResult {
        struct Stringy {
            string: &'static str,
        }

        let stringy = Stringy { string: "Hi niko!" };

        let text = plain(stringy.string);

        assert_eq!(render(&text)?, "[normal:Hi niko!]");

        Ok(())
    }

    #[test]
    fn test_styled() -> TestResult {
        let text = styled("Hi niko!", Color::Red.into());

        assert_eq!(render(&text)?, "[Red:Hi niko!]");

        Ok(())
    }

    #[test]
    fn test_list() -> TestResult {
        let doc = list![styled("Hi niko!", Color::Red.into()), plain("Bye!")];

        assert_eq!(render(&doc)?, "[Red:Hi niko!][normal:Bye!]");

        Ok(())
    }

    #[test]
    fn test_group() -> TestResult {
        let doc = group![styled("Hi niko!", Color::Red.into()), plain("Bye!")];

        assert_eq!(render(&doc)?, "[Red:Hi niko!][normal:Bye!]");

        Ok(())
    }

    fn render(text: &impl Doc) -> Result<String, std::fmt::Error> {
        Buf::collect_string(|writer| {
            let intern = Intern::new();
            let arena = StyledArena::new(&intern);
            let mut context = RenderContext::new(arena);
            context.render(text, EmitForTest, writer, RenderConfig::width(80))?;

            Ok(())
        })
    }
}
