use derive_new::new;

use crate::{
    string::copy_string::StringContext, structure::render::Render, HighLevel, Primitive, Structure,
};

#[derive(Debug, new)]
pub struct NestedStructure<Ctx>
where
    Ctx: StringContext,
{
    prefix: Structure<Ctx>,
    postfix: Structure<Ctx>,
    body: Structure<Ctx>,
}

impl<Ctx> Clone for NestedStructure<Ctx>
where
    Ctx: StringContext,
{
    fn clone(&self) -> Self {
        NestedStructure {
            prefix: self.prefix.clone(),
            postfix: self.postfix.clone(),
            body: self.body.clone(),
        }
    }
}

impl<Ctx> Render<Ctx> for NestedStructure<Ctx>
where
    Ctx: StringContext,
{
    fn into_primitive(self, ctx: &mut Ctx, recursive: bool) -> Primitive<Ctx> {
        let prefix = self.prefix.into_primitive(ctx, recursive);
        let postfix = self.postfix.into_primitive(ctx, recursive);
        let body = self.body.into_primitive(ctx, recursive);

        Primitive::Empty
            .append(prefix)
            .append(Primitive::Empty.append(body).nest())
            .append(postfix)
    }
}

#[allow(non_snake_case)]
pub fn Nested<Ctx>(
    prefix: impl Into<Structure<Ctx>>,
    body: impl Into<Structure<Ctx>>,
    postfix: impl Into<Structure<Ctx>>,
) -> Structure<Ctx>
where
    Ctx: StringContext,
{
    Structure::HighLevel(HighLevel::Nested(Box::new(NestedStructure {
        prefix: prefix.into(),
        postfix: postfix.into(),
        body: body.into(),
    })))
}

#[cfg(test)]
mod tests {
    use crate::{string::copy_string::SimpleContext, structure::prelude::*, Style, GAP, GAP_HINT};
    use std::error::Error;

    use console::{Attribute, Color};

    use crate::{structure::test::render, EmitForTest};

    use super::*;

    fn frag(s: &'static str, style: impl Into<Style>) -> Structure<SimpleContext> {
        Structure::fragment(SimpleContext.styled(s, style))
    }

    #[test]
    fn high_level_nested() -> Result<(), Box<dyn Error>> {
        let red = frag("it-is-red", Color::Red);
        let blue = frag("it-is-blue", Color::Blue);
        let bold = frag("it-is-bold", Attribute::Bold);

        let structure = Nested(
            SimpleContext.plain("("),
            vec![red, blue, bold].join(GAP()),
            SimpleContext.plain(")"),
        );

        assert_eq!(
            render(&structure, &EmitForTest, 100)?,
            "[normal:(][Red:it-is-red][normal: ][Blue:it-is-blue][normal: ][normal,bold:it-is-bold][normal:)]","100 wide"
        );

        assert_eq!(
            render(&structure, &EmitForTest, 10)?,
            "[normal:(]\n  [Red:it-is-red]\n  [Blue:it-is-blue]\n  [normal,bold:it-is-bold]\n[normal:)]", "10 wide"
        );

        assert_eq!(
            render(&structure, &EmitForTest, 25)?,
            "[normal:(]\n  [Red:it-is-red]\n  [Blue:it-is-blue]\n  [normal,bold:it-is-bold]\n[normal:)]", "25 wide"
        );

        Ok(())
    }

    #[test]
    fn high_level_nested_with_break_hints() -> Result<(), Box<dyn Error>> {
        let red = frag("it-is-red", Color::Red);
        let blue = frag("it-is-blue", Color::Blue);
        let bold = frag("it-is-bold", Attribute::Bold);

        let structure = Nested(
            SimpleContext.plain("("),
            vec![red, blue, bold].join(GAP_HINT()),
            SimpleContext.plain(")"),
        );

        assert_eq!(
            render(&structure, &EmitForTest, 100)?,
            "[normal:(][Red:it-is-red][normal: ][Blue:it-is-blue][normal: ][normal,bold:it-is-bold][normal:)]"
        );

        assert_eq!(
            render(&structure, &EmitForTest, 10)?,
            "[normal:(]\n  [Red:it-is-red]\n  [Blue:it-is-blue]\n  [normal,bold:it-is-bold]\n[normal:)]"
        );

        assert_eq!(
            render(&structure, &EmitForTest, 25)?,
            "[normal:(]\n  [Red:it-is-red][normal: ][Blue:it-is-blue]\n  [normal,bold:it-is-bold]\n[normal:)]"
        );

        Ok(())
    }

    #[test]
    fn regular_nest_with_gap_hint() -> TestResult {
        let red = frag("it-is-red", Color::Red);
        let blue = frag("it-is-blue", Color::Blue);
        let bold = frag("it-is-bold", Attribute::Bold);

        let doc: Structure<SimpleContext> = Doc(SimpleContext.plain("("))
            .append_group(vec![red, blue, bold].join(GAP_HINT()).nest())
            .append(SimpleContext.plain(")"));

        assert_eq!(
            render(&doc, &EmitForTest, 100)?,
            "[normal:(][Red:it-is-red][normal: ][Blue:it-is-blue][normal: ][normal,bold:it-is-bold][normal:)]"
        );

        assert_eq!(
            render(&doc, &EmitForTest, 10)?,
            "[normal:(]\n  [Red:it-is-red]\n  [Blue:it-is-blue]\n  [normal,bold:it-is-bold]\n[normal:)]"
        );

        assert_eq!(
            render(&doc, &EmitForTest, 25)?,
            "[normal:(]\n  [Red:it-is-red][normal: ][Blue:it-is-blue]\n  [normal,bold:it-is-bold]\n[normal:)]"
        );

        Ok(())
    }

    #[test]
    fn custom_nest_with_gap_hint() -> TestResult {
        let red = frag("it-is-red", Color::Red);
        let blue = frag("it-is-blue", Color::Blue);
        let bold = frag("it-is-bold", Attribute::Bold);

        let doc = Doc(SimpleContext.plain("("))
            .append_group(
                vec![red, blue, bold]
                    .join(GAP_HINT())
                    // only make a newline at first if it's already necessary, but if any newlines
                    // were inserted, make a newline at the end
                    .wrapping_nest(BOUNDARY_HINT(), BOUNDARY()),
            )
            .append(SimpleContext.plain(")"));

        assert_eq!(
            render(&doc, &EmitForTest, 100)?,
            "[normal:(][Red:it-is-red][normal: ][Blue:it-is-blue][normal: ][normal,bold:it-is-bold][normal:)]"
        );

        assert_eq!(
            render(&doc, &EmitForTest, 10)?,
            "[normal:(][Red:it-is-red]\n  [Blue:it-is-blue]\n  [normal,bold:it-is-bold]\n[normal:)]"
        );

        assert_eq!(
            render(&doc, &EmitForTest, 25)?,
            "[normal:(][Red:it-is-red][normal: ][Blue:it-is-blue]\n  [normal,bold:it-is-bold]\n[normal:)]"
        );

        Ok(())
    }
}
