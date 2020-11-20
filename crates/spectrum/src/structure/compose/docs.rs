use pretty::DocAllocator;

use crate::{render::RenderState, Style};

use super::{BoxedDoc, Doc, Styled};
use super::{StyledArena, StyledDoc};

#[macro_export]
macro_rules! list {
    ($($expr:expr),*) => {{
        $crate::structure::compose::DocList::new($crate::list_impl!($($expr),*))
    }}
}

#[macro_export]
macro_rules! group {
    ($($expr:expr),*) => {{
        $crate::structure::compose::Group::new($crate::list_impl!($($expr),*))
    }}
}

#[doc(hidden)]
#[macro_export]
macro_rules! list_impl {
    ($($expr:expr),*) => {{
        #[allow(unused)]
        use $crate::Doc;

        #[allow(unused_mut)]
        let mut vec: Vec<$crate::BoxedDoc> = vec![];

        $(
            vec.push($expr.boxed());
        )*

        vec
    }}
}

#[macro_export]
macro_rules! doc {
    ($name:ident as $struct_name:ident |$ctx:pat, $state:pat| $expr:expr) => {
        doc! { generate => $name as $struct_name lt = {} plus = {} struct = { ; } args = {} |_, $ctx, $state| $expr }
    };

    ($name:ident as $struct_name:ident { $($arg:ident : $arg_ty:ty),* } |$this:pat, $ctx:pat, $state:pat| $expr:expr) => {
        doc! { generate => $name as $struct_name lt = {} plus = {} struct = { { $($arg : $arg_ty),* } } args = { $($arg : $arg_ty),* } |$this, $ctx, $state| $expr }
    };

    ($name:ident as $struct_name:ident <$lt:tt> { $($arg:ident : $arg_ty:ty),* } |$this:pat, $ctx:pat, $state:pat| $expr:expr) => {
        doc! { generate => $name as $struct_name lt = { <$lt> } plus = { + $lt } struct = { { $($arg : $arg_ty),* } } args = { $($arg : $arg_ty),* } |$this, $ctx, $state| $expr }
    };

    (generate => $name:ident as $struct_name:ident lt = { $($lt:tt)* } plus = { $($plus:tt)* } struct = { $struct:tt } args = { $($arg:ident : $arg_ty:ty),* } |$this:pat, $ctx:pat, $state:pat| $expr:expr) => {
        #[derive(Debug)]
        pub struct $struct_name $($lt)* $struct

        impl $($lt)* $crate::structure::compose::Doc for $struct_name $($lt)* {
            fn render<'ctx>(&'ctx self, $ctx: &'ctx StyledArena<'ctx>, $state: $crate::render::RenderState) -> StyledDoc<'ctx> {
                let $this = self;

                $expr
            }
        }

        #[allow(unused)]
        pub fn $name$($lt)*($($arg : $arg_ty),*) -> impl $crate::structure::compose::Doc $($plus)* {
            $struct_name { $($arg),* }
        }
    };
}

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

#[macro_export]
macro_rules! either {
    (inline: $inline:expr, block: $block:expr) => {{
        use $crate::structure::compose::docs::either;
        use $crate::Doc;

        either($inline.boxed(), $block.boxed())
    }};
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
