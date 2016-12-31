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

    assert_eq!(result.len(), 4);
    assert_eq!(result[0], ParserResult::Result1);
    assert_eq!(result[1], ParserResult::Result2);
    assert_eq!(result[2], ParserResult::Result3);
    assert_eq!(result[3], ParserResult::Result4);
}

#[test]
fn peel_success_133() {
    let mut peel = peel_example();
    peel.set_log_level(LogLevel::Trace);
    let result = peel.traverse(b"133", vec![]).unwrap();

    assert_eq!(result.len(), 3);
    assert_eq!(result[0], ParserResult::Result1);
    assert_eq!(result[1], ParserResult::Result3);
    assert_eq!(result[2], ParserResult::Result3);
}

#[test]
fn peel_success_dot() {
    let mut peel = peel_example();
    assert!(peel.create_dot_file().is_ok());
}

#[test]
fn peel_success_parser1() {
    let mut parser = Parser1{test:0};
    let result = parser.parse(b"1", None).unwrap().1;
    assert_eq!(result, ParserResult::Result1);
}

#[test]
fn peel_failure_no_tree_root() {
    let mut peel :Peel<ParserResult, ParserVariant> = Peel::new();
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
