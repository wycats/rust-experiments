use derive_new::new;

use super::{Primitive, Render, Structure};

use crate::{string::copy_string::StringContext, structure::HighLevel, NonemptyList};

pub trait JoinExt<Ctx>
where
    Ctx: StringContext,
{
    fn join(self, delimiter: impl Into<Structure<Ctx>>) -> Structure<Ctx>;
    fn join_trailing(self, delimiter: impl Into<Structure<Ctx>>) -> Structure<Ctx>;
}

impl<Ctx> JoinExt<Ctx> for Vec<Structure<Ctx>>
where
    Ctx: StringContext,
{
    fn join(self, delimiter: impl Into<Structure<Ctx>>) -> Structure<Ctx> {
        Structure::HighLevel(HighLevel::DelimitedList(Box::new(JoinList {
            delimiter: delimiter.into(),
            items: self.into(),
            trailing: false,
        })))
    }

    fn join_trailing(self, delimiter: impl Into<Structure<Ctx>>) -> Structure<Ctx> {
        Structure::HighLevel(HighLevel::DelimitedList(Box::new(JoinList {
            delimiter: delimiter.into(),
            items: self.into(),
            trailing: true,
        })))
    }
}

#[derive(Debug, new)]
pub struct JoinList<Ctx>
where
    Ctx: StringContext,
{
    delimiter: Structure<Ctx>,
    items: NonemptyList<Structure<Ctx>>,
    trailing: bool,
}

impl<Ctx> Clone for JoinList<Ctx>
where
    Ctx: StringContext,
{
    fn clone(&self) -> Self {
        JoinList {
            delimiter: self.delimiter.clone(),
            items: self.items.clone(),
            trailing: self.trailing,
        }
    }
}

impl<Ctx> Render<Ctx> for JoinList<Ctx>
where
    Ctx: StringContext,
{
    fn into_primitive(self, recursive: bool) -> Primitive<Ctx> {
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
                list = list.append(Structure::Primitive(value.into_primitive(true)));
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
        string::copy_string::StringContext, structure::prelude::*, Style, StyledString, GAP,
    };
    use std::error::Error;

    use console::{Attribute, Color};

    use crate::{structure::test::render, EmitForTest};

    use super::*;

    fn frag<Ctx>(s: &'static str, style: impl Into<Style>) -> Structure<Ctx>
    where
        Ctx: StringContext,
    {
        Structure::fragment(StyledString::str(s, style.into()))
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
