use std::{cmp::Ordering, fmt::Display, iter::Peekable, str::MatchIndices};

use ansi_term::{ANSIGenericString, Style};
use difference::{Changeset, Difference};

#[doc(hidden)]
#[derive(Debug)]
pub enum StyledLinePart<'a> {
    FirstLine(ANSIGenericString<'a, str>),
    Line(ANSIGenericString<'a, str>),
    Newline(ANSIGenericString<'a, str>),
}

#[doc(hidden)]
#[derive(Debug)]
pub struct StyledLines<'a> {
    string: &'a str,
    last_index: usize,
    indices: Peekable<MatchIndices<'a, char>>,
    style: Style,
}

impl<'a> Iterator for StyledLines<'a> {
    type Item = StyledLinePart<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let is_first = self.last_index == 0;
        let (consume, value) = match self.indices.peek() {
            Some((index, join)) => {
                let prev = &self.string[self.last_index..*index];

                if prev.is_empty() {
                    self.last_index = index + join.len();
                    (
                        true,
                        Some(StyledLinePart::Newline(Style::default().paint(*join))),
                    )
                } else {
                    self.last_index = *index;

                    (
                        false,
                        if is_first {
                            Some(StyledLinePart::FirstLine(self.style.paint(prev)))
                        } else {
                            Some(StyledLinePart::Line(self.style.paint(prev)))
                        },
                    )
                }
            }
            None => {
                let remainder = &self.string[self.last_index..];

                if remainder.is_empty() {
                    (false, None)
                } else {
                    self.last_index = self.string.len();

                    (
                        false,
                        if is_first {
                            Some(StyledLinePart::FirstLine(self.style.paint(remainder)))
                        } else {
                            Some(StyledLinePart::Line(self.style.paint(remainder)))
                        },
                    )
                }
            }
        };

        if consume {
            self.indices.next();
        }

        value
        // let next_index = indices.next();
    }
}

#[doc(hidden)]
pub struct Styled {
    string: String,
    style: Style,
}

impl Styled {
    #[doc(hidden)]
    pub fn new(string: impl Into<String>, style: impl Into<Style>) -> Styled {
        Styled {
            string: string.into(),
            style: style.into(),
        }
    }

    fn lines(&self) -> StyledLines<'_> {
        StyledLines {
            string: &self.string,
            last_index: 0,
            indices: self.string.match_indices('\n').peekable(),
            style: self.style,
        }

        // let style = self.style;
        // let lines = self.string.split('\n').peekable();

        // .map(move |line| style.paint(line))
        // .peekable()
    }
}

impl Display for Styled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.style.paint(&self.string))
    }
}

/// A macro that helps identify stray printlns
#[doc(hidden)]
#[macro_export]
macro_rules! traceln {
    (true, $($tok:tt)*) => {
        println!($($tok)*)
    };

    ($($tok:tt)*) => {
    }
}

#[doc(hidden)]
pub struct Printer;

impl Printer {
    #[doc(hidden)]
    pub fn lines<'a>(self, text: &'a Styled) -> (Vec<ANSIGenericString<'a, str>>, bool) {
        traceln!("LINES FOR {:?}", text.string);

        let mut out: Vec<ANSIGenericString<'a, str>> = vec![];

        for line in text.lines() {
            traceln!("{:#?}", line);

            match line {
                StyledLinePart::Line(line) | StyledLinePart::Newline(line) => {
                    out.push(line);
                }
                StyledLinePart::FirstLine(line) => {
                    out.push(line);
                }
            }
        }

        if let Some(last) = out.last() {
            if last.ends_with('\n') {
                return (out, true);
            }
        }

        (out, false)
    }
}

/// For some reason, the difference library can produce `Add(x), Same(""), Rem(x)` as a sequence of
/// diffs. Remove this sequence.
fn fix_diffs(diffs: Vec<Difference>, sep: &str) -> Vec<Difference> {
    let mut prev_len = diffs.len();
    let mut current_diffs = diffs;

    loop {
        let mut out = vec![];

        for diff in current_diffs {
            // println!("<- out={:#?}, diff={:?}", out, diff);
            match diff {
                Difference::Same(v) if v.is_empty() => {}
                Difference::Same(same) => match out.pop() {
                    Some(Difference::Same(mut prev)) => {
                        prev.push_str(sep);
                        prev.push_str(&same);
                        out.push(Difference::Same(prev))
                    }
                    Some(prev) => {
                        out.push(prev);
                        out.push(Difference::Same(same.clone()));
                    }
                    None => {
                        out.push(Difference::Same(same.clone()));
                    }
                },
                Difference::Add(add) => match out.pop() {
                    Some(Difference::Rem(prev)) if add[..] == prev[..] => {
                        out.push(Difference::Same(prev));
                    }
                    Some(prev) => {
                        out.push(prev);
                        out.push(Difference::Add(add.clone()));
                    }
                    None => {
                        out.push(Difference::Add(add.clone()));
                    }
                },
                Difference::Rem(rem) => match out.pop() {
                    Some(Difference::Add(prev)) if rem[..] == prev[..] => {
                        out.push(Difference::Same(prev));
                    }
                    Some(prev) => {
                        out.push(prev);
                        out.push(Difference::Rem(rem.clone()));
                    }
                    None => {
                        out.push(Difference::Rem(rem.clone()));
                    }
                },
            }
            traceln!("-> out={:#?}", out);
        }

        match out.len().cmp(&prev_len) {
            Ordering::Equal => return out,
            Ordering::Greater => {
                panic!("Somehow, retrying the cleanup step increased the number of diffs. This shouldn't happen");
            }
            Ordering::Less => {}
        }

        prev_len = out.len();
        current_diffs = out;

        traceln!("len={}, new_len={}", len, out.len());
    }
}

#[doc(hidden)]
pub fn ideal_diff(left: &str, right: &str, sep: &str) -> Vec<Difference> {
    let Changeset { diffs: diff1, .. } = Changeset::new(left, right, sep);
    let Changeset { diffs: diff2, .. } = Changeset::new(right, left, sep);

    let diff1 = fix_diffs(diff1, sep);
    let diff2 = fix_diffs(diff2, sep);

    if diff1.len() <= diff2.len() {
        return diff1;
    }

    let mut flipped = vec![];

    for diff in diff2 {
        match diff {
            same @ Difference::Same(_) => {
                flipped.push(same);
            }
            Difference::Add(add) => flipped.push(Difference::Rem(add.clone())),
            Difference::Rem(rem) => {
                flipped.push(Difference::Add(rem.clone()));
            }
        }
    }

    flipped
}

#[doc(hidden)]
pub fn show_invisibles(diffs: Vec<Difference>) -> Vec<Difference> {
    let mut out = vec![];

    for diff in diffs {
        match diff {
            same @ Difference::Same(_) => {
                out.push(same);
            }
            Difference::Add(s) => {
                for line in s.split('\n') {
                    if !line.is_empty() {
                        out.push(Difference::Add(line.to_string()));
                    }
                    out.push(Difference::Add("⏎\n".to_string()));
                }
            }
            Difference::Rem(s) => {
                for line in s.split('\n') {
                    if !line.is_empty() {
                        out.push(Difference::Rem(line.to_string()));
                    }
                    out.push(Difference::Rem("⏎\n".to_string()));
                }
            }
        }
    }

    out
}

#[macro_export]
macro_rules! assert_eq {
    ($left:expr, $right:expr) => {
        $crate::traceln!("left ::\n {}", $left);
        $crate::traceln!("right ::\n {}", $right);

        let diffs = $crate::ideal_diff(&format!("{}", $left), &format!("{}", $right), "\n");
        let diffs = $crate::show_invisibles(diffs);

        let has_diff = diffs
            .iter()
            .filter(|item| match item {
                Difference::Same(_) => false,
                _ => true,
            })
            .count()
            > 0;

        if has_diff {
            println!(
                "expected {} and {} to be equal",
                Color::Purple.paint(stringify!($left)),
                Color::Green.paint(stringify!($right))
            );

            println!("");

            $crate::traceln!("{:#?}", diffs);

            let mut diffs = diffs.iter().peekable();

            fn lines(lines: &str, style: impl Into<Style>, sep: &str, next: bool) {
                let styled = $crate::Styled::new(lines, style);
                let (lines, ends_in_newline) = $crate::Printer.lines(&styled);

                print!("{}", ANSIGenericStrings(&lines));

                if !ends_in_newline && next {
                    print!("{}", sep)
                }
            }

            $crate::traceln!("{:#?}", diffs);

            while let Some(diff) = diffs.next() {
                match diff {
                    Difference::Same(x) => {
                        lines(x, Color::White, "\n", diffs.peek().is_some());
                    }
                    Difference::Add(x) => {
                        lines(x, Color::Green, "\n", false);
                    }
                    Difference::Rem(x) if &x[..] != "⏎\n" => {
                        let next = diffs.peek();

                        match next {
                            Some(Difference::Add(y)) => {
                                diffs.next();

                                let diffs = $crate::ideal_diff(y, x, "");

                                $crate::traceln!("diffing: {:?} / {:?}", y, x);
                                $crate::traceln!("nested: {:#?}", diffs);

                                let mut diffs = diffs.iter().peekable();

                                while let Some(c) = diffs.next() {
                                    match c {
                                        Difference::Same(z) => {
                                            lines(z, Color::White, "", diffs.peek().is_some());
                                        }
                                        Difference::Add(z) => {
                                            lines(
                                                z,
                                                Color::White.on(Color::Purple),
                                                "",
                                                diffs.peek().is_some(),
                                            );
                                        }
                                        Difference::Rem(z) => {
                                            lines(
                                                z,
                                                Color::White.on(Color::Green),
                                                "",
                                                diffs.peek().is_some(),
                                            );
                                        }
                                    }
                                }
                            }
                            _ => {
                                lines(x, Color::Purple, "\n", false);
                            }
                        };
                    }

                    Difference::Rem(x) => {
                        lines(x, Color::Purple, "\n", false);
                    }
                }
            }

            panic!("expected {} == {}", stringify!($left), stringify!($right));
        }
    };
}
