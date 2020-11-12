use std::fmt::Formatter;

use crate::Style;

use crate::{EmitBackendTrait, EmitResult};

pub struct EmitForTest;

impl EmitBackendTrait for EmitForTest {
    fn emit(&self, f: &mut Formatter<'_>, fragment: &str, style: &Style) -> EmitResult {
        if fragment == "\n" {
            writeln!(f)?;
        } else {
            write!(f, "[{:?}:{}]", style, fragment)?;
        }

        Ok(())
    }
}
