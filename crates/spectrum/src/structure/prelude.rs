pub use crate::emit::fragment::{EmitColored, EmitPlain};
pub use crate::render::Render;
pub use crate::structure::high_level::join::JoinExt;
pub use crate::structure::high_level::nested::Nested;
pub use crate::structure::{
    Alt, Doc, Group, HighLevel, Primitive, Structure, BOUNDARY, BOUNDARY_HINT, EMPTY, GAP,
    GAP_HINT, HARDLINE,
};

#[cfg(test)]
pub use crate::EmitForTest;

#[cfg(test)]
pub use crate::structure::test::render;

#[cfg(test)]
pub type TestResult = Result<(), Box<dyn std::error::Error>>;
