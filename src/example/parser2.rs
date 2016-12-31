//! Example parser 2
use example::prelude::*;

#[derive(Debug, Clone)]
/// The second example parser
pub struct Parser2;

impl Parser<()> for Parser2 {
    type Result = ParserResult;
    type Variant = ParserVariant;

    /// The actual parsing entry point
    fn parse<'a>(&mut self,
                 input: &'a [u8],
                 _: Option<&Vec<Self::Result>>,
                 _: Option<&mut ()>)
                 -> IResult<&'a [u8], Self::Result> {
        do_parse!(input, tag!("2") >> (ParserResult::Result2))
    }

    fn variant(&self) -> Self::Variant {
        ParserVariant::Variant2(self.clone())
    }
}
