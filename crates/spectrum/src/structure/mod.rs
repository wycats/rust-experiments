mod renderer;

use pretty::BoxDoc;

use crate::{StyledFragment, StyledFragmentTrait, StyledNewline};

type ColumnFn = Box<dyn Fn(usize) -> Structure + 'static>;

#[allow(unused)]
pub enum Structure {
    Empty,
    Fragment(StyledFragment),
    /// must be a newline
    Hardline,
    /// If possible, a group is laid out inline. If not possible,
    /// each element in the group is laid out as its own block.
    Group(Box<Structure>),
    /// A collection of structures
    List(Vec<Structure>),
    Alt {
        inline: Box<Structure>,
        block: Box<Structure>,
    },
    Nested {
        indent: isize,
        structure: Box<Structure>,
    },
    ForColumn(ColumnFn),
    ForNesting(ColumnFn),
}

impl Structure {
    #[allow(unused)]
    pub fn fragment(frag: impl Into<StyledFragment>) -> Structure {
        Structure::Fragment(frag.into())
    }

    #[allow(unused)]
    pub fn append(self, structure: impl Into<Structure>) -> Structure {
        match self {
            Structure::List(mut v) => {
                v.push(structure.into());
                Structure::List(v)
            }

            other => Structure::List(vec![other, structure.into()]),
        }
    }

    #[allow(unused)]
    pub fn render(&self) -> BoxDoc<StyledFragment> {
        match self {
            Structure::Empty => BoxDoc::nil(),
            Structure::Fragment(frag) => BoxDoc::text(frag.plain()).annotate(frag.clone()),
            Structure::Hardline => BoxDoc::hardline().annotate(StyledNewline.clone_frag()),
            Structure::Group(s) => s.render().group(),
            Structure::List(items) => {
                let mut s = BoxDoc::nil();

                for item in items {
                    s = s.append(item.render())
                }

                s
            }
            Structure::Alt { inline, block } => BoxDoc::flat_alt(inline.render(), block.render()),
            Structure::Nested { indent, structure } => structure.render().nest(*indent),
            Structure::ForColumn(_) => todo!(),
            Structure::ForNesting(_) => todo!(),
        }
    }
}
