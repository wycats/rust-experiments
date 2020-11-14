//! Wrap console::Attribute and console::Style to tweak their behavior (especially debug output)

use derive_new::new;
use std::{fmt::Debug, hash::Hash};

use indexmap::IndexSet;

#[derive(Copy, Clone)]
pub struct Attribute {
    attr: console::Attribute,
}

impl Debug for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.attr {
                console::Attribute::Bold => "bold",
                console::Attribute::Dim => "dim",
                console::Attribute::Italic => "italic",
                console::Attribute::Underlined => "underlined",
                console::Attribute::Blink => "blink",
                console::Attribute::Reverse => "reverse",
                console::Attribute::Hidden => "hidden",
            }
        )
    }
}

impl PartialOrd for Attribute {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.attr.partial_cmp(&other.attr)
    }
}

impl Ord for Attribute {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.attr.cmp(&other.attr)
    }
}

impl PartialEq for Attribute {
    fn eq(&self, other: &Self) -> bool {
        self.attr == other.attr
    }
}

impl Eq for Attribute {}

impl Hash for Attribute {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self.attr {
            console::Attribute::Bold => 0.hash(state),
            console::Attribute::Dim => 1.hash(state),
            console::Attribute::Italic => 2.hash(state),
            console::Attribute::Underlined => 3.hash(state),
            console::Attribute::Blink => 4.hash(state),
            console::Attribute::Reverse => 5.hash(state),
            console::Attribute::Hidden => 6.hash(state),
        }
    }
}

impl Into<Attribute> for console::Attribute {
    fn into(self) -> Attribute {
        Attribute { attr: self }
    }
}

#[derive(Clone, new)]
pub struct Style {
    #[new(value = "None")]
    fg: Option<console::Color>,
    #[new(value = "None")]
    bg: Option<console::Color>,
    #[new(default)]
    attrs: IndexSet<Attribute>,
}

impl From<console::Color> for Style {
    fn from(color: console::Color) -> Self {
        Style::default().fg(color)
    }
}

impl From<Attribute> for Style {
    fn from(attr: Attribute) -> Self {
        Style::default().attr(attr)
    }
}

impl From<console::Attribute> for Style {
    fn from(attr: console::Attribute) -> Self {
        Style::default().attr(Attribute { attr })
    }
}

impl Default for Style {
    fn default() -> Self {
        Style::new()
    }
}

impl Style {
    pub fn apply_to<D>(&self, fragment: D) -> console::StyledObject<D> {
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

    pub fn attr(mut self, attr: impl Into<Attribute>) -> Style {
        self.attrs.insert(attr.into());
        self
    }
}

impl<'a> From<&'a Style> for console::Style {
    fn from(style: &'a Style) -> Self {
        let style = style.clone();
        style.into()
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
            console_style = console_style.attr(attr.attr);
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

        let mut debug_attrs = String::new();

        for attr in itertools::sorted(attrs.iter()) {
            debug_attrs.push(',');

            debug_attrs.push_str(&format!("{:?}", attr));
        }

        write!(f, "{}{}", desc, debug_attrs)
    }
}
