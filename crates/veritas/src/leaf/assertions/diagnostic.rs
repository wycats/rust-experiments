#![allow(unused)]

use std::ops::Range;

pub struct Label {
    range: Range<usize>,
    message: String,
}

pub struct Described<T> {
    description: String,
    value: T,
    labels: Vec<Label>,
    notes: Vec<String>,
}

pub enum Diagnostic {
    Diffable(DiffableDiagnostic),
}

pub struct DiffableDiagnostic {
    actual: Described<String>,
    expected: Described<String>,
}
