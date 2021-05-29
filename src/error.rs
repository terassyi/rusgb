use std::fmt;

#[derive(Debug)]
pub enum GBError {
    InvalidData,
    InvalidInput,
    NotFound,
    // cpu
    InstructionNotFound,
}

impl std::error::Error for GBError {}

impl fmt::Display for GBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::GBError::*;
        match self {
            InvalidData => write!(f, "Invalid Data."),
            InvalidInput => write!(f, "Invalid Input."),
            NotFound => write!(f, "Not Found."),
            InstructionNotFound => write!(f, "Instruction not found."),
        }
    }
}

pub type GBResult<T> = Result<T, GBError>;
