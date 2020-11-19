use pretty::DocAllocator;

use crate::Style;

use super::Styled;
use super::{StyledArena, StyledDoc};

#[macro_export]
macro_rules! list {
    ($($expr:expr),*) => {{
        let mut vec: Vec<Box<dyn $crate::structure::compose::Doc>> = vec![];

        $(
            vec.push(Box::new($expr));
        )*

        $crate::structure::compose::DocList::new(vec)
    }}
}

#[macro_export]
macro_rules! doc {
    ($name:ident { $($arg:ident : $arg_ty:ty),* } |$this:pat, $ctx:pat| $expr:expr) => {
        doc! { generate => $name {} {} { $($arg : $arg_ty),* } |$this, $ctx| $expr }
    };

    ($name:ident <$lt:tt> { $($arg:ident : $arg_ty:ty),* } |$this:pat, $ctx:pat| $expr:expr) => {
        doc! { generate => $name { <$lt> } { + $lt } { $($arg : $arg_ty),* } |$this, $ctx| $expr }
    };

    (generate => $name:ident { $($lt:tt)* } { $($plus:tt)* } { $($arg:ident : $arg_ty:ty),* } |$this:pat, $ctx:pat| $expr:expr) => {
        #[allow(unused)]
        pub fn $name$($lt)*($($arg : $arg_ty),*) -> impl $crate::structure::compose::Doc $($plus)* {
            struct Impl $($lt)* { $($arg: $arg_ty),* }

            impl $($lt)* $crate::structure::compose::Doc for Impl $($lt)* {
                fn render<'ctx>(&'ctx self, $ctx: &'ctx StyledArena<'ctx>) -> StyledDoc<'ctx> {
                    let $this = self;

                    $expr
                }
            }

            Impl { $($arg),* }
        }
    };
}

doc! {
    plain<'a> { string: &'a str }
    |plain, ctx| ctx.text(plain.string).annotate(Styled::plain(plain.string))
}

doc! {
    styled<'a> { string: &'a str, style: Style }
    |styled, ctx| ctx.text(styled.string).annotate(Styled::str(styled.string, styled.style))
}

#[cfg(test)]
mod tests {

    use console::Color;

    use crate::{
        emit::buf::Buf, prelude::TestResult, structure::compose::render_context::RenderContext,
        structure::compose::Doc, EmitForTest,
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

    fn render(text: &impl Doc) -> Result<String, std::fmt::Error> {
        Buf::collect_string(|writer| {
            let mut context = RenderContext::new(writer);
            context.render(text, EmitForTest, 80)?;

            Ok(())
        })
    }
}
