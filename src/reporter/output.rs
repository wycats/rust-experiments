use std::{
    fmt::Display,
    io::{stderr, stdout, Write},
};

use laboratory_expectations::{emit::write::StyledEmitterTrait, emitter, EmitWriter, Style};

use crate::suite::DurationWithPrecision;

use super::ReportResult;

macro_rules! out {
    ($out:expr, $($token:tt)*) => {
        write!($out, $($token)*)?;
    };
}

macro_rules! outln {
    ($out:expr) => {{
        use laboratory_expectations::StyledEmitterTrait;
        laboratory_expectations::emitln!(&mut $out.boxed())?;
    }};

    ($out:expr, $($tokens:tt)*) => {{
        use laboratory_expectations::StyledEmitterTrait;

        $out.start_line()?;
        laboratory_expectations::emitln!(&mut $out.boxed(), $($tokens)*)?;
    }};

    (+1 => $out:expr, $($tokens:tt)*) => {{
        use laboratory_expectations::StyledEmitterTrait;

        $out.indent();
        $out.start_line()?;
        laboratory_expectations::emitln!(&mut $out.boxed(), $($tokens)*)?;
        $out.outdent();
    }}
}

#[derive(Debug)]
pub struct ReporterOutput<'a> {
    write: EmitWriter<'a>,
    indent: usize,
}

impl<'a> ReporterOutput<'a> {
    pub fn child(&mut self) -> ReporterOutput<'_> {
        ReporterOutput {
            write: self.write.child(),
            indent: self.indent,
        }
    }

    pub fn write(write: &'a mut dyn std::fmt::Write) -> ReporterOutput<'a> {
        ReporterOutput {
            write: EmitWriter::borrowed_fmt(write),
            indent: 0,
        }
    }

    pub fn null() -> ReporterOutput<'static> {
        ReporterOutput {
            write: EmitWriter::owned_io(vec![]),
            indent: 0,
        }
    }

    pub fn buffer(s: &'a mut dyn std::io::Write) -> ReporterOutput<'a> {
        ReporterOutput {
            write: EmitWriter::borrowed_io(s as &mut dyn std::io::Write),
            indent: 0,
        }
    }

    pub fn stdout() -> ReporterOutput<'static> {
        ReporterOutput {
            write: EmitWriter::owned_io(stdout()),
            indent: 0,
        }
    }

    pub fn stderr() -> ReporterOutput<'static> {
        ReporterOutput {
            write: EmitWriter::owned_io(stderr()),
            indent: 0,
        }
    }

    pub fn for_suite(&mut self, count: usize) -> SuiteOutput<'_> {
        SuiteOutput {
            write: self.write.child(),
            indent: self.indent,
            count_size: format!("{}", count).len(),
            count,
        }
    }
}

#[derive(Debug)]
pub struct SuiteOutput<'a> {
    write: EmitWriter<'a>,
    indent: usize,
    count_size: usize,
    count: usize,
}

impl<'a> SuiteOutput<'a> {
    pub fn child(&mut self) -> SuiteOutput<'_> {
        SuiteOutput {
            write: self.write.child(),
            indent: self.indent,
            count_size: self.count_size,
            count: self.count,
        }
    }

    pub fn for_nested(&mut self) -> ReporterOutput<'_> {
        ReporterOutput {
            write: self.write.child(),
            indent: self.indent,
        }
    }

    pub fn blank_line(&mut self) -> ReportResult {
        Ok(writeln!(self.write)?)
    }

    pub fn next_line(&mut self) -> ReportResult {
        writeln!(self.write)?;
        self.start_line()
    }

    pub fn start_line(&mut self) -> ReportResult {
        // start with a bit of indent no matter what
        let mut indent = "  ".to_string();

        for _ in 0..self.indent {
            indent.push_str("  ");
        }

        Ok(write!(self.write, "{}", indent)?)
    }

    pub fn enumerate(&self, num: usize) -> String {
        let total_size = self.count.to_string().len();

        format!("{:>width$}", num, width = total_size)
    }

    pub fn duration(&mut self, duration: DurationWithPrecision) -> ReportResult {
        Ok(write!(self.write, "{}", duration)?)
    }

    pub fn indent(&mut self) {
        self.indent += 1;
    }

    pub fn outdent(&mut self) {
        self.indent -= 1;
    }

    pub fn nest(&mut self, nesting: usize) {
        self.indent = nesting;
    }

    pub fn next(self) -> ReporterOutput<'a> {
        ReporterOutput {
            write: self.write,
            indent: self.indent,
        }
    }

    pub fn fatal_error(&mut self, error: impl Display) {
        write!(self.write, "{}", error)
            .unwrap_or_else(|_| panic!("Failed to print fatal error: {}", error))
    }
}

impl<'a> std::fmt::Write for SuiteOutput<'a> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        write!(self.write, "{}", s).map_err(|_| std::fmt::Error)
    }
}
