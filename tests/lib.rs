#[macro_use]
extern crate log;
use log::LogLevel;

extern crate peel;
use peel::example::prelude::*;
use peel::error::ErrorType;

extern crate nom;
use nom::Needed;

use std::error::Error;

#[test]
fn peel_success_1234() {
    let mut peel = peel_example();
    peel.set_log_level(LogLevel::Trace);
    let whole_result = peel.traverse(b"1234", vec![], vec![]);
    let result = whole_result.result;
    let parsers = whole_result.parsers;

    assert_eq!(result.len(), 4);
    assert_eq!(result[0].downcast_ref::<Parser1Result>(),
               Some(&Parser1Result));
    assert_eq!(result[1].downcast_ref::<Parser2Result>(),
               Some(&Parser2Result));
    assert_eq!(result[2].downcast_ref::<Parser3Result>(),
               Some(&Parser3Result));
    assert_eq!(result[3].downcast_ref::<Parser4Result>(),
               Some(&Parser4Result));
    assert_eq!(parsers.len(), 4);
    assert_eq!(ParserId::Parser1, parsers[0]);
    assert_eq!(ParserId::Parser2, parsers[1]);
    assert_eq!(ParserId::Parser3, parsers[2]);
    assert_eq!(ParserId::Parser4, parsers[3]);
}

#[test]
fn peel_success_1334() {
    let mut peel = peel_example();
    peel.set_log_level(LogLevel::Trace);
    let whole_result = peel.traverse(b"1334", vec![], vec![]);
    let result = whole_result.result;
    let parsers = whole_result.parsers;

    assert_eq!(result.len(), 4);
    assert_eq!(result[0].downcast_ref::<Parser1Result>(),
               Some(&Parser1Result));
    assert_eq!(result[1].downcast_ref::<Parser3Result>(),
               Some(&Parser3Result));
    assert_eq!(result[2].downcast_ref::<Parser3Result>(),
               Some(&Parser3Result));
    assert_eq!(result[3].downcast_ref::<Parser4Result>(),
               Some(&Parser4Result));
    assert_eq!(parsers.len(), 4);
    assert_eq!(ParserId::Parser1, parsers[0]);
    assert_eq!(ParserId::Parser3, parsers[1]);
    assert_eq!(ParserId::Parser3, parsers[2]);
    assert_eq!(ParserId::Parser4, parsers[3]);
}

#[test]
fn peel_success_133_incomplete_continue_4() {
    let mut peel = peel_example();
    peel.set_log_level(LogLevel::Trace);
    let ret = peel.traverse(b"133", vec![], vec![]);
    assert_eq!(ret.result.len(), 3);
    assert!(ret.left_input.is_empty());
    assert!(ret.error.is_some());
    let ret = peel.continue_traverse(b"34", vec![], vec![]);
    assert_eq!(ret.result.len(), 2);
    assert!(ret.left_input.is_empty());
    assert!(ret.error.is_none());
}

#[test]
fn peel_success_incomplete() {
    let mut peel = peel_example();
    peel.set_log_level(LogLevel::Trace);
    let peel_result = peel.traverse(b"", vec![], vec![]);
    info!("{:?}", peel_result);

    let error = peel_result.error.unwrap();
    let res = peel_result.result;
    let parsers = peel_result.parsers;

    if let ErrorType::Incomplete(needed) = error.code {
        assert_eq!(needed, Needed::Size(1));
        assert!(res.is_empty());
        assert!(parsers.is_empty());
        let result = peel.continue_traverse(b"1234", res, parsers).result;
        assert_eq!(result.len(), 4);
    } else {
        unreachable!();
    }
}

#[test]
fn peel_success_link() {
    let mut peel = Peel::new();
    let p1 = peel.new_parser(Parser1, ParserId::Parser1);
    let p2 = peel.new_parser(Parser2, ParserId::Parser2);
    peel.link(p1, p2);
    assert_eq!(peel.graph.node_indices().count(), 2);
}

#[test]
fn peel_success_link_new_parser() {
    let mut peel = Peel::new();
    let p1 = peel.new_parser(Parser1, ParserId::Parser1);
    peel.link_new_parser(p1, Parser2, ParserId::Parser2);
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
    let error = peel.traverse(b"TEST", vec![], vec![]).error.unwrap();
    assert_eq!(error.code, ErrorType::NoTreeRoot);
}

#[test]
fn peel_failure_no_parser_succeed() {
    let mut peel = peel_example();
    peel.set_log_level(LogLevel::Trace);
    let error = peel.traverse(b"888", vec![], vec![]).error.unwrap();
    println!("{:?}", error);
    println!("{}", error.description());
}
