mod emit;
mod string;
mod structure;

pub use console::Color;
pub use emit::error::*;
pub use emit::fragment::*;
pub use emit::into::ToStyledString;
pub use emit::style::*;
pub use emit::test::EmitForTest;
pub use string::copy_string::{SimpleContext, StringContext};
pub use string::intern::StringArena;
pub use structure::*;
