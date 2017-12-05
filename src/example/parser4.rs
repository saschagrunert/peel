//! Example parser 4
use example::prelude::*;

/// The fourth example parser
#[derive(Debug)]
pub struct Parser4;

#[derive(Debug, PartialEq)]
/// The result of the fourth example parser
pub struct Parser4Result;

impl Parsable<()> for Parser4 {
    /// The actual parsing entry point
    fn parse<'a>(&mut self,
                 input: &'a [u8],
                 _: Option<&ParserResultVec>,
                 _: Option<&mut ()>)
                 -> IResult<&'a [u8], ParserResult> {

        do_parse!(input, tag!("4") >> (Box::new(Parser4Result)))
    }
}
