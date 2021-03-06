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

    pub fn iter(&self) -> IterList<'_, T> {
        let list: &[T] = &self.vec[..];
        list.into()
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

pub struct IterList<'a, T> {
    list: &'a [T],
    len: usize,
    offset: usize,
}

impl<'a, T> From<&'a [T]> for IterList<'a, T> {
    fn from(list: &'a [T]) -> Self {
        let len = list.len();

        IterList {
            list,
            len,
            offset: 0,
        }
    }
}

// The Clone is totally unnecessary here, but it's hard to model without self-referential structs
// or unsafe code, so I'll go with it for now.
impl<'a, T> Iterator for IterList<'a, T>
where
    T: 'a,
{
    type Item = IterValue<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let offset = self.offset;

        if offset == self.len {
            return None;
        }

        self.offset += 1;

        let value = &self.list[offset];

        Some(IterValue {
            value,
            offset,
            len: self.len,
        })
    }
}

pub struct IterValue<'a, T> {
    value: &'a T,
    offset: usize,
    len: usize,
}

impl<'a, T> IterValue<'a, T> {
    pub fn is_last(&self) -> bool {
        self.offset == self.len - 1
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn value(self) -> &'a T {
        self.value
    }
}

impl<'a, T> Deref for IterValue<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
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
