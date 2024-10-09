use std::io::Error;

#[derive(Debug)]
pub enum VmError {
    OutOfBoundsError,
    KeyboardInputError(Error),
}
