use std::{fmt::Display, ops::Deref};

#[derive(Debug, Clone)]
pub struct NonemptyList<T> {
    vec: Vec<T>,
}

impl<T> NonemptyList<T> {
    pub fn new(list: Vec<T>) -> NonemptyList<T> {
        assert!(!list.is_empty(), "A nonempty list must have elements");

        NonemptyList { vec: list }
    }

    pub fn drain(self) -> DrainList<T> {
        self.vec.into()
    }
}

#[derive(Debug)]
pub struct EmptyVec;

impl Display for EmptyVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A nonempty list must have element")
    }
}

impl std::error::Error for EmptyVec {}

impl<T> From<Vec<T>> for NonemptyList<T> {
    fn from(vec: Vec<T>) -> Self {
        NonemptyList::new(vec)
    }
}

pub struct DrainList<T> {
    vec: Vec<T>,
    len: usize,
    offset: usize,
}

impl<T> From<Vec<T>> for DrainList<T> {
    fn from(vec: Vec<T>) -> Self {
        let len = vec.len();

        DrainList {
            vec,
            len,
            offset: 0,
        }
    }
}

// The Clone is totally unnecessary here, but it's hard to model without self-referential structs
// or unsafe code, so I'll go with it for now.
impl<T> Iterator for DrainList<T>
where
    T: Clone,
{
    type Item = DrainValue<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let offset = self.offset;

        if offset == self.len {
            return None;
        }

        self.offset += 1;

        let value = self.vec[offset].clone();

        Some(DrainValue {
            value,
            offset,
            len: self.len,
        })
    }
}

pub struct DrainValue<T> {
    value: T,
    offset: usize,
    len: usize,
}

impl<T> DrainValue<T> {
    pub fn is_last(&self) -> bool {
        self.offset == self.len - 1
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn value(self) -> T {
        self.value
    }
}

impl<T> Deref for DrainValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
