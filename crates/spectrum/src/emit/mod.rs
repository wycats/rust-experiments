#[macro_use]
mod macros;

pub mod buf;
pub mod error;
pub mod fragment;
pub mod into;
pub mod style;
pub mod test;

// pub mod write;

// ///
// /// StyledFragment represents a piece of content that could be emitted in color or plain
// /// StyledEmitter wraps some kind of Writer, and asks the StyledFragment to emit color or plain
// ///

// pub trait StyledFragmentTrait {
//     fn emit_into(&self, writer: &mut StyledEmitter) -> std::io::Result<()>;

//     fn boxed_fragment(self, style: Option<Style>) -> StyledFragment;
// }

// pub struct StyledFragment {
//     fragment: Box<dyn StyledFragmentTrait>,
// }

// impl<'a, T> StyledFragmentTrait for T
// where
//     T: Debug,
// {
//     fn emit_into(&self, writer: &mut StyledEmitter) -> std::io::Result<()> {
//         writer.emit(&format!("{:?}", self), Style::default())
//     }

//     fn boxed_fragment(self, style: Option<Style>) -> StyledFragment {
//         match style {
//             Some(style) => StyledFragment {
//                 fragment: Box::new(Styled::new(format!("{:?}", self), style)),
//             },
//             None => StyledFragment {
//                 fragment: Box::new(Styled::new(format!("{:?}", self), Style::default())),
//             },
//         }
//     }
// }

// impl StyledFragmentTrait for StyledFragment {
//     fn emit_into(&self, writer: &mut StyledEmitter) -> std::io::Result<()> {
//         self.fragment.emit_into(writer)
//     }

//     fn boxed_fragment(self, _style: Option<Style>) -> StyledFragment {
//         self
//     }
// }

// impl<'a> StyledFragment {
//     pub fn new(fragment: impl StyledFragmentTrait + 'static) -> StyledFragment {
//         StyledFragment {
//             fragment: Box::new(fragment),
//         }
//     }

//     pub fn emit_into(&self, writer: &mut StyledEmitter) -> std::io::Result<()> {
//         self.fragment.emit_into(writer)
//     }
// }

// use ansi_term::Style;

// use self::write::StyledEmitter;
// #[derive(new)]
// pub struct Styled {
//     fragment: String,
//     style: Style,
// }

// impl StyledFragmentTrait for Styled {
//     fn emit_into(&self, writer: &mut StyledEmitter) -> std::io::Result<()> {
//         writer.emit(&self.fragment, self.style)
//     }

//     fn boxed_fragment(self, _style: Option<Style>) -> StyledFragment {
//         StyledFragment {
//             fragment: Box::new(self),
//         }
//     }
// }

// impl<'a> Styled {
//     fn into_line(self) -> StyledLine {
//         StyledLine::new(vec![self.boxed_fragment(None)])
//     }

//     #[doc(hidden)]
//     pub fn for_emit(self) -> StyledBlock {
//         let line: StyledLine = self.into_line();
//         line.for_emit()
//     }

//     #[doc(hidden)]
//     pub fn for_emitln(self) -> StyledBlock {
//         let line: StyledLine = self.into_line();
//         line.for_emitln()
//     }
// }

// #[macro_export]
// macro_rules! frag {
//     // ({ $($tokens:tt)* }) => {
//     //     frag!($($tokens)*)
//     // };

//     ($color:ident : $str:expr) => {{
//         use $crate::formatting::Inspect;
//         $crate::emit::Styled::new($str.inspect(), $color.into()).boxed_fragment(Some($color.into()))
//     }};

//     ([$fg:ident $(, $attr:ident)* : $str:expr ]) => {{
//         use $crate::formatting::Inspect;

//         #[allow(unused_mut)]
//         let mut style = $fg.normal();

//         $(
//             style = style.$attr();
//         )*

//         let frag = Styled {
//             fragment: $str.inspect(),
//             style,
//         };

//         frag.boxed_fragment(None)
//     }};

//     ([$fg:ident on $bg:ident $(, $attr:ident)* : $str:expr ]) => {{
//         use $crate::formatting::Inspect;

//         #[allow(unused_mut)]
//         let mut style = $crate::ansi_term::Color::$fg.on($crate::ansi_term::Color::$bg);

//         $(
//             style = style.$attr();
//         )*

//         let frag = Styled::new($str.inspect(), style);

//         frag.boxed_fragment(None)
//     }};

//     ($str:expr) => {{
//         $str.boxed_fragment(None)
//     }};
// }

// pub fn styled(fragment: impl Into<String>, style: impl Into<Style>) -> Styled {
//     Styled {
//         fragment: fragment.into(),
//         style: style.into(),
//     }
// }

// #[derive(new)]
// pub struct StyledLine {
//     fragments: Vec<StyledFragment>,
// }

// impl<'a> StyledFragmentTrait for StyledLine {
//     fn emit_into(&self, writer: &mut StyledEmitter) -> std::io::Result<()> {
//         for fragment in self.fragments.iter() {
//             fragment.emit_into(writer)?;
//         }

//         Ok(())
//     }

//     fn boxed_fragment(self, _style: Option<Style>) -> StyledFragment {
//         StyledFragment {
//             fragment: Box::new(self),
//         }
//     }
// }

// impl<'a> StyledLine {
//     fn into_item(self) -> StyledItem {
//         StyledItem::Line(self)
//     }

//     pub fn for_emit(self) -> StyledBlock {
//         StyledBlock::new(vec![self.into_item()])
//     }

//     pub fn for_emitln(self) -> StyledBlock {
//         StyledBlock::new(vec![self.into_item(), StyledItem::Newline])
//     }
// }

// #[macro_export]
// macro_rules! inline {
//     ({ result = { $($result:tt)* } rest = {} }) => {
//         $crate::emit::StyledLine::new($($result)*)
//     };

//     ({ result = { $($result:tt)* } rest = { $($tokens:tt)+ } }) => {
//         compile_error!($($tokens)+)
//     };

//     (head = { $({ $($head:tt)* })* } tail = {}) => {
//         $crate::inline!({ result = { vec![$($($head)*),*] } rest = {} })
//     };

//     (head = { $({ $($head:tt)* })* } tail = { ; $($rest:tt)* }) => {
//         $crate::inline!(result = { vec![$($($head)*),*] } rest = { $($rest)* })
//     };

//     (head = { $($head:tt)* } tail = { [$color:ident : $str:expr] $($rest:tt)* }) => {
//         $crate::inline!(head = { $($head)* { $crate::frag!($color : $str) } } tail = { $($rest)* })
//     };

//     (head = { $($head:tt)* } tail = { [$fg:ident $(, $attr:ident)* : $str:expr ] $($rest:tt)* }) => {
//         $crate::inline!(head = { $($head)* { $crate::frag!([$fg $(, $attr)* : $str]) } } tail = { $($rest)* })
//     };

//     (head = { $($head:tt)* } tail = { [$fg:ident on $bg:ident $(, $attr:ident)* : $str:expr ] $($rest:tt)* }) => {
//         $crate::inline!(head = { $($head)* { $crate::frag!([$fg on $bg $(, $attr)* : $str]) } } tail = { $($rest)* })
//     };

//     (head = { $($head:tt)* } tail = { [$str:expr] $($rest:tt)* }) => {
//         $crate::inline!(head = { $($head)* { $crate::frag!($str) } } tail = { $($rest)* })
//     };

//     (head = { $($head:tt)* } tail = { $str:tt $($rest:tt)* }) => {
//         $crate::inline!(head = { $($head)* { $crate::frag!($str) } } tail = { $($rest)* })
//     };

//     ($($tokens:tt)*) => {
//         $crate::inline!(head = {} tail = { $($tokens)* })
//     };

//     () => {{

//     }}
// }

// pub fn line(fragments: Vec<StyledFragment>) -> StyledLine {
//     StyledLine { fragments }
// }

// pub struct Newline;

// impl Into<StyledItem> for Newline {
//     fn into(self) -> StyledItem {
//         StyledItem::Newline
//     }
// }

// impl<'a> Into<StyledItem> for StyledLine {
//     fn into(self) -> StyledItem {
//         StyledItem::Line(self)
//     }
// }

// pub enum StyledItem {
//     Newline,
//     Line(StyledLine),
// }

// impl StyledFragmentTrait for StyledItem {
//     fn emit_into(&self, emitter: &mut StyledEmitter) -> std::io::Result<()> {
//         match self {
//             StyledItem::Newline => emitter.emit("\n", Style::default())?,
//             StyledItem::Line(line) => line.emit_into(emitter)?,
//         }

//         Ok(())
//     }

//     fn boxed_fragment(self, _style: Option<Style>) -> StyledFragment {
//         StyledFragment {
//             fragment: Box::new(self),
//         }
//     }
// }

// #[macro_export]
// macro_rules! block {
//     ($($tokens:tt)*) => {
//         $crate::block_parts!(head = {} tail = { $($tokens)* })
//     };
// }

// #[macro_export]
// macro_rules! block_parts {
//     // last
//     (head = { $($head:tt)* } tail = { $token:tt }) => {
//         $crate::emit::StyledBlock::new(vec![$($head,)* $crate::emit::StyledItem::Line($crate::inline!($token))])
//     };

//     (head = { $($head:tt)* } tail = { [ $($line:tt)* ] }) => {
//         $crate::emit::StyledBlock::new(vec![$($head,)* $crate::emit::StyledItem::Line($crate::inline!([$($line)*]))])
//     };

//     // (head = { $($head:tt)* } tail = { [$($line:tt)*] }) => {
//     //     $crate::emit::StyledBlock::new(vec![$($head,)* $crate::emit::StyledItem::Line($crate::inline!([$($line)*]))])
//     // };

//     // // last
//     // (head = { $($head:tt)* } tail = { [$($line:tt)*] }) => {
//     //     $crate::emit::StyledBlock::new(vec![$($head,)* $crate::emit::StyledItem::Line($crate::inline!($($line)*))])
//     // };

//     // middle
//     (head = { $($head:tt)* } tail = { ; $($rest:tt)* } ) => {
//         $crate::block_parts!(head = { $($head)* { $crate::emit::StyledItem::Newline } } tail = { $($rest)* })
//     };

//     (head = { $($head:tt)* } tail = { [ $($line:tt)* ] $($rest:tt)* } ) => {
//         $crate::block_parts!(head = { $($head)* { $crate::emit::StyledItem::Line(inline!([$($line)*])) } } tail = { $($rest)* })
//     };

//     // middle
//     (head = { $($head:tt)* } tail = { $line:tt $($rest:tt)* } ) => {
//         $crate::block_parts!(head = { $($head)* { $crate::emit::StyledItem::Line($crate::inline!($line)) }  } tail = { $($rest)* })
//     };

// }

// #[derive(new)]
// pub struct StyledBlock {
//     items: Vec<StyledItem>,
// }

// impl StyledFragmentTrait for StyledBlock {
//     fn emit_into(&self, writer: &mut StyledEmitter) -> std::io::Result<()> {
//         for item in self.items.iter() {
//             item.emit_into(writer)?;
//         }

//         Ok(())
//     }

//     fn boxed_fragment(self, _style: Option<Style>) -> StyledFragment {
//         StyledFragment {
//             fragment: Box::new(self),
//         }
//     }
// }

// impl<'a> StyledBlock {
//     pub fn newline() -> StyledBlock {
//         StyledBlock {
//             items: vec![StyledItem::Newline],
//         }
//     }

//     #[doc(hidden)]
//     pub fn for_emit(self) -> StyledBlock {
//         self
//     }

//     #[doc(hidden)]
//     pub fn for_emitln(mut self) -> StyledBlock {
//         let last = self.items.last();

//         if let Some(StyledItem::Newline) = last {
//             self
//         } else {
//             self.items.extend(Some(StyledItem::Newline));
//             self
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::Styled;

//     use super::{write::StyledEmitterTrait, *};
//     use ansi_term::Style;
//     use derive_new::new;

//     #[derive(new)]
//     pub struct TestEmit {
//         #[new(default)]
//         string: String,
//     }

//     impl TestEmit {
//         fn done(self) -> String {
//             self.string
//         }
//     }

//     impl StyledEmitterTrait for TestEmit {
//         fn emit(&mut self, fragment: &str, style: Style) -> std::io::Result<()> {
//             if fragment == "\n" {
//                 self.string.push('\n');
//                 return Ok(());
//             }

//             self.string.push_str(&format!("{}[", debug_style(style)));
//             self.string.push_str(&fragment);
//             self.string.push(']');

//             Ok(())
//         }
//     }

//     fn debug_style(style: Style) -> String {
//         let mut desc = String::new();
//         let mut short = true;

//         match (style.foreground, style.background) {
//             (Some(fg), None) => {
//                 desc.push_str(&format!("{:?}", fg));
//             }
//             (None, Some(bg)) => {
//                 desc.push_str(&format!("normal on {:?}", bg));
//                 short = false;
//             }
//             (None, None) => {
//                 desc.push_str("normal");
//             }
//             (Some(fg), Some(bg)) => {
//                 desc.push_str(&format!("{:?} on {:?}", fg, bg));
//                 short = false;
//             }
//         }

//         let mut attrs = String::new();

//         if style.is_bold {
//             attrs.push_str(", bold");
//         }

//         if style.is_dimmed {
//             attrs.push_str(", dimmed");
//         }

//         if style.is_italic {
//             attrs.push_str(", italic");
//         }

//         if style.is_underline {
//             attrs.push_str(", underline");
//         }

//         if style.is_blink {
//             attrs.push_str(", blink");
//         }

//         if style.is_reverse {
//             attrs.push_str(", reverse");
//         }

//         if style.is_hidden {
//             attrs.push_str(", hidden");
//         }

//         if style.is_strikethrough {
//             attrs.push_str(", strikethrough");
//         }

//         if !attrs.is_empty() {
//             short = false;
//         }

//         if short {
//             desc
//         } else {
//             format!("[{}{}]", desc, attrs)
//         }
//     }

//     fn emit(frag: impl StyledFragmentTrait) -> std::io::Result<String> {
//         let mut emitter = TestEmit::new();
//         let mut styled_emitter = StyledEmitter::borrowed(&mut emitter);
//         frag.emit_into(&mut styled_emitter)?;
//         styled_emitter.done();

//         Ok(emitter.done())
//     }

//     #[test]
//     fn fragment() -> std::io::Result<()> {
//         use ansi_term::Color::*;

//         assert_eq!(emit(frag!(Red:"fragment"))?, "Red[fragment]");
//         assert_eq!(
//             emit(frag!([Red on Blue:"fragment"]))?,
//             "[Red on Blue][fragment]"
//         );
//         assert_eq!(
//             emit(frag!([Red on Blue, bold, blink:"fragment"]))?,
//             "[Red on Blue, bold, blink][fragment]"
//         );

//         assert_eq!(emit(frag!("fragment"))?, "normal[fragment]");

//         Ok(())
//     }

//     #[test]
//     fn line() -> std::io::Result<()> {
//         use ansi_term::Color::*;

//         assert_eq!(emit(inline!([Red:"fragment"]))?, "Red[fragment]");

//         assert_eq!(
//             emit(inline!([Red: "fragment"] [Red on Blue:"fragment"]))?,
//             "Red[fragment][Red on Blue][fragment]"
//         );

//         assert_eq!(
//             emit(inline!([Red on Blue: "fragment"]))?,
//             "[Red on Blue][fragment]"
//         );

//         assert_eq!(
//             emit(inline!([Red on Blue: "fragment"] ["hello"]))?,
//             "[Red on Blue][fragment]normal[hello]"
//         );

//         assert_eq!(
//             emit(inline!([Red on Blue, bold, blink: "fragment"]))?,
//             "[Red on Blue, bold, blink][fragment]"
//         );

//         assert_eq!(
//             emit(
//                 inline!([Red on Blue, bold, blink: "fragment"] [Red: "fragment2"] "hello" [Cyan: "world"])
//             )?,
//             "[Red on Blue, bold, blink][fragment]Red[fragment2]normal[hello]Cyan[world]"
//         );

//         assert_eq!(emit(inline!("fragment"))?, "normal[fragment]");

//         Ok(())
//     }

//     #[test]
//     fn block() -> std::io::Result<()> {
//         use ansi_term::Color::*;

//         assert_eq!(emit(block!([Red:"fragment"]))?, "Red[fragment]");

//         assert_eq!(
//             emit(block!([Red:"fragment"] ; [Green:"fragment"]))?,
//             "Red[fragment]\nGreen[fragment]"
//         );

//         assert_eq!(
//             emit(block!([Red: "fragment"] [Red on Blue:"fragment"]))?,
//             "Red[fragment][Red on Blue][fragment]"
//         );

//         assert_eq!(
//             emit(block!([Red: "fragment"] [Red on Blue:"fragment"] ; [Red, reverse: "fragment"]))?,
//             "Red[fragment][Red on Blue][fragment]\n[Red, reverse][fragment]"
//         );

//         assert_eq!(
//             emit(block!([Red on Blue: "fragment"]))?,
//             "[Red on Blue][fragment]"
//         );

//         assert_eq!(
//             emit(block!([Red on Blue: "fragment"] ; [Red on White, blink: "fragment"]))?,
//             "[Red on Blue][fragment]\n[Red on White, blink][fragment]"
//         );

//         assert_eq!(
//             emit(block!([Red on Blue: "fragment"] "hello" ; "world" [Blue, dimmed: "fragment"]))?,
//             "[Red on Blue][fragment]normal[hello]\nnormal[world][Blue, dimmed][fragment]"
//         );

//         assert_eq!(
//             emit(block!([Red on Blue, bold, blink: "fragment"]))?,
//             "[Red on Blue, bold, blink][fragment]"
//         );

//         assert_eq!(
//             emit(
//                 block!([Red on Blue, bold, blink: "fragment"] [Red: "fragment2"] "hello" [Cyan: "world"])
//             )?,
//             "[Red on Blue, bold, blink][fragment]Red[fragment2]normal[hello]Cyan[world]"
//         );

//         assert_eq!(
//             emit(
//                 block!([Red on Blue, bold, blink: "fragment"] [Red: "fragment2"] "hello" [Cyan: "world"] ; [Red: "fragment3"])
//             )?,
//             "[Red on Blue, bold, blink][fragment]Red[fragment2]normal[hello]Cyan[world]\nRed[fragment3]"
//         );

//         assert_eq!(emit(block!("fragment"))?, "normal[fragment]");

//         assert_eq!(
//             emit(block!("fragment" ; "fragment2"))?,
//             "normal[fragment]\nnormal[fragment2]"
//         );

//         Ok(())
//     }

//     #[test]
//     fn block_with_variables() -> std::io::Result<()> {
//         use ansi_term::Color::*;

//         let fragment = "fragment";

//         assert_eq!(emit(block!([Red: fragment]))?, "Red[fragment]");

//         assert_eq!(
//             emit(block!([Red:fragment] ; [Green:fragment]))?,
//             "Red[fragment]\nGreen[fragment]"
//         );

//         assert_eq!(
//             emit(block!([Red:fragment] [Red on Blue:fragment]))?,
//             "Red[fragment][Red on Blue][fragment]"
//         );

//         assert_eq!(
//             emit(block!([Red: fragment] [Red on Blue:fragment] ; [Red, reverse: fragment]))?,
//             "Red[fragment][Red on Blue][fragment]\n[Red, reverse][fragment]"
//         );

//         assert_eq!(
//             emit(block!([Red on Blue: fragment]))?,
//             "[Red on Blue][fragment]"
//         );

//         assert_eq!(
//             emit(block!([Red on Blue: fragment] ; [Red on White, blink: fragment] ))?,
//             "[Red on Blue][fragment]\n[Red on White, blink][fragment]"
//         );

//         assert_eq!(
//             emit(block!([Red on Blue: fragment] "hello" ; "world" [Blue, dimmed: fragment]))?,
//             "[Red on Blue][fragment]normal[hello]\nnormal[world][Blue, dimmed][fragment]"
//         );

//         assert_eq!(
//             emit(block!([Red on Blue, bold, blink: fragment]))?,
//             "[Red on Blue, bold, blink][fragment]"
//         );

//         assert_eq!(
//             emit(
//                 block!([Red on Blue, bold, blink: fragment] [Red: "fragment2"] "hello" [Cyan: "world"])
//             )?,
//             "[Red on Blue, bold, blink][fragment]Red[fragment2]normal[hello]Cyan[world]"
//         );

//         assert_eq!(
//             emit(
//                 block!([Red on Blue, bold, blink: fragment] [Red: "fragment2"] "hello" [Cyan: "world"] ; [Red: "fragment3"])
//             )?,
//             "[Red on Blue, bold, blink][fragment]Red[fragment2]normal[hello]Cyan[world]\nRed[fragment3]"
//         );

//         assert_eq!(emit(block!(fragment))?, "normal[fragment]");

//         assert_eq!(
//             emit(block!(fragment ; "fragment2"))?,
//             "normal[fragment]\nnormal[fragment2]"
//         );

//         Ok(())
//     }
// }
