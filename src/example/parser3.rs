//! Example parser 2
use example::prelude::*;

/// The third example parser
pub struct Parser3;

#[derive(Debug, PartialEq)]
/// The result of the third example parser
pub struct Parser3Result;

impl Parsable<()> for Parser3 {
    /// The actual parsing entry point
    fn parse<'a>(&mut self,
                 input: &'a [u8],
                 _: Option<&ParserResultVec>,
                 _: Option<&mut ()>)
                 -> IResult<&'a [u8], ParserResult> {

        do_parse!(input, tag!("3") >> (Box::new(Parser3Result)))
    }
}

impl fmt::Display for Parser3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parser 3")
    }
}
