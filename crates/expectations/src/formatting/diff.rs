use crate::traits::Described;

struct Diff {
    format: fn(&str, &str) -> String,
    actual: Described<String>,
    expected: Described<String>,
}
