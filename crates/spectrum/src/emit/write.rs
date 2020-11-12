// use std::{cell::RefCell, fmt::Arguments, io::Write, rc::Rc};

// use ansi_term::Style;
// use format::lazy_format;

// pub trait StyledEmitterTrait {
//     fn emit(&mut self, fragment: &str, style: Style) -> std::io::Result<()>;

//     fn boxed(&mut self) -> StyledEmitter<'_>
//     where
//         Self: Sized,
//     {
//         StyledEmitter::Borrowed(self)
//     }
// }

// impl<'a> StyledEmitterTrait for &'a mut dyn StyledEmitterTrait {
//     fn emit(&mut self, fragment: &str, style: Style) -> std::io::Result<()> {
//         StyledEmitterTrait::emit(&mut **self, fragment, style)
//     }
// }

// pub enum StyledEmitter<'a> {
//     Owned(Box<dyn StyledEmitterTrait + 'a>),
//     Borrowed(&'a mut dyn StyledEmitterTrait),
// }

// impl<'a, T> From<T> for StyledEmitter<'a>
// where
//     T: StyledEmitterTrait + 'a,
// {
//     fn from(value: T) -> Self {
//         StyledEmitter::new(value)
//     }
// }

// impl<'a> StyledEmitter<'a> {
//     pub fn new(emitter: impl StyledEmitterTrait + 'a) -> StyledEmitter<'a> {
//         StyledEmitter::Owned(Box::new(emitter))
//     }

//     pub fn borrowed(emitter: &'a mut dyn StyledEmitterTrait) -> StyledEmitter<'a> {
//         StyledEmitter::Borrowed(emitter)
//     }

//     pub fn styled(writer: EmitWriter<'a>) -> StyledEmitter<'a> {
//         StyledEmitter::new(ConcreteStyledEmitter::styled(writer))
//     }

//     pub fn unstyled(writer: EmitWriter<'a>) -> StyledEmitter<'a> {
//         StyledEmitter::new(ConcreteStyledEmitter::unstyled(writer))
//     }

//     pub fn emit(&mut self, fragment: &str, style: Style) -> std::io::Result<()> {
//         match self {
//             StyledEmitter::Owned(emitter) => {
//                 StyledEmitterTrait::emit(&mut **emitter, fragment, style)
//             }
//             StyledEmitter::Borrowed(emitter) => {
//                 StyledEmitterTrait::emit(&mut **emitter, fragment, style)
//             }
//         }
//     }

//     pub fn done(self) {}
// }

// pub fn emitter(writer: EmitWriter<'_>, styled: bool) -> ConcreteStyledEmitter<'_> {
//     ConcreteStyledEmitter { writer, styled }
// }

// pub struct ConcreteStyledEmitter<'a> {
//     writer: EmitWriter<'a>,
//     styled: bool,
// }

// impl<'a> ConcreteStyledEmitter<'a> {
//     pub fn styled(writer: EmitWriter<'a>) -> ConcreteStyledEmitter<'a> {
//         ConcreteStyledEmitter {
//             writer,
//             styled: true,
//         }
//     }

//     pub fn unstyled(writer: EmitWriter<'a>) -> ConcreteStyledEmitter<'a> {
//         ConcreteStyledEmitter {
//             writer,
//             styled: false,
//         }
//     }
// }

// fn map_io(err: std::fmt::Result) -> std::io::Result<()> {
//     err.map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))
// }

// impl<'a> StyledEmitterTrait for ConcreteStyledEmitter<'a> {
//     fn emit(&mut self, fragment: &str, style: Style) -> std::io::Result<()> {
//         if self.styled {
//             map_io(write!(self.writer, "{}", style.prefix()))?;
//             map_io(write!(self.writer, "{:?}", fragment))?;
//             map_io(write!(self.writer, "{}", style.suffix()))?;
//         } else {
//             map_io(write!(self.writer, "{:?}", fragment))?;
//         }

//         Ok(())
//     }
// }

// pub enum EmitWriter<'a> {
//     OwnedIo(Rc<RefCell<dyn std::io::Write + 'a>>),
//     BorrowedIo(&'a mut dyn std::io::Write),
//     BorrowedFmt(&'a mut dyn std::fmt::Write),
// }

// impl<'a> Write for EmitWriter<'a> {
//     fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//         match self {
//             EmitWriter::OwnedIo(owned) => owned.borrow_mut().write(buf),
//             EmitWriter::BorrowedIo(io) => io.write(buf),
//             EmitWriter::BorrowedFmt(fmt) => {
//                 let str = String::from_utf8_lossy(buf);
//                 fmt.write_str(&str)
//                     .map(|_| buf.len())
//                     .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))
//             }
//         }
//     }

//     fn flush(&mut self) -> std::io::Result<()> {
//         todo!()
//     }
// }

// impl<'a> std::fmt::Debug for EmitWriter<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             EmitWriter::OwnedIo(_) => write!(f, "OwnedIo"),
//             EmitWriter::BorrowedIo(_) => write!(f, "BorrowedIo"),
//             EmitWriter::BorrowedFmt(_) => write!(f, "BorrowedFmt"),
//         }
//     }
// }

// impl<'a> EmitWriter<'a> {
//     pub fn owned_io(io: impl std::io::Write + 'a) -> EmitWriter<'a> {
//         EmitWriter::OwnedIo(Rc::new(RefCell::new(io)))
//     }

//     pub fn borrowed_io(io: &'a mut dyn std::io::Write) -> EmitWriter<'a> {
//         EmitWriter::BorrowedIo(io)
//     }

//     pub fn borrowed_fmt(fmt: &'a mut dyn std::fmt::Write) -> EmitWriter<'a> {
//         EmitWriter::BorrowedFmt(fmt)
//     }

//     pub fn child(&mut self) -> EmitWriter<'_> {
//         match self {
//             EmitWriter::OwnedIo(rc) => EmitWriter::OwnedIo(rc.clone()),
//             EmitWriter::BorrowedIo(write) => EmitWriter::BorrowedIo(*write),
//             EmitWriter::BorrowedFmt(fmt) => EmitWriter::BorrowedFmt(*fmt),
//         }
//     }
// }

// impl<'a> EmitWriter<'a> {
//     fn write_fmt(&mut self, args: Arguments<'_>) -> std::fmt::Result {
//         match self {
//             EmitWriter::OwnedIo(owned) => owned
//                 .borrow_mut()
//                 .write_fmt(args)
//                 .map_err(|_| std::fmt::Error),
//             EmitWriter::BorrowedIo(borrowed) => {
//                 borrowed.write_fmt(args).map_err(|_| std::fmt::Error)
//             }
//             EmitWriter::BorrowedFmt(borrowed) => borrowed.write_fmt(args),
//         }
//     }
// }
