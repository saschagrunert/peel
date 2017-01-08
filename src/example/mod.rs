//! Example parsers
mod parser1;

use self::prelude::*;

pub mod prelude {
    //! Sensible defaults for the example parsers
    pub use std::fmt;
    pub use Peel;
    pub use parser::{Parser, ParserResult, ParserResultVec};
    pub use super::peel_example;
    pub use nom::IResult;

    pub use example::parser1::*;
}

/// Return a `Peel` instance for the example parsers
pub fn peel_example() -> Peel<()> {
    // Create a tree
    let mut p = Peel::new();

    // Create and link the parsers
    let root = p.new_parser(Parser1);
    let parser_1 = p.link_new_parser(root, Parser1);
    p.link_new_parser(parser_1, Parser1);

    p
}
