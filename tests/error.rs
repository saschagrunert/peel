#[macro_use]
extern crate peel;
use peel::prelude::*;

extern crate term;
use std::io;

#[test]
fn success_convert_from_io_error() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "Not found");
    let peal_error: PeelError = io_error.into();
    assert_eq!(peal_error.code, ErrorType::Other);
    assert_eq!(peal_error.description, "Not found".to_string());
}

#[test]
fn success_convert_from_term_error() {
    let term_error = term::Error::NotSupported;
    let peal_error: PeelError = term_error.into();
    assert_eq!(peal_error.code, ErrorType::Other);
    assert_eq!(peal_error.description,
               "operation not supported by the terminal".to_string());
}
