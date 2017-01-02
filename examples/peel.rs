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
    assert_eq!(result[4], ParserResult::Custom);
}

#[derive(Debug, Clone)]
struct MyParser;

impl Parser<()> for MyParser {
    type Result = ParserResult;
    type Variant = ParserVariant;

    /// The actual parsing entry point
    fn parse<'a>(&mut self,
                 input: &'a [u8],
                 _: Option<&Vec<Self::Result>>,
                 _: Option<&mut ()>)
                 -> IResult<&'a [u8], Self::Result> {
        do_parse!(input,
            tag!("5") >>
            (ParserResult::Custom)
        )
    }

    fn variant(&self) -> Self::Variant {
        ParserVariant::Custom
    }
}
