//! Example parsers
mod parser1;
mod parser2;
mod parser3;
mod parser4;

use self::prelude::*;

pub mod prelude {
    //! Sensible defaults for the example parsers
    pub use std::fmt;
    pub use Peel;
    pub use parser::{Parsable, ParserResult, ParserResultVec};
    pub use super::peel_example;
    pub use nom::IResult;

    pub use example::parser1::*;
    pub use example::parser2::*;
    pub use example::parser3::*;
    pub use example::parser4::*;
}

/// Return a `Peel` instance for the example parsers
pub fn peel_example() -> Peel<()> {
    // Create a tree
    let mut p = Peel::new();

    // Create some parsers
    let parser_1 = p.new_parser(Parser1);
    let parser_2 = p.new_parser(Parser2);
    let parser_3 = p.new_parser(Parser3);
    let parser_4 = p.new_parser(Parser4);

    // Link the parsers together
    p.link_nodes(&[(parser_1, parser_2),
                   (parser_1, parser_3),
                   (parser_2, parser_3),
                   (parser_3, parser_3),
                   (parser_3, parser_4)]);

    p
}
