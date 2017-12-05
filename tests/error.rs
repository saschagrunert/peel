extern crate peel;
use peel::error::*;

use std::io;

#[test]
fn success_convert_from_io_error() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "Not found");
    let peel_error: PeelError = io_error.into();
    assert_eq!(peel_error.description, "Not found".to_string());
}
