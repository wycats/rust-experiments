pub mod join;
pub mod nested;

use crate::{
    render::RenderConfig, render::RenderState, string::copy_string::StringContext, Style,
    StyledDoc, BOUNDARY, GAP,
};

use self::{join::JoinList, nested::NestedStructure};
use super::{nonempty::NonemptyList, Primitive, Render, Structure};

/// You can implement [DynRender] to create custom high-level structures outside of the [spectrum]
/// crate.
pub trait DynRender<Ctx>: std::fmt::Debug
where
    Ctx: StringContext,
{
    fn into_primitive(&self, ctx: &Ctx, recursive: bool) -> Primitive<Ctx>;

    fn clone_dyn_render(&self) -> Box<dyn DynRender<Ctx>>;

    fn render(&self, ctx: &mut Ctx) -> StyledDoc<Ctx> {
        self.render_with_state(&RenderState::default(), ctx)
    }

    fn render_with_config(&self, config: RenderConfig, ctx: &mut Ctx) -> StyledDoc<Ctx> {
        self.render_with_state(&RenderState::top(config), ctx)
    }

    fn render_with_state(&self, state: &RenderState, ctx: &mut Ctx) -> StyledDoc<Ctx> {
        self.into_primitive(ctx, true).render_with_state(state, ctx)
    }
}

impl<Ctx> Clone for Box<dyn DynRender<Ctx>>
where
    Ctx: StringContext,
{
    fn clone(&self) -> Self {
        self.clone_dyn_render()
    }
}

/// The purpose of `HighLevelStructure` is to support fundamental building blocks for representing
/// pretty-printable data structures, without confusing them with the even more fundamental building
/// blocks of Wadler-style pretty-printers.
#[derive(Debug)]
pub enum HighLevel<Ctx>
where
    Ctx: StringContext,
{
    DelimitedList(Box<JoinList<Ctx>>),
    Nested(Box<NestedStructure<Ctx>>),
    HighLevel(Box<dyn DynRender<Ctx>>),
    /// A space if laid out inline, or a newline if laid out as a block
    Gap,
    /// Like gap, but may render as a space even if other siblings are laid out as a block
    GapHint,
    /// Nothing if laid out inline, or a newline if laid out as a block
    Boundary,
    /// Like Boundary, but may render as nothing even if other siblings are laid out as a block
    BoundaryHint,
}

impl<Ctx> Clone for HighLevel<Ctx>
where
    Ctx: StringContext,
{
    fn clone(&self) -> Self {
        match self {
            HighLevel::DelimitedList(l) => HighLevel::DelimitedList(l.clone()),
            HighLevel::Nested(n) => HighLevel::Nested(n.clone()),
            HighLevel::HighLevel(h) => HighLevel::HighLevel(h.clone()),
            HighLevel::Gap => todo!(),
            HighLevel::GapHint => todo!(),
            HighLevel::Boundary => todo!(),
            HighLevel::BoundaryHint => todo!(),
        }
    }
}

impl<Ctx> HighLevel<Ctx>
where
    Ctx: StringContext,
{
    pub fn delimited(
        items: NonemptyList<Structure<Ctx>>,
        delimiter: Structure<Ctx>,
        trailing: bool,
    ) -> HighLevel<Ctx> {
        HighLevel::DelimitedList(Box::new(JoinList::new(delimiter, items, trailing)))
    }
}

impl<Ctx> Render<Ctx> for HighLevel<Ctx>
where
    Ctx: StringContext,
{
    fn into_primitive(self, ctx: &mut Ctx, recursive: bool) -> Primitive<Ctx> {
        match self {
            HighLevel::DelimitedList(d) => d.into_primitive(ctx, recursive),
            HighLevel::Nested(nested) => nested.into_primitive(ctx, recursive),
            HighLevel::HighLevel(r) => r.into_primitive(ctx, recursive),
            HighLevel::Gap => Primitive::Alt {
                inline: Box::new(Structure::Primitive(Primitive::Fragment(
                    ctx.styled(" ".into(), Style::default()).into(),
                ))),
                block: Box::new(Structure::Primitive(Primitive::Hardline)),
            },
            HighLevel::GapHint => GAP().into_primitive(ctx, recursive).group(),
            HighLevel::Boundary => Primitive::Alt {
                inline: Box::new(Structure::Primitive(Primitive::Empty)),
                block: Box::new(Structure::Primitive(Primitive::Hardline)),
            },
            HighLevel::BoundaryHint => BOUNDARY().into_primitive(ctx, recursive).group(),
        }
    }
}
