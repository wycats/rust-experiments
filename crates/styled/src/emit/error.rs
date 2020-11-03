use std::{error, fmt, io};

#[derive(Debug)]
pub enum EmitError {
    Error(Box<dyn error::Error + Send + Sync + 'static>),
}

impl EmitError {
    pub fn new(err: impl error::Error + Send + Sync + 'static) -> EmitError {
        EmitError::Error(Box::new(err))
    }
}

impl fmt::Display for EmitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmitError::Error(err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for EmitError {}

impl From<EmitError> for fmt::Error {
    fn from(_: EmitError) -> Self {
        fmt::Error
    }
}

impl From<EmitError> for io::Error {
    fn from(err: EmitError) -> Self {
        io::Error::new(io::ErrorKind::Other, err)
    }
}

macro_rules! emit_error {
    ($ty:ty) => {
        impl From<$ty> for EmitError {
            fn from(err: $ty) -> EmitError {
                EmitError::new(err)
            }
        }
    };
}

emit_error!(fmt::Error);
emit_error!(io::Error);

pub type EmitResult<T = ()> = Result<T, EmitError>;
