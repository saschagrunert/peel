//! Example parser 3
use example::prelude::*;

#[derive(Debug, Clone)]
/// The third example parser
pub struct Parser3;

impl Parser for Parser3 {
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
            res: opt!(tag!("3")) >>
            (ParserResult::Result3(true), match res {
                Some(_) => ParserState::ContinueWithCurrent,
                None => ParserState::ContinueWithFirstChild,
            })
        )
    }

    fn variant(&self) -> Self::Variant {
        ParserVariant::Variant3(self.clone())
    }
}
