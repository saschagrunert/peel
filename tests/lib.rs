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
    assert_eq!(result[0].downcast_ref::<Parser1Result>(),
               Some(&Parser1Result));
    assert_eq!(result[1].downcast_ref::<Parser2Result>(),
               Some(&Parser2Result));
    assert_eq!(result[2].downcast_ref::<Parser3Result>(),
               Some(&Parser3Result));
    assert_eq!(result[3].downcast_ref::<Parser4Result>(),
               Some(&Parser4Result));
}

#[test]
fn peel_success_133() {
    let mut peel = peel_example();
    peel.set_log_level(LogLevel::Trace);
    let result = peel.traverse(b"133", vec![]).unwrap();

    assert_eq!(result.len(), 3);
    assert_eq!(result[0].downcast_ref::<Parser1Result>(),
               Some(&Parser1Result));
    assert_eq!(result[2].downcast_ref::<Parser3Result>(),
               Some(&Parser3Result));
    assert_eq!(result[2].downcast_ref::<Parser3Result>(),
               Some(&Parser3Result));
}

#[test]
fn peel_success_link() {
    let mut peel :Peel<()> = Peel::new();
    let p1 = peel.new_parser(Parser1);
    let p2 = peel.new_parser(Parser2);
    peel.link(p1, p2);
    assert_eq!(peel.graph.node_indices().count(), 2);
}

#[test]
fn peel_success_link_new_parser() {
    let mut peel :Peel<()> = Peel::new();
    let p1 = peel.new_parser(Parser1);
    peel.link_new_parser(p1, Parser2);
    assert_eq!(peel.graph.node_indices().count(), 2);
}

#[test]
fn peel_success_dot() {
    let mut peel = peel_example();
    assert!(peel.create_dot_file().is_ok());
}

#[test]
fn peel_success_parser1() {
    let mut parser = Parser1;
    let result = parser.parse(b"1", None, None).unwrap().1;
    assert_eq!(result.downcast_ref::<Parser1Result>(), Some(&Parser1Result));
}

#[test]
fn peel_success_remove() {
    let mut peel = peel_example();
    let count = peel.graph.node_indices().count();
    let last_node = peel.graph.node_indices().last().unwrap();
    assert!(peel.remove(last_node).is_some());
    assert_eq!(peel.graph.node_indices().count(), count - 1);
}

#[test]
fn peel_failure_no_tree_root() {
    let mut peel: Peel<()> = Peel::new();
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
