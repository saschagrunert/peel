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
    pub use super::{peel_example, ParserId};
    pub use nom::IResult;

    pub use example::parser1::*;
    pub use example::parser2::*;
    pub use example::parser3::*;
    pub use example::parser4::*;
}

/// An ID of a parser for the `peel_example` parsers.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ParserId {
    /// `parser1::Parser1`
    Parser1,
    /// `parser2::Parser2`
    Parser2,
    /// `parser3::Parser3`
    Parser3,
    /// `parser4::Parser4`
    Parser4,
    /// Something else
    Other(usize),
}

/// Return a `Peel` instance for the example parsers
pub fn peel_example() -> Peel<(), ParserId> {
    // Create a tree
    let mut p = Peel::new();

    // Create some parsers
    let parser_1 = p.new_parser(Parser1, ParserId::Parser1);
    let parser_2 = p.new_parser(Parser2, ParserId::Parser2);
    let parser_3 = p.new_parser(Parser3, ParserId::Parser3);
    let parser_4 = p.new_parser(Parser4, ParserId::Parser4);

    // Link the parsers together
    p.link_nodes(&[(parser_1, parser_2),
                   (parser_1, parser_3),
                   (parser_2, parser_3),
                   (parser_3, parser_3),
                   (parser_3, parser_4)]);

    p
}
