mod high_level;
mod nonempty;
pub mod prelude;
mod primitive;
pub mod render;
mod renderer;

#[cfg(test)]
mod test;

use pretty::BoxDoc;
use std::fmt::Debug;

use crate::StyledFragment;

use self::render::Render;
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

pub type StyledDoc = BoxDoc<'static, StyledFragment>;

#[allow(non_snake_case)]
pub fn Group(s: impl Into<Structure>) -> Structure {
    Structure::Primitive(Primitive::Group(Box::new(s.into())))
}

pub const GAP: Structure = Structure::HighLevel(HighLevel::Gap);
pub const GAP_HINT: Structure = Structure::HighLevel(HighLevel::GapHint);
pub const BOUNDARY: Structure = Structure::HighLevel(HighLevel::Boundary);
pub const BOUNDARY_HINT: Structure = Structure::HighLevel(HighLevel::BoundaryHint);

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

    pub fn fragment(frag: impl Into<StyledFragment>) -> Structure {
        Structure::Primitive(Primitive::Fragment(frag.into()))
    }

    pub fn append(self, structure: impl Into<Structure>) -> Structure {
        match self {
            Structure::Primitive(p) => Structure::Primitive(p.append(structure)),

            other => Structure::Primitive(Primitive::Empty.append(other).append(structure)),
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
