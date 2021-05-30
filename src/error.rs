use std::fmt;

#[derive(Debug)]
pub enum GBError {
    InvalidData,
    InvalidInput,
    NotFound,
    // cpu
    InstructionNotFound(u8),
}

impl std::error::Error for GBError {}

impl fmt::Display for GBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::GBError::*;
        match self {
            InvalidData => write!(f, "Invalid Data."),
            InvalidInput => write!(f, "Invalid Input."),
            NotFound => write!(f, "Not Found."),
            InstructionNotFound(inst) => write!(f, "Instruction not found({}).", inst),
        }
    }
}

pub type GBResult<T> = Result<T, GBError>;
