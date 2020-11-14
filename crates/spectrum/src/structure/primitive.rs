use std::{fmt::Debug, rc::Rc};

use pretty::{BoxAllocator, BoxDoc, Doc, DocAllocator, DocBuilder};

use crate::{
    render::Nesting, render::RenderState, StyledFragment, StyledFragmentTrait, StyledNewline,
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
    Fragment(StyledFragment),
    /// A hard newline
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
        indent: Nesting,
        structure: Box<Structure>,
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
        Primitive::Nested {
            indent: Nesting::Configured(1),
            structure: Box::new(Structure::Primitive(self)),
        }
    }
}

impl Render for Primitive {
    fn render_with_state(self, state: &RenderState) -> StyledDoc {
        match self {
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
            Primitive::Nested { indent, structure } => {
                let state = state.indent(indent);

                BoxAllocator
                    .nil()
                    .append(structure.render_with_state(&state).nest(state.size(indent)))
                    .into_doc()
            }
        }
    }

    fn into_primitive(self, _recursive: bool) -> Primitive {
        self
    }
}
