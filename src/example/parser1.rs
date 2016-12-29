//! Example parser 1
use example::prelude::*;

#[derive(Debug, Clone)]
/// The first example parser
pub struct Parser1;

impl Parser for Parser1 {
    type Result = ParserResult;
    type Variant = ParserVariant;

    /// The actual parsing entry point
    fn parse<'a>(&self,
                 input: &'a [u8],
                 _: Option<&ParserNode>,
                 _: Option<&ExampleGraph>,
                 _: Option<&Vec<Self::Result>>)
                 -> IResult<&'a [u8], Self::Result> {
        do_parse!(input,
            tag!("1") >>
            (ParserResult::Result1)
        )
    }

    fn variant(&self) -> Self::Variant {
        ParserVariant::Variant1(self.clone())
    }
}
