#![allow(clippy::identity_op)]

//! Wrap console::Attribute and console::Style to tweak their behavior (especially debug output)

use derive_new::new;
use modular_bitfield::bitfield;
use std::{fmt::Debug, fmt::Display, hash::Hash, ops::BitOr, ops::BitOrAssign};

#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Attributes {
    bold: bool,
    dim: bool,
    italic: bool,
    underlined: bool,
    blink: bool,
    reverse: bool,
    hidden: bool,
    #[skip]
    __: bool,
}

impl Attributes {
    pub fn insert(&mut self, attrs: impl Into<Attributes>) {
        *self = Attributes::from(u8::from(*self) | u8::from(attrs.into()))
    }

    pub fn has_any_attr(self) -> bool {
        self.bold()
            || self.dim()
            || self.italic()
            || self.underlined()
            || self.blink()
            || self.reverse()
            || self.hidden()
    }

    pub fn is_empty(self) -> bool {
        !self.has_any_attr()
    }
}

macro_rules! match_attr {
    ($attr:expr, {
        bold => $bold:expr,
        dim => $dim:expr,
        italic => $italic:expr,
        underlined => $underlined:expr,
        blink => $blink:expr,
        reverse => $reverse:expr,
        hidden => $hidden:expr
    } => $($apply:tt)*) => {
        if $attr.bold() {
            $($apply)*($bold)
        }

        if $attr.dim() {
            $($apply)*($dim)
        }

        if $attr.italic() {
            $($apply)*($italic)
        }

        if $attr.underlined() {
            $($apply)*($underlined)
        }

        if $attr.blink() {
            $($apply)*($blink)
        }

        if $attr.reverse() {
            $($apply)*($reverse)
        }

        if $attr.hidden() {
            $($apply)*($hidden)
        }
    };
}

impl Display for Attributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out: Vec<&'static str> = vec![];

        match_attr!(self, {
            bold => "bold",
            dim => "dim",
            italic => "italic",
            underlined => "underlined",
            blink => "blink",
            reverse => "reverse",
            hidden => "hidden"
        } => out.push);

        write!(f, "{}", itertools::join(out, ", "))
    }
}

impl IntoIterator for Attributes {
    type Item = console::Attribute;
    type IntoIter = AttributesIterator;

    fn into_iter(self) -> Self::IntoIter {
        AttributesIterator {
            iterating: self,
            seen: Attributes::new(),
        }
    }
}

impl BitOr for Attributes {
    type Output = Attributes;

    fn bitor(self, rhs: Self) -> Self::Output {
        let bytes = u8::from(self) | u8::from(rhs);
        Attributes::from(bytes)
    }
}

impl BitOrAssign for Attributes {
    fn bitor_assign(&mut self, rhs: Self) {
        let bytes = u8::from(*self) | u8::from(rhs);
        *self = Attributes::from(bytes)
    }
}

impl From<console::Attribute> for Attributes {
    fn from(attr: console::Attribute) -> Attributes {
        match attr {
            console::Attribute::Bold => Attributes::new().with_bold(true),
            console::Attribute::Dim => Attributes::new().with_dim(true),
            console::Attribute::Italic => Attributes::new().with_italic(true),
            console::Attribute::Underlined => Attributes::new().with_underlined(true),
            console::Attribute::Blink => Attributes::new().with_blink(true),
            console::Attribute::Reverse => Attributes::new().with_reverse(true),
            console::Attribute::Hidden => Attributes::new().with_hidden(true),
        }
    }
}

pub struct AttributesIterator {
    seen: Attributes,
    iterating: Attributes,
}

macro_rules! iterate_attr {
    ($self:ident, $field:ident, $set_field:ident, $attr:ident) => {
        if $self.seen.$field() == false {
            $self.seen.$set_field(true);
            if $self.iterating.bold() {
                return Some(console::Attribute::$attr);
            }
        }
    };
}

impl Iterator for AttributesIterator {
    type Item = console::Attribute;

    fn next(&mut self) -> Option<Self::Item> {
        iterate_attr!(self, bold, set_bold, Bold);
        iterate_attr!(self, dim, set_dim, Dim);
        iterate_attr!(self, italic, set_italic, Bold);
        iterate_attr!(self, underlined, set_underlined, Underlined);
        iterate_attr!(self, blink, set_blink, Blink);
        iterate_attr!(self, reverse, set_reverse, Reverse);
        iterate_attr!(self, hidden, set_hidden, Hidden);

        None
    }
}

#[derive(Copy, Clone, new)]
pub struct Style {
    #[new(value = "None")]
    fg: Option<console::Color>,
    #[new(value = "None")]
    bg: Option<console::Color>,
    #[new(default)]
    attrs: Attributes,
}

impl From<console::Color> for Style {
    fn from(color: console::Color) -> Self {
        Style::default().fg(color)
    }
}

impl From<Attributes> for Style {
    fn from(attr: Attributes) -> Self {
        Style::default().attr(attr)
    }
}

impl From<console::Attribute> for Style {
    fn from(attr: console::Attribute) -> Self {
        Style::default().attrs(Attributes::from(attr))
    }
}

impl Default for Style {
    fn default() -> Self {
        Style::new()
    }
}

impl Style {
    pub fn apply_to<D>(self, fragment: D) -> console::StyledObject<D> {
        let style: console::Style = self.into();
        style.apply_to(fragment)
    }

    pub fn fg(mut self, color: impl Into<console::Color>) -> Style {
        self.fg = Some(color.into());
        self
    }

    pub fn bg(mut self, color: impl Into<console::Color>) -> Style {
        self.bg = Some(color.into());
        self
    }

    pub fn attr(mut self, attr: impl Into<Attributes>) -> Style {
        self.attrs |= attr.into();
        self
    }

    pub fn attrs(mut self, attrs: impl Into<Attributes>) -> Style {
        self.attrs |= attrs.into();
        self
    }
}

impl From<Style> for console::Style {
    fn from(style: Style) -> Self {
        let mut console_style = console::Style::new();

        if let Some(fg) = style.fg {
            console_style = console_style.fg(fg);
        }

        if let Some(bg) = style.bg {
            console_style = console_style.bg(bg);
        }

        for attr in style.attrs {
            console_style = console_style.attr(attr);
        }

        console_style
    }
}

impl Debug for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { fg, bg, attrs } = self;

        let mut desc = String::new();

        match (fg, bg) {
            (Some(fg), None) => {
                desc.push_str(&format!("{:?}", fg));
            }
            (None, Some(bg)) => {
                desc.push_str(&format!("normal on {:?}", bg));
            }
            (None, None) => {
                desc.push_str("normal");
            }
            (Some(fg), Some(bg)) => {
                desc.push_str(&format!("{:?} on {:?}", fg, bg));
            }
        }

        write!(f, "{}{}", desc, attrs)
    }
}
