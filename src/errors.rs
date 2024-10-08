use std::io::Error;

#[derive(Debug)]
pub enum VmError {
    OutOfBoundsError,
    KeyboardInputError(Error),
}

#[derive(Debug)]
pub struct OutOfBoundsError;

pub struct KeyboardInputError;
