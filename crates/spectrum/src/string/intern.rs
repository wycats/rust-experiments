use std::{fmt, fmt::Debug, hash::Hash};

use bimap::BiMap;
use format::Display;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum StringId {
    Id(usize),
    Literal(&'static str),
}

impl Into<StringId> for &'static str {
    fn into(self) -> StringId {
        StringId::Literal(self)
    }
}

pub struct InternInternal {
    id_num: usize,
    map: BiMap<StringId, String>,
}

pub struct Intern {
    intern: InternInternal,
}

impl Default for Intern {
    fn default() -> Self {
        Intern::new()
    }
}

impl Intern {
    pub fn new() -> Intern {
        Intern {
            intern: InternInternal {
                id_num: 0,
                map: BiMap::new(),
            },
        }
    }

    pub fn get(&self, id: StringId) -> &str {
        self.intern.map.get_by_left(&id).unwrap()
    }

    pub fn intern(&mut self, string: impl Into<String>) -> StringId {
        let string = string.into();
        let intern = &mut self.intern;

        if let Some(id) = intern.map.get_by_right(&string) {
            return *id;
        }

        let next = intern.id_num + 1;
        intern.id_num = next;
        let id = StringId::Id(next);
        intern.map.insert(id, string);
        id
    }
}

pub trait DerefInternedString {
    fn fmt_interned(&self, f: &mut std::fmt::Formatter<'_>, intern: &Intern) -> fmt::Result;
    fn interned_string(&self, intern: &Intern) -> String {
        format!("{}", Display(move |f| self.fmt_interned(f, intern)))
    }
}

impl DerefInternedString for String {
    fn fmt_interned(&self, f: &mut fmt::Formatter<'_>, _intern: &Intern) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl DerefInternedString for &str {
    fn fmt_interned(&self, f: &mut fmt::Formatter<'_>, _intern: &Intern) -> fmt::Result {
        write!(f, "{}", self)
    }
}
