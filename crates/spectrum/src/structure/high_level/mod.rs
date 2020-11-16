pub mod join;
pub mod nested;

use crate::{
    render::RenderConfig, render::RenderState, Style, StyledDoc, StyledFragmentTrait, StyledString,
    BOUNDARY, GAP,
};

use self::{join::JoinList, nested::NestedStructure};
use super::{nonempty::NonemptyList, Primitive, Render, Structure};

/// You can implement [DynRender] to create custom high-level structures outside of the [spectrum]
/// crate.
pub trait DynRender: std::fmt::Debug {
    fn into_primitive(&self, recursive: bool) -> Primitive;

    fn clone_dyn_render(&self) -> Box<dyn DynRender>;

    fn render(&self) -> StyledDoc {
        self.render_with_state(&RenderState::default())
    }

    fn render_with_config(&self, config: RenderConfig) -> StyledDoc {
        self.render_with_state(&RenderState::top(config))
    }

    fn render_with_state(&self, state: &RenderState) -> StyledDoc {
        self.into_primitive(true).render_with_state(state)
    }
}

impl Clone for Box<dyn DynRender> {
    fn clone(&self) -> Self {
        self.clone_dyn_render()
    }
}

/// The purpose of `HighLevelStructure` is to support fundamental building blocks for representing
/// pretty-printable data structures, without confusing them with the even more fundamental building
/// blocks of Wadler-style pretty-printers.
#[derive(Debug, Clone)]
pub enum HighLevel {
    DelimitedList(Box<JoinList>),
    Nested(Box<NestedStructure>),
    HighLevel(Box<dyn DynRender>),
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
        HighLevel::DelimitedList(Box::new(JoinList::new(delimiter, items, trailing)))
    }
}

impl Render for HighLevel {
    fn into_primitive(self, recursive: bool) -> Primitive {
        match self {
            HighLevel::DelimitedList(d) => d.into_primitive(recursive),
            HighLevel::Nested(nested) => nested.into_primitive(recursive),
            HighLevel::HighLevel(r) => r.into_primitive(recursive),
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
        }
    }
}
