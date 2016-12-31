//! Example parser 3
use example::prelude::*;

#[derive(Debug, Clone)]
/// The third example parser
pub struct Parser3;

impl Parser<()> for Parser3 {
    type Result = ParserResult;
    type Variant = ParserVariant;

    /// The actual parsing entry point
    fn parse<'a>(&mut self,
                 input: &'a [u8],
                 _: Option<&Vec<Self::Result>>,
                 _: Option<&mut ()>)
                 -> IResult<&'a [u8], Self::Result> {
        do_parse!(input, tag!("3") >> (ParserResult::Result3))
    }

    fn variant(&self) -> Self::Variant {
        ParserVariant::Variant3(self.clone())
    }
}
