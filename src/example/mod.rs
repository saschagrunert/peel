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

    // Create and link the parsers
    let parser_1 = p.new_parser(Parser1);

    // Append Parser2 to Parser1
    let parser_2 = p.link_new_parser(parser_1, Parser2);

    // Append Parser3 to Parser1
    let parser_3 = p.link_new_parser(parser_1, Parser3);

    // Parser 3 referse to itself
    p.link(parser_3, parser_3);

    // Parser 2 referse to Parser 3
    p.link(parser_2, parser_3);

    // Append Parser4 to Parser3
    p.link_new_parser(parser_3, Parser4);

    p
}
