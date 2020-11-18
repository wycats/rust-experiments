use std::{borrow::Cow, fmt::Display, marker::PhantomData, pin::Pin};

use bimap::BiMap;

use super::copy_string::{Repr, StringContext};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum StringId<'a> {
    Id(usize),
    String(&'a str),
}

impl<'a> From<StringId<'a>> for Repr<'a, StringArena<'a>> {
    fn from(id: StringId<'a>) -> Self {
        Repr::new(id)
    }
}

impl<'a> From<Repr<'a, StringArena<'a>>> for StringId<'a> {
    fn from(id: Repr<'a, StringArena<'a>>) -> Self {
        id.value()
    }
}

impl<'a> From<&'a str> for StringId<'a> {
    fn from(s: &'a str) -> Self {
        StringId::String(s)
    }
}

#[derive(Debug, Clone)]
pub struct StringArena<'a> {
    id: usize,
    map: Pin<Box<BiMap<String, usize>>>,
    lt: PhantomData<&'a ()>,
}

impl<'a> Default for StringArena<'a> {
    fn default() -> Self {
        StringArena {
            id: 0,
            map: Pin::new(Box::new(BiMap::new())),
            lt: PhantomData,
        }
    }
}

impl<'a> StringArena<'a> {
    pub fn intern(&mut self, s: impl Display + 'a) -> Repr<'a, StringArena<'a>> {
        let s = format!("{}", s);

        if let Some(id) = self.map.get_by_left(&s) {
            return Repr::new(StringId::Id(*id));
        }

        let id = self.next_id();
        self.map.insert(s, id);
        Repr::new(StringId::Id(id))
    }

    fn get<'b>(&'b self, s: StringId<'a>) -> Cow<'b, str>
    where
        'a: 'b,
    {
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

pub enum StringArenaInput<'a> {
    #[allow(unused)]
    String(String),
    Str(&'a str),
    #[allow(unused)]
    Id(StringId<'a>),
}

impl<'a> From<&'a str> for StringArenaInput<'a> {
    fn from(input: &'a str) -> Self {
        StringArenaInput::Str(input)
    }
}

impl<'a> StringContext<'a> for StringArena<'a> {
    type CustomRepr = StringId<'a>;
    type ValidInput = String;

    fn repr_as_string<'b>(&'b self, id: Repr<'a, Self>) -> Cow<'b, str>
    where
        'a: 'b,
    {
        self.get(id.value())
    }

    fn take(&mut self, value: impl Into<Self::ValidInput> + 'a) -> Repr<'a, Self> {
        self.intern(value.into())
    }
}

// impl From<StringId> for Structure<StringArena> {
//     fn from(id: StringId) -> Self {
//         Structure::Primitive(Primitive::Fragment(
//             StyledString::custom(id, Style::default()).into(),
//         ))
//     }
// }
