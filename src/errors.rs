use std::{fmt, io::Error};

#[derive(Debug)]
pub enum VmError {
    OutOfBoundsError,
    KeyboardInputError(Error),
    NotEnoughArguments,
    IncorrectFileNameError(String, Error),
    BadFileError(Error),
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfBoundsError => {
                write!(f, "The register required is out of bounds")
            }
            Self::KeyboardInputError(e) => {
                write!(f, "An error ocurred while reading Keyboard Input: {}", e)
            }
            Self::NotEnoughArguments => {
                write!(
                    f,
                    "There are arguments missing. Usage: cargo run [image-file1] ..."
                )
            }
            Self::IncorrectFileNameError(name, e) => {
                write!(f, "Error opening the file '{}': {}", name, e)
            }
            Self::BadFileError(e) => {
                write!(f, "The file had an error while reading: {}", e)
            }
        }
    }
}
