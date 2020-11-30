#![feature(const_pin)]

mod emit;
pub mod string;

#[doc(hidden)]
#[macro_use]
pub mod structure;

pub use console::Color;
pub use emit::backend::*;
pub use emit::error::*;
pub use emit::into::ToStyledString;
pub use emit::style::*;
pub use emit::test::EmitForTest;
pub use string::intern::Intern;
pub use structure::compose::docs::{self, *};
pub use structure::compose::list::{DocList, Group, Nested};
pub use structure::compose::render_context::RenderContext;
pub use structure::compose::InternedBoxedDoc;
pub use structure::compose::{BoxedDoc, Doc, Fragment};
pub use structure::render::RenderConfig;
pub use structure::*;
