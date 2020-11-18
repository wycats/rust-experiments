use std::{fmt::Debug, rc::Rc};

use pretty::{BoxAllocator, BoxDoc, DocAllocator};

use crate::{
    render::Nesting, render::RenderState, string::copy_string::StringContext, StyledFragment,
    BOUNDARY,
};

use super::{render::Render, Structure, StyledDoc};

#[derive(Clone)]
pub struct ColumnFn<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    callback: Rc<dyn Fn(usize) -> Structure<'a, Ctx> + Send + Sync + 'static>,
}

impl<'a, Ctx> Debug for ColumnFn<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ColumnFn")
    }
}

#[derive(Debug)]
pub enum Primitive<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    Empty,
    /// A `StyledFragment` is a piece of text that must not contain any line breaks of any kind.
    Fragment(StyledFragment<'a, Ctx>),
    /// A hard newline
    Hardline,
    /// Attempt to lay out each element of the group by choosing the inline branch of any `Alt`s
    /// inside of the group. A bare `Hardline` inside the group will force all elements of the group
    /// to be laid out using the `block` branch of any `Alt`s.
    Group(Box<Structure<'a, Ctx>>),
    /// A collection of structures which are logically spread out in its location in the data
    /// structure. If a list contains an `Alt`, it is considered to be directly nested inside of its
    /// immediate parent group.
    List(Vec<Structure<'a, Ctx>>),
    /// A structure that is laid out differently in inline context vs. block context.
    Alt {
        inline: Box<Structure<'a, Ctx>>,
        block: Box<Structure<'a, Ctx>>,
    },
    /// A structure that is logically indented by a number of indents. Indents are applied after
    /// each newline, and can either be a precise number of indents, or a logical number of indents
    /// (controlled by configuration).
    Nested {
        indent: Nesting,
        structure: Box<Structure<'a, Ctx>>,
        start_gap: Box<Structure<'a, Ctx>>,
        end_gap: Box<Structure<'a, Ctx>>,
    },
}

impl<'a, Ctx> Clone for Primitive<'a, Ctx>
where
    Ctx: StringContext<'a>,
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

impl<'a, Ctx: StringContext<'a>> From<Primitive<'a, Ctx>> for Structure<'a, Ctx> {
    fn from(primitive: Primitive<'a, Ctx>) -> Self {
        Structure::Primitive(primitive)
    }
}

impl<'a, Ctx> Primitive<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    pub fn append(self, structure: impl Into<Structure<'a, Ctx>>) -> Primitive<'a, Ctx> {
        match self {
            Primitive::List(mut v) => {
                v.push(structure.into());
                Primitive::List(v)
            }

            other => Primitive::List(vec![Structure::Primitive(other), structure.into()]),
        }
    }

    pub fn group(self) -> Primitive<'a, Ctx> {
        Primitive::Group(Box::new(self.into()))
    }

    pub fn nest(self) -> Primitive<'a, Ctx> {
        self.wrapping_nest(BOUNDARY(), BOUNDARY())
    }

    pub fn wrapping_nest(
        self,
        pre: impl Into<Structure<'a, Ctx>>,
        post: impl Into<Structure<'a, Ctx>>,
    ) -> Primitive<'a, Ctx> {
        Primitive::Nested {
            indent: Nesting::Configured(1),
            structure: Box::new(Structure::Primitive(self)),
            start_gap: Box::new(pre.into()),
            end_gap: Box::new(post.into()),
        }
    }
}

impl<'a, Ctx> Render<'a, Ctx> for Primitive<'a, Ctx>
where
    Ctx: StringContext<'a>,
{
    fn render_with_state<'b>(self, state: &RenderState, ctx: &'b mut Ctx) -> StyledDoc<'a, Ctx> {
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

    fn into_primitive(self, _ctx: &mut Ctx, _recursive: bool) -> Primitive<'a, Ctx> {
        self
    }
}
