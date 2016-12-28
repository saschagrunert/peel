#[macro_use]
extern crate log;
use log::LogLevel;

extern crate peel;
use peel::example::prelude::*;

use std::error::Error;

#[test]
fn peel_success_1234() {
    let mut peel = peel_example();
    peel.set_log_level(LogLevel::Trace);
    let result = peel.traverse(b"1234", vec![]).unwrap();

    assert_eq!(result.len(), 5);
    assert_eq!(result[0], ParserResult::Result1(true));
    assert_eq!(result[1], ParserResult::Result2(true));
    assert_eq!(result[2], ParserResult::Result3(true));
    assert_eq!(result[3], ParserResult::Result3(true));
    assert_eq!(result[4], ParserResult::Result4(true));
}

#[test]
fn peel_success_133() {
    let mut peel = peel_example();
    peel.set_log_level(LogLevel::Trace);
    let result = peel.traverse(b"133", vec![]).unwrap();

    assert_eq!(result.len(), 3);
    assert_eq!(result[0], ParserResult::Result1(true));
    assert_eq!(result[1], ParserResult::Result3(true));
    assert_eq!(result[2], ParserResult::Result3(true));
}

#[test]
fn peel_success_parser1() {
    let parser = Parser1;
    let result = parser.parse(b"1", None, None, None).unwrap().1;
    assert_eq!(result.0, ParserResult::Result1(true));
    assert_eq!(result.1, ParserState::ContinueWithFirstChild);
}

#[test]
fn peel_failure_no_tree_root() {
    let peel :Peel<ParserResult, ParserVariant> = Peel::new();
    let result = peel.traverse(b"TEST", vec![]);
    assert!(result.is_err());
}

#[test]
fn peel_failure_no_parser_succeed() {
    let mut peel = peel_example();
    peel.set_log_level(LogLevel::Trace);
    let result = peel.traverse(b"888", vec![]);
    assert!(result.is_err());
    if let Err(error) = result {
        println!("{:?}", error);
        println!("{}", error.description());
    }
}
