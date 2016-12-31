//! Example parser 4
use example::prelude::*;

#[derive(Debug, Clone)]
/// The fourth example parser
pub struct Parser4;

impl Parser<()> for Parser4 {
    type Result = ParserResult;
    type Variant = ParserVariant;

    /// The actual parsing entry point
    fn parse<'a>(&mut self,
                 input: &'a [u8],
                 _: Option<&Vec<Self::Result>>,
                 _: Option<&mut ()>)
                 -> IResult<&'a [u8], Self::Result> {
        do_parse!(input, tag!("4") >> (ParserResult::Result4))
    }

    fn variant(&self) -> Self::Variant {
        ParserVariant::Variant4(self.clone())
    }
}
