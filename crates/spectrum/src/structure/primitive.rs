use std::{fmt::Debug, rc::Rc};

use pretty::{BoxAllocator, BoxDoc, DocAllocator};

use crate::{
    render::Nesting, render::RenderState, StyledFragment, StyledFragmentTrait, StyledNewline,
    BOUNDARY,
};

use super::{render::Render, Structure, StyledDoc};

#[derive(Clone)]
pub struct ColumnFn {
    callback: Rc<dyn Fn(usize) -> Structure + Send + Sync + 'static>,
}

impl Debug for ColumnFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ColumnFn")
    }
}

#[derive(Debug, Clone)]
pub enum Primitive {
    Empty,
    /// A `StyledFragment` is a piece of text that must not contain any line breaks of any kind.
    Fragment(StyledFragment),
    /// A hard newline
    Hardline,
    /// Attempt to lay out each element of the group by choosing the inline branch of any `Alt`s
    /// inside of the group. A bare `Hardline` inside the group will force all elements of the group
    /// to be laid out using the `block` branch of any `Alt`s.
    Group(Box<Structure>),
    /// A collection of structures which are logically spread out in its location in the data
    /// structure. If a list contains an `Alt`, it is considered to be directly nested inside of its
    /// immediate parent group.
    List(Vec<Structure>),
    /// A structure that is laid out differently in inline context vs. block context.
    Alt {
        inline: Box<Structure>,
        block: Box<Structure>,
    },
    /// A structure that is logically indented by a number of indents. Indents are applied after
    /// each newline, and can either be a precise number of indents, or a logical number of indents
    /// (controlled by configuration).
    Nested {
        indent: Nesting,
        structure: Box<Structure>,
        start_gap: Box<Structure>,
        end_gap: Box<Structure>,
    },
}

impl From<Primitive> for Structure {
    fn from(primitive: Primitive) -> Self {
        Structure::Primitive(primitive)
    }
}

impl Primitive {
    pub fn append(self, structure: impl Into<Structure>) -> Primitive {
        match self {
            Primitive::List(mut v) => {
                v.push(structure.into());
                Primitive::List(v)
            }

            other => Primitive::List(vec![Structure::Primitive(other), structure.into()]),
        }
    }

    pub fn group(self) -> Primitive {
        Primitive::Group(Box::new(self.into()))
    }

    pub fn nest(self) -> Primitive {
        self.wrapping_nest(BOUNDARY, BOUNDARY)
    }

    pub fn wrapping_nest(self, pre: impl Into<Structure>, post: impl Into<Structure>) -> Primitive {
        Primitive::Nested {
            indent: Nesting::Configured(1),
            structure: Box::new(Structure::Primitive(self)),
            start_gap: Box::new(pre.into()),
            end_gap: Box::new(post.into()),
        }
    }
}

impl Render for Primitive {
    fn render_with_state(self, state: &RenderState) -> StyledDoc {
        match self {
            // A `Primitive::Empty` is equivalent to `Doc::Nil`
            Primitive::Empty => BoxDoc::nil(),
            Primitive::Fragment(frag) => BoxDoc::text(frag.plain()).annotate(frag),
            Primitive::Hardline => BoxDoc::hardline().annotate(StyledNewline.dynamic()),
            Primitive::Group(s) => s.render_with_state(state).group(),
            Primitive::List(items) => {
                let mut s = BoxDoc::nil();

                for item in items {
                    s = s.append(item.render_with_state(state))
                }

                s
            }
            Primitive::Alt { inline, block } => block
                .render_with_state(state)
                .flat_alt(inline.render_with_state(state)),
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
                            .append(start_gap.render())
                            // .line_()
                            .append(structure.render_with_state(&state))
                            .nest(state.size(indent))
                            .append(end_gap.render())
                            // .append(BoxAllocator.line_())
                            .group(),
                    )
                    .into_doc()
            }
        }
    }

    fn into_primitive(self, _recursive: bool) -> Primitive {
        self
    }
}
