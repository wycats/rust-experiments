use std::{fmt::Debug, rc::Rc};

use pretty::{BoxAllocator, BoxDoc, DocAllocator};

use crate::{
    render::Nesting, render::RenderState, string::copy_string::StringContext, StyledFragment,
    BOUNDARY,
};

use super::{render::Render, Structure, StyledDoc};

#[derive(Clone)]
pub struct ColumnFn<Ctx>
where
    Ctx: StringContext,
{
    callback: Rc<dyn Fn(usize) -> Structure<Ctx> + Send + Sync + 'static>,
}

impl<Ctx> Debug for ColumnFn<Ctx>
where
    Ctx: StringContext,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ColumnFn")
    }
}

#[derive(Debug)]
pub enum Primitive<Ctx>
where
    Ctx: StringContext,
{
    Empty,
    /// A `StyledFragment` is a piece of text that must not contain any line breaks of any kind.
    Fragment(StyledFragment<Ctx>),
    /// A hard newline
    Hardline,
    /// Attempt to lay out each element of the group by choosing the inline branch of any `Alt`s
    /// inside of the group. A bare `Hardline` inside the group will force all elements of the group
    /// to be laid out using the `block` branch of any `Alt`s.
    Group(Box<Structure<Ctx>>),
    /// A collection of structures which are logically spread out in its location in the data
    /// structure. If a list contains an `Alt`, it is considered to be directly nested inside of its
    /// immediate parent group.
    List(Vec<Structure<Ctx>>),
    /// A structure that is laid out differently in inline context vs. block context.
    Alt {
        inline: Box<Structure<Ctx>>,
        block: Box<Structure<Ctx>>,
    },
    /// A structure that is logically indented by a number of indents. Indents are applied after
    /// each newline, and can either be a precise number of indents, or a logical number of indents
    /// (controlled by configuration).
    Nested {
        indent: Nesting,
        structure: Box<Structure<Ctx>>,
        start_gap: Box<Structure<Ctx>>,
        end_gap: Box<Structure<Ctx>>,
    },
}

impl<Ctx> Clone for Primitive<Ctx>
where
    Ctx: StringContext,
{
    fn clone(&self) -> Self {
        match self {
            Primitive::Empty => Primitive::Empty,
            Primitive::Fragment(frag) => Primitive::Fragment(frag.clone()),
            Primitive::Hardline => Primitive::Hardline,
            Primitive::Group(g) => Primitive::Group(g.clone()),
            Primitive::List(l) => Primitive::List(l.clone()),
            Primitive::Alt { inline, block } => Primitive::Alt {
                inline: inline.clone(),
                block: block.clone(),
            },
            Primitive::Nested {
                indent,
                structure,
                start_gap,
                end_gap,
            } => Primitive::Nested {
                indent: *indent,
                structure: structure.clone(),
                start_gap: start_gap.clone(),
                end_gap: end_gap.clone(),
            },
        }
    }
}

impl<Ctx: StringContext> From<Primitive<Ctx>> for Structure<Ctx> {
    fn from(primitive: Primitive<Ctx>) -> Self {
        Structure::Primitive(primitive)
    }
}

impl<Ctx> Primitive<Ctx>
where
    Ctx: StringContext,
{
    pub fn append(self, structure: impl Into<Structure<Ctx>>) -> Primitive<Ctx> {
        match self {
            Primitive::List(mut v) => {
                v.push(structure.into());
                Primitive::List(v)
            }

            other => Primitive::List(vec![Structure::Primitive(other), structure.into()]),
        }
    }

    pub fn group(self) -> Primitive<Ctx> {
        Primitive::Group(Box::new(self.into()))
    }

    pub fn nest(self) -> Primitive<Ctx> {
        self.wrapping_nest(BOUNDARY(), BOUNDARY())
    }

    pub fn wrapping_nest(
        self,
        pre: impl Into<Structure<Ctx>>,
        post: impl Into<Structure<Ctx>>,
    ) -> Primitive<Ctx> {
        Primitive::Nested {
            indent: Nesting::Configured(1),
            structure: Box::new(Structure::Primitive(self)),
            start_gap: Box::new(pre.into()),
            end_gap: Box::new(post.into()),
        }
    }
}

impl<Ctx> Render<Ctx> for Primitive<Ctx>
where
    Ctx: StringContext,
{
    fn render_with_state(self, state: &RenderState, ctx: &Ctx) -> StyledDoc<Ctx> {
        match self {
            // A `Primitive::Empty` is equivalent to `Doc::Nil`
            Primitive::Empty => BoxDoc::nil(),
            Primitive::Fragment(frag) => BoxDoc::text(frag.emit_plain(ctx)).annotate(frag),
            Primitive::Hardline => BoxDoc::hardline().annotate(StyledFragment::Newline),
            Primitive::Group(s) => s.render_with_state(state, ctx).group(),
            Primitive::List(items) => {
                let mut s = BoxDoc::nil();

                for item in items {
                    s = s.append(item.render_with_state(state, ctx))
                }

                s
            }
            Primitive::Alt { inline, block } => block
                .render_with_state(state, ctx)
                .flat_alt(inline.render_with_state(state, ctx)),
            Primitive::Nested {
                indent,
                structure,
                start_gap,
                end_gap,
            } => {
                let state = state.indent(indent);

                BoxAllocator
                    .nil()
                    .append(
                        BoxAllocator
                            .nil()
                            .append(start_gap.render_with(ctx))
                            // .line_()
                            .append(structure.render_with_state(&state, ctx))
                            .nest(state.size(indent))
                            .append(end_gap.render_with(ctx))
                            // .append(BoxAllocator.line_())
                            .group(),
                    )
                    .into_doc()
            }
        }
    }

    fn into_primitive(self, _recursive: bool) -> Primitive<Ctx> {
        self
    }
}
