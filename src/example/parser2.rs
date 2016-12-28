//! Example parser 2
use example::prelude::*;

#[derive(Debug, Clone)]
/// The second example parser
pub struct Parser2;

impl Parser for Parser2 {
    type Result = ParserResult;
    type Variant = ParserVariant;

    /// The actual parsing entry point
    fn parse<'a>(&self,
                 input: &'a [u8],
                 _: Option<&ExampleNode>,
                 _: Option<&ExampleArena>,
                 _: Option<&Vec<Self::Result>>)
                 -> IResult<&'a [u8], (Self::Result, ParserState)> {
        do_parse!(input,
            tag!("2") >>
            (ParserResult::Result2(true), ParserState::ContinueWithNextSibling)
        )
    }

    fn variant(&self) -> Self::Variant {
        ParserVariant::Variant2(self.clone())
    }
}
