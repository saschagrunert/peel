#[macro_use]
extern crate log;

#[macro_use]
extern crate nom;
extern crate peel;

use log::LogLevel;
use peel::example::prelude::*;

fn main() {
    let mut peel = peel_example();
    peel.set_log_level(LogLevel::Info);

    let root = peel.graph.node_indices().last().unwrap();
    peel.link_new_parser(root, MyParser);

    let result = peel.traverse(b"12345", vec![]).unwrap();
    assert_eq!(result.len(), 5);
    assert_eq!(result[4].downcast_ref::<MyParserResult>(),
               Some(&MyParserResult));
}

#[derive(Debug, PartialEq)]
struct MyParserResult;
struct MyParser;

impl Parsable<()> for MyParser {
    /// The actual parsing entry point
    fn parse<'a>(&mut self,
                 input: &'a [u8],
                 _: Option<&ParserResultVec>,
                 _: Option<&mut ()>)
                 -> IResult<&'a [u8], ParserResult> {
        do_parse!(input, tag!("5") >> (Box::new(MyParserResult)))
    }
}

impl fmt::Display for MyParser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "My Parser")
    }
}
