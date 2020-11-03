pub mod diff;
pub mod inline;
pub mod traits;

use format::lazy_format;
use std::fmt::{Debug, Display, Formatter};

use ansi_term::{Color, Style};

pub enum Semantic {
    Actual,
    Expected,
    Match,
    Relationship,
}

impl Into<Style> for Semantic {
    fn into(self) -> Style {
        match self {
            Semantic::Actual => Color::Cyan.normal(),
            Semantic::Expected => Color::Purple.normal(),
            Semantic::Match => Color::Green.normal(),
            Semantic::Relationship => Style::default(),
        }
    }
}

pub trait Inspect: Debug {
    fn inspect(&self) -> String {
        format!("{:?}", self)
    }
}

impl<T> Inspect for T where T: Debug {}
