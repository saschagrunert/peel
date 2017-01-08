#[macro_use]
extern crate log;
use log::LogLevel;

extern crate peel;
use peel::example::prelude::*;

#[test]
fn peel_success() {
    let mut peel = peel_example();
    peel.set_log_level(LogLevel::Trace);
    let result = peel.traverse(b"1123", vec![]).unwrap();
    let result_struct = result[0].downcast_ref::<Parser1Result>().unwrap();
    assert_eq!(result_struct, &Parser1Result);
}
