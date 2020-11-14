pub mod delimited;
pub mod nested;

use crate::{Style, StyledFragmentTrait, StyledString, BOUNDARY, GAP};

use self::{delimited::DelimitedList, nested::NestedStructure};
use super::{nonempty::NonemptyList, Primitive, Render, Structure};

/// The purpose of `HighLevelStructure` is to support fundamental building blocks for representing
/// pretty-printable data structures, without confusing them with the even more fundamental building
/// blocks of Wadler-style pretty-printers.
#[derive(Debug, Clone)]
pub enum HighLevel {
    DelimitedList(Box<DelimitedList>),
    Nested(Box<NestedStructure>),
    /// A space if laid out inline, or a newline if laid out as a block
    Gap,
    /// Like gap, but may render as a space even if other siblings are laid out as a block
    GapHint,
    /// Nothing if laid out inline, or a newline if laid out as a block
    Boundary,
    /// Like Boundary, but may render as nothing even if other siblings are laid out as a block
    BoundaryHint,
}

impl HighLevel {
    pub fn delimited(
        items: NonemptyList<Structure>,
        delimiter: Structure,
        trailing: bool,
    ) -> HighLevel {
        HighLevel::DelimitedList(Box::new(DelimitedList::new(delimiter, items, trailing)))
    }
}

impl Render for HighLevel {
    fn into_primitive(self, recursive: bool) -> Primitive {
        match self {
            HighLevel::DelimitedList(d) => d.into_primitive(recursive),
            HighLevel::Gap => Primitive::Alt {
                inline: Box::new(Structure::Primitive(Primitive::Fragment(
                    StyledString::new(" ", Style::default()).dynamic(),
                ))),
                block: Box::new(Structure::Primitive(Primitive::Hardline)),
            },
            HighLevel::GapHint => GAP.into_primitive(recursive).group(),
            HighLevel::Boundary => Primitive::Alt {
                inline: Box::new(Structure::Primitive(Primitive::Empty)),
                block: Box::new(Structure::Primitive(Primitive::Hardline)),
            },
            HighLevel::BoundaryHint => BOUNDARY.into_primitive(recursive).group(),
            HighLevel::Nested(nested) => nested.into_primitive(recursive),
        }
    }
}
