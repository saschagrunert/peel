//! Example parsers
mod parser1;
mod parser2;
mod parser3;
mod parser4;

use std::fmt;
use self::prelude::*;

pub mod prelude {
    //! Sensible defaults for the example parsers
    pub use ::Peel;
    pub use parser::{Parser, ParserNode, ParserGraph};
    pub use super::{ParserResult, ParserVariant, peel_example};
    pub use nom::IResult;

    pub use example::parser1::*;
    pub use example::parser2::*;
    pub use example::parser3::*;
    pub use example::parser4::*;
}

/// Collects all possible parser variants
pub enum ParserVariant {
    /// First example parser
    Variant1(Parser1),

    /// Second example parser
    Variant2(Parser2),

    /// Third example parser
    Variant3(Parser3),

    /// Fourth example parser
    Variant4(Parser4),
}

#[derive(PartialEq, Debug)]
/// Return values of the parsers
pub enum ParserResult {
    /// The result of the first example parser
    Result1,

    /// The result of the second example parser
    Result2,

    /// The result of the third example parser
    Result3,

    /// The result of the fourth example parser
    Result4,
}

/// Return a `Peel` instance for the example parsers
pub fn peel_example() -> Peel<ParserResult, ParserVariant> {
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

impl fmt::Display for ParserVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParserVariant::Variant1(_) => write!(f, "Parser 1"),
            ParserVariant::Variant2(_) => write!(f, "Parser 2"),
            ParserVariant::Variant3(_) => write!(f, "Parser 3"),
            ParserVariant::Variant4(_) => write!(f, "Parser 4"),
        }
    }
}
