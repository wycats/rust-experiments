use derive_new::new;

use super::{Primitive, Render, Structure};

use crate::{string::copy_string::StringContext, structure::HighLevel, NonemptyList};

pub trait JoinExt<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    fn join(self, delimiter: impl Into<Structure<'a, Ctx>>) -> Structure<'a, Ctx>;
    fn join_trailing(self, delimiter: impl Into<Structure<'a, Ctx>>) -> Structure<'a, Ctx>;
}

impl<'a, Ctx> JoinExt<'a, Ctx> for Vec<Structure<'a, Ctx>>
where
    Ctx: StringContext<'a>,
{
    fn join(self, delimiter: impl Into<Structure<'a, Ctx>>) -> Structure<'a, Ctx> {
        Structure::HighLevel(HighLevel::DelimitedList(Box::new(JoinList {
            delimiter: delimiter.into(),
            items: self.into(),
            trailing: false,
        })))
    }

    fn join_trailing(self, delimiter: impl Into<Structure<'a, Ctx>>) -> Structure<'a, Ctx> {
        Structure::HighLevel(HighLevel::DelimitedList(Box::new(JoinList {
            delimiter: delimiter.into(),
            items: self.into(),
            trailing: true,
        })))
    }
}

#[derive(Debug, new)]
pub struct JoinList<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    delimiter: Structure<'a, Ctx>,
    items: NonemptyList<Structure<'a, Ctx>>,
    trailing: bool,
}

impl<'a, Ctx> Clone for JoinList<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    fn clone(&self) -> Self {
        JoinList {
            delimiter: self.delimiter.clone(),
            items: self.items.clone(),
            trailing: self.trailing,
        }
    }
}

impl<'a, Ctx> Render<'a, Ctx> for JoinList<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    fn into_primitive(self, ctx: &mut Ctx, recursive: bool) -> Primitive<'a, Ctx> {
        let mut list = Primitive::Empty;

        let Self {
            delimiter,
            items,
            trailing,
        } = self;

        for item in items.drain() {
            let last = item.is_last();
            let value = item.value();

            if recursive {
                list = list.append(Structure::Primitive(value.into_primitive(ctx, true)));
            } else {
                list = list.append(value);
            }

            if !last || trailing {
                list = list.append(delimiter.clone());
            }
        }

        list
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        string::copy_string::SimpleContext, string::copy_string::StringContext,
        structure::prelude::*, Style, GAP,
    };
    use std::error::Error;

    use console::{Attribute, Color};

    use crate::{structure::test::render, EmitForTest};

    use super::*;

    fn frag(s: &'static str, style: impl Into<Style>) -> Structure<SimpleContext> {
        Structure::fragment(SimpleContext::styled(s, style))
    }

    #[test]
    fn high_level_join() -> Result<(), Box<dyn Error>> {
        let red = frag("it-is-red", Color::Red);
        let blue = frag("it-is-blue", Color::Blue);
        let bold = frag("it-is-bold", Attribute::Bold);

        let structure = Group(vec![red, blue, bold].join(GAP()));

        assert_eq!(
            render(&structure, &EmitForTest, 50)?,
            "[Red:it-is-red][normal: ][Blue:it-is-blue][normal: ][normal,bold:it-is-bold]"
        );

        assert_eq!(
            render(&structure, &EmitForTest, 5)?,
            "[Red:it-is-red]\n[Blue:it-is-blue]\n[normal,bold:it-is-bold]"
        );

        Ok(())
    }
}
