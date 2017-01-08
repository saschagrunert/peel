//! Example parser 1
use example::prelude::*;

#[derive(Debug, Clone)]
/// The first example parser
pub struct Parser1;

#[derive(Debug, Clone, PartialEq)]
/// The result of the first example parser
pub struct Parser1Result;

impl Parser<()> for Parser1 {
    /// The actual parsing entry point
    fn parse<'a>(&mut self,
                 input: &'a [u8],
                 _: Option<&ParserResultVec>,
                 _: Option<&mut ()>)
                 -> IResult<&'a [u8], ParserResult> {
        do_parse!(input, tag!("1") >> (Box::new(Parser1Result)))
    }
}

impl fmt::Display for Parser1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parser 1")
    }
}
