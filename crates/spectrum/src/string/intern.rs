use std::borrow::Cow;

use bimap::BiMap;

use super::copy_string::{Repr, StringContext};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum StringId {
    Id(usize),
    String(&'static str),
}

impl From<&'static str> for StringId {
    fn from(s: &'static str) -> Self {
        StringId::String(s)
    }
}

#[derive(Debug, Clone)]
pub struct StringArena {
    id: usize,
    map: BiMap<String, usize>,
}

impl Default for StringArena {
    fn default() -> Self {
        StringArena {
            id: 0,
            map: BiMap::new(),
        }
    }
}

impl StringArena {
    pub fn intern(&mut self, s: impl Into<String>) -> Repr<StringArena> {
        let s = s.into();

        if let Some(id) = self.map.get_by_left(&s) {
            return Repr::new(StringId::Id(*id));
        }

        let id = self.next_id();
        self.map.insert(s, id);
        Repr::new(StringId::Id(id))
    }

    fn get(&self, s: StringId) -> Cow<'_, str> {
        match s {
            StringId::Id(id) => {
                let value = self.map.get_by_right(&id).expect(
                    "Once a StringId has been given out for an arena, it should always be valid",
                );

                Cow::Borrowed(value)
            }
            StringId::String(s) => Cow::Borrowed(s),
        }
    }

    fn next_id(&mut self) -> usize {
        let id = self.id;
        self.id += 1;
        id
    }
}

pub enum StringArenaInput {
    #[allow(unused)]
    String(String),
    Str(&'static str),
    #[allow(unused)]
    Id(StringId),
}

impl From<&'static str> for StringArenaInput {
    fn from(input: &'static str) -> Self {
        StringArenaInput::Str(input)
    }
}

impl StringContext for StringArena {
    type CustomRepr = StringId;
    type InputCustomRepr = StringArenaInput;

    fn as_repr(&mut self, input: StringArenaInput) -> Repr<Self> {
        match input {
            StringArenaInput::String(s) => self.intern(s),
            StringArenaInput::Str(s) => self.intern(s),
            StringArenaInput::Id(id) => Repr::new(id),
        }
    }

    fn repr_as_string(&self, id: StringId) -> Cow<'_, str> {
        self.get(id)
    }
}

// impl From<StringId> for Structure<StringArena> {
//     fn from(id: StringId) -> Self {
//         Structure::Primitive(Primitive::Fragment(
//             StyledString::custom(id, Style::default()).into(),
//         ))
//     }
// }
