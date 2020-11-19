#![allow(non_snake_case)]

mod compose;
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
use std::{fmt::Debug, marker::PhantomData};

use crate::{
    string::copy_string::{Repr, StringContext},
    StyledFragment,
};

use self::render::{Nesting, Render};
pub use self::{high_level::HighLevel, nonempty::NonemptyList, primitive::Primitive};

#[derive(Debug)]
pub enum Structure<'a, Ctx>
where
    Ctx: StringContext<'a> + 'a,
{
    Primitive(Primitive<'a, Ctx>),
    HighLevel(HighLevel<'a, Ctx>),
}

impl<'a, Ctx> Into<Structure<'a, Ctx>> for &'static str
where
    Ctx: StringContext<'a>,
{
    fn into(self) -> Structure<'a, Ctx> {
        Structure::Primitive(Primitive::Fragment(
            Ctx::plain_repr(Repr::new(self.into())).into(),
        ))
    }
}

impl<'a, Ctx> Clone for Structure<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    fn clone(&self) -> Self {
        match self {
            Structure::Primitive(p) => Structure::Primitive(p.clone()),
            Structure::HighLevel(h) => Structure::HighLevel(h.clone()),
        }
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
pub type StyledDoc<'a, Ctx> = BoxDoc<'a, StyledFragment<'a, Ctx>>;

#[allow(non_snake_case)]
pub fn Group<'a, Ctx>(s: impl Into<Structure<'a, Ctx>>) -> Structure<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    Structure::Primitive(Primitive::Group(Box::new(s.into())))
}

#[allow(non_snake_case)]
pub fn Doc<'a, Ctx>(s: impl Into<Structure<'a, Ctx>>) -> Structure<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    EMPTY().append(s)
}

/// An [EMPTY] is the starting point ("unit value") of any structure.
pub fn EMPTY<'a, Ctx>() -> Structure<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    Structure::Primitive(Primitive::Empty)
}

/// A [HARDLINE] forces a line break, no matter what. The presence of a [HARDLINE] inside of a group
/// means that all other members of the group will be laid out in block mode.
pub fn HARDLINE<'a, Ctx>() -> Structure<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    Structure::Primitive(Primitive::Hardline)
}

/// A [GAP] is a space when laid out inline and a newline when laid out as a block. When any sibling
/// of [GAP] is laid out like a block, the gap becomes a newline.
pub fn GAP<'a, Ctx>() -> Structure<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    Structure::HighLevel(HighLevel::Gap)
}

/// a [GAP_HINT] is a newline **only** if laying out the next element inline (after a space for the
/// gap) would overflow the page size.
pub fn GAP_HINT<'a, Ctx>() -> Structure<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    Structure::HighLevel(HighLevel::GapHint)
}

/// A [BOUNDARY] is nothing when laid out inline and a newline when laid out as a block. When any
/// sibling of [BOUNDARY] is laid out like a block, the boundary becomes a newline.
pub fn BOUNDARY<'a, Ctx>() -> Structure<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    Structure::HighLevel(HighLevel::Boundary)
}
/// A [BOUNDARY_HINT] is a newline **only** if laying out the next element inline would overflow the
/// page size.
pub fn BOUNDARY_HINT<'a, Ctx>() -> Structure<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    Structure::HighLevel(HighLevel::BoundaryHint)
}

pub struct Alt<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    lt: PhantomData<&'a ()>,
    ctx: PhantomData<Ctx>,
}

impl<'a, Ctx> Alt<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    pub fn inline(s: impl Into<Structure<'a, Ctx>>) -> InlineAlt<'a, Ctx> {
        InlineAlt { inline: s.into() }
    }
}

pub struct InlineAlt<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    inline: Structure<'a, Ctx>,
}

impl<'a, Ctx> InlineAlt<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    pub fn block(self, s: impl Into<Structure<'a, Ctx>>) -> Structure<'a, Ctx> {
        Structure::Primitive(Primitive::Alt {
            inline: Box::new(self.inline),
            block: Box::new(s.into()),
        })
    }
}

impl<'a, Ctx> Structure<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    pub fn hardline() -> Structure<'a, Ctx> {
        Structure::Primitive(Primitive::Hardline)
    }

    pub fn delimited(
        items: impl Into<NonemptyList<Structure<'a, Ctx>>>,
        delimiter: impl Into<Structure<'a, Ctx>>,
    ) -> Structure<'a, Ctx> {
        Structure::HighLevel(HighLevel::delimited(items.into(), delimiter.into(), false))
    }

    pub fn delimited_trailing(
        items: impl Into<NonemptyList<Structure<'a, Ctx>>>,
        delimiter: impl Into<Structure<'a, Ctx>>,
    ) -> Structure<'a, Ctx> {
        Structure::HighLevel(HighLevel::delimited(items.into(), delimiter.into(), true))
    }

    pub fn group(self) -> Structure<'a, Ctx> {
        Structure::Primitive(Primitive::Group(Box::new(self)))
    }

    pub fn nest(self) -> Structure<'a, Ctx> {
        self.wrapping_nest(BOUNDARY(), BOUNDARY())
    }

    pub fn wrapping_nest(
        self,
        pre: impl Into<Structure<'a, Ctx>>,
        post: impl Into<Structure<'a, Ctx>>,
    ) -> Structure<'a, Ctx> {
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

    pub fn fragment(frag: impl Into<StyledFragment<'a, Ctx>>) -> Structure<'a, Ctx> {
        Structure::Primitive(Primitive::Fragment(frag.into()))
    }

    // pub fn add(self, s: )

    pub fn append(self, structure: impl Into<Structure<'a, Ctx>>) -> Structure<'a, Ctx> {
        match self {
            Structure::Primitive(p) => Structure::Primitive(p.append(structure)),

            other => Structure::Primitive(Primitive::Empty.append(other).append(structure)),
        }
    }

    pub fn append_group(self, structure: impl Into<Structure<'a, Ctx>>) -> Structure<'a, Ctx> {
        match self {
            Structure::Primitive(p) => Structure::Primitive(p.append(structure.into().group())),

            other => Structure::Primitive(Primitive::Empty.append(other).append(structure).group()),
        }
    }
}

impl<'a, Ctx> Render<'a, Ctx> for Structure<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    fn into_primitive(self, ctx: &mut Ctx, recursive: bool) -> Primitive<'a, Ctx> {
        match self {
            Structure::Primitive(p) => p,
            Structure::HighLevel(h) => h.into_primitive(ctx, recursive),
        }
    }
}
