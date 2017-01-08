//! Example parser 2
use example::prelude::*;

/// The second example parser
pub struct Parser2;

#[derive(Debug, PartialEq)]
/// The result of the second example parser
pub struct Parser2Result;

impl Parsable<()> for Parser2 {
    /// The actual parsing entry point
    fn parse<'a>(&mut self,
                 input: &'a [u8],
                 _: Option<&ParserResultVec>,
                 _: Option<&mut ()>)
                 -> IResult<&'a [u8], ParserResult> {

        do_parse!(input, tag!("2") >> (Box::new(Parser2Result)))
    }
}

impl fmt::Display for Parser2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parser 2")
    }
}
