//! Example parser 4
use example::prelude::*;

#[derive(Debug, Clone)]
/// The third example parser
pub struct Parser4;

impl Parser for Parser4 {
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
            tag!("4") >>
            (ParserResult::Result4(true), ParserState::Stop)
        )
    }

    fn variant(&self) -> Self::Variant {
        ParserVariant::Variant4(self.clone())
    }
}
