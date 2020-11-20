pub use crate::emit::backend::{EmitColored, EmitPlain};

pub mod test {
    pub use crate::EmitForTest;
    pub type TestResult = Result<(), Box<dyn std::error::Error>>;
}
