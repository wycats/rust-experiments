use derive_new::new;

use super::{Primitive, Render, Structure};

use crate::{structure::HighLevel, NonemptyList};

pub trait JoinExt {
    fn join(self, delimiter: impl Into<Structure>) -> Structure;
    fn join_trailing(self, delimiter: impl Into<Structure>) -> Structure;
}

impl JoinExt for Vec<Structure> {
    fn join(self, delimiter: impl Into<Structure>) -> Structure {
        Structure::HighLevel(HighLevel::DelimitedList(Box::new(JoinList {
            delimiter: delimiter.into(),
            items: self.into(),
            trailing: false,
        })))
    }

    fn join_trailing(self, delimiter: impl Into<Structure>) -> Structure {
        Structure::HighLevel(HighLevel::DelimitedList(Box::new(JoinList {
            delimiter: delimiter.into(),
            items: self.into(),
            trailing: true,
        })))
    }
}

#[derive(Debug, Clone, new)]
pub struct JoinList {
    delimiter: Structure,
    items: NonemptyList<Structure>,
    trailing: bool,
}

impl Render for JoinList {
    fn into_primitive(self, recursive: bool) -> Primitive {
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
    use crate::{structure::prelude::*, GAP};
    use std::error::Error;

    use console::{Attribute, Color};

    use crate::{
        structure::{test::render, Primitive},
        EmitForTest, StyledFragment,
    };

    use super::*;

    fn frag(frag: impl Into<StyledFragment>) -> Structure {
        Structure::Primitive(Primitive::Fragment(frag.into()))
    }

    #[test]
    fn high_level_join() -> Result<(), Box<dyn Error>> {
        let red = frag(("it-is-red", Color::Red));
        let blue = frag(("it-is-blue", Color::Blue));
        let bold = frag(("it-is-bold", Attribute::Bold));

        let structure = Group(vec![red, blue, bold].join(GAP));

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
