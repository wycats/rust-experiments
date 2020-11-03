use std::fmt::Display;

pub trait ToStyledString {
    fn to_styled_string(&self) -> String;
}

impl<T> ToStyledString for T
where
    T: Display,
{
    fn to_styled_string(&self) -> String {
        self.to_string()
    }
}
