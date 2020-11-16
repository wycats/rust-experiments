pub mod high_level;
mod nonempty;
pub mod prelude;
mod primitive;
pub mod render;
mod renderer;

#[cfg(test)]
mod test;

#[cfg(test)]
mod tests;

use pretty::BoxDoc;
use std::fmt::Debug;

use crate::StyledFragment;

use self::render::{Nesting, Render};
pub use self::{high_level::HighLevel, nonempty::NonemptyList, primitive::Primitive};

#[derive(Debug, Clone)]
pub enum Structure {
    Primitive(Primitive),
    HighLevel(HighLevel),
}

impl From<&'_ str> for Structure {
    fn from(string: &'_ str) -> Self {
        Structure::Primitive(Primitive::Fragment(string.into()))
    }
}

/// A [StyledDoc] is a [pretty::BoxDoc] that is annotated with a [StyledFragment].
///
/// What this means is that the contents of the [pretty::Doc] are used for the purpose of the pretty
/// printing algorithm, but the [StyledFragment] is used for the purpose of rendering. This also
/// means that the physical size of the contents of the [pretty::Doc] must be equivalent to the
/// physical size of the [StyledFragment] after printing.
///
/// The implementation of [StyledFragment] and [Primitive] protect this invariant, as users of
/// [spectrum] cannot create [StyleDoc]s manually through the [spectrum::structure] API.
pub type StyledDoc = BoxDoc<'static, StyledFragment>;

#[allow(non_snake_case)]
pub fn Group(s: impl Into<Structure>) -> Structure {
    Structure::Primitive(Primitive::Group(Box::new(s.into())))
}

#[allow(non_snake_case)]
pub fn Doc(s: impl Into<Structure>) -> Structure {
    EMPTY.append(s)
}

/// An [EMPTY] is the starting point ("unit value") of any structure.
pub const EMPTY: Structure = Structure::Primitive(Primitive::Empty);

/// A [HARDLINE] forces a line break, no matter what. The presence of a [HARDLINE] inside of a group
/// means that all other members of the group will be laid out in block mode.
pub const HARDLINE: Structure = Structure::Primitive(Primitive::Hardline);

/// A [GAP] is a space when laid out inline and a newline when laid out as a block. When any sibling
/// of [GAP] is laid out like a block, the gap becomes a newline.
pub const GAP: Structure = Structure::HighLevel(HighLevel::Gap);
/// a [GAP_HINT] is a newline **only** if laying out the next element inline (after a space for the
/// gap) would overflow the page size.
pub const GAP_HINT: Structure = Structure::HighLevel(HighLevel::GapHint);
/// A [BOUNDARY] is nothing when laid out inline and a newline when laid out as a block. When any
/// sibling of [BOUNDARY] is laid out like a block, the boundary becomes a newline.
pub const BOUNDARY: Structure = Structure::HighLevel(HighLevel::Boundary);
/// A [BOUNDARY_HINT] is a newline **only** if laying out the next element inline would overflow the
/// page size.
pub const BOUNDARY_HINT: Structure = Structure::HighLevel(HighLevel::BoundaryHint);

pub struct Alt;

impl Alt {
    pub fn inline(s: impl Into<Structure>) -> InlineAlt {
        InlineAlt { inline: s.into() }
    }
}

pub struct InlineAlt {
    inline: Structure,
}

impl InlineAlt {
    pub fn block(self, s: impl Into<Structure>) -> Structure {
        Structure::Primitive(Primitive::Alt {
            inline: Box::new(self.inline),
            block: Box::new(s.into()),
        })
    }
}

impl Structure {
    pub fn hardline() -> Structure {
        Structure::Primitive(Primitive::Hardline)
    }

    pub fn delimited(
        items: impl Into<NonemptyList<Structure>>,
        delimiter: impl Into<Structure>,
    ) -> Structure {
        Structure::HighLevel(HighLevel::delimited(items.into(), delimiter.into(), false))
    }

    pub fn delimited_trailing(
        items: impl Into<NonemptyList<Structure>>,
        delimiter: impl Into<Structure>,
    ) -> Structure {
        Structure::HighLevel(HighLevel::delimited(items.into(), delimiter.into(), true))
    }

    pub fn group(self) -> Structure {
        Structure::Primitive(Primitive::Group(Box::new(self)))
    }

    pub fn nest(self) -> Structure {
        self.wrapping_nest(BOUNDARY, BOUNDARY)
    }

    pub fn wrapping_nest(self, pre: impl Into<Structure>, post: impl Into<Structure>) -> Structure {
        Structure::Primitive(match self {
            Structure::Primitive(p) => p.wrapping_nest(pre, post),
            Structure::HighLevel(h) => Primitive::Nested {
                indent: Nesting::Configured(1),
                structure: Box::new(Structure::HighLevel(h)),
                start_gap: Box::new(pre.into()),
                end_gap: Box::new(post.into()),
            },
        })
    }

    pub fn fragment(frag: impl Into<StyledFragment>) -> Structure {
        Structure::Primitive(Primitive::Fragment(frag.into()))
    }

    pub fn append(self, structure: impl Into<Structure>) -> Structure {
        match self {
            Structure::Primitive(p) => Structure::Primitive(p.append(structure)),

            other => Structure::Primitive(Primitive::Empty.append(other).append(structure)),
        }
    }

    pub fn append_group(self, structure: impl Into<Structure>) -> Structure {
        match self {
            Structure::Primitive(p) => Structure::Primitive(p.append(structure.into().group())),

            other => Structure::Primitive(Primitive::Empty.append(other).append(structure).group()),
        }
    }
}

impl Render for Structure {
    fn into_primitive(self, recursive: bool) -> Primitive {
        match self {
            Structure::Primitive(p) => p,
            Structure::HighLevel(h) => h.into_primitive(recursive),
        }
    }
}
