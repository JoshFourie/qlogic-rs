use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct LinearAlgebraError {
    repr: Repr
}

impl LinearAlgebraError {
    pub(crate) fn kind(&self) -> ErrorKind {
        match &self.repr {
            Repr::Simple(kind) => *kind,
            Repr::Custom(custom) => custom.kind
        }
    }
}

#[derive(Debug)]
enum Repr {
    Simple(ErrorKind),
    Custom(CustomError)
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum ErrorKind {
    MatrixStructure,
    Other
}

impl ErrorKind {
    pub(crate) fn as_str(&self) -> &'static str {
        match *self {
            ErrorKind::MatrixStructure => "matrix is badly formed for the operation",
            ErrorKind::Other => "undefined error in crate",
        }
    }
}

impl From<ErrorKind> for LinearAlgebraError {
    #[inline] fn from(e: ErrorKind) -> Self {
        Self { repr: Repr::Simple(e) }
    }
}

#[derive(Debug)]
pub(crate) struct CustomError {
    kind: ErrorKind,
    error: Box<dyn std::error::Error+Send+Sync>
}
 
impl Display for LinearAlgebraError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.repr)
    }
}

impl Error for LinearAlgebraError {
    
    fn description(&self) -> &str {
        match self.repr {
            Repr::Simple(..) => self.kind().as_str(),
            Repr::Custom(..) => self.kind().as_str()
        }
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self.repr {
            Repr::Simple(_) => None,
            Repr::Custom(ref custom) => custom.error.source()
        }
    }

} 