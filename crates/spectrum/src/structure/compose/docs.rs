use pretty::DocAllocator;

use crate::{render::RenderState, Style};

use super::{BoxedDoc, Doc, Styled};
use super::{StyledArena, StyledDoc};

doc! {
    plain as Plain<'a> { string: &'a str }
    |plain, ctx, _| ctx.text(plain.string).annotate(Styled::plain(plain.string))
}

doc! {
    styled as StyledFragment<'a> { string: &'a str, style: Style }
    |styled, ctx, _| ctx.text(styled.string).annotate(Styled::str(styled.string, styled.style))
}

#[derive(Debug)]
struct HardLine;

impl Doc for HardLine {
    fn render<'ctx>(
        &'ctx self,
        ctx: &'ctx StyledArena<'ctx>,
        _state: RenderState,
    ) -> StyledDoc<'ctx> {
        ctx.hardline()
    }
}

doc! {
    either as Either { inline: BoxedDoc, block: BoxedDoc }
    |either, ctx, state| either.block.render(ctx, state).flat_alt(either.inline.render(ctx, state))
}

impl Doc for Box<dyn Doc> {
    fn render<'ctx>(
        &'ctx self,
        ctx: &'ctx StyledArena<'ctx>,
        state: RenderState,
    ) -> StyledDoc<'ctx> {
        Doc::render(&**self, ctx, state)
    }
}

doc! {
    empty as Empty
    |ctx, _| ctx.nil()
}

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
        emit::buf::Buf, prelude::test::*, render::RenderConfig,
        structure::compose::render_context::RenderContext, structure::compose::Doc, EmitForTest,
    };

    use super::*;

    #[test]
    fn test_plain() -> TestResult {
        struct Stringy {
            string: String,
        }

        let stringy = Stringy {
            string: "Hi niko!".to_string(),
        };

        let text = plain(&stringy.string);

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
            let mut context = RenderContext::new(writer);
            context.render(text, EmitForTest, RenderConfig::width(80))?;

            Ok(())
        })
    }
}
