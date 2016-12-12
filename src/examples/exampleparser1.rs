//! First example parser
use examples::prelude::*;

#[derive(Debug, Clone)]
/// Could hold some parser data like configuration parameters
pub struct ExampleParser1;

#[derive(Debug)]
/// The actual parser returning result
pub struct ExampleParser1Result {
    /// Some value
    pub value_1: u8,

    /// Another value
    pub value_2: u16,
}

impl Parser for ExampleParser1 {
    type Result = ParserResult;
    type Variant = ParserVariant;

    fn parse<'a>(&self,
                 input: &'a [u8],
                 _: &ParserNode<ParserResult, ParserVariant>,
                 _: &ParserArena<ParserResult, ParserVariant>)
                 -> IResult<&'a [u8], ParserResult> {
        do_parse!(input,
                  value_1: be_u8 >>
                  value_2: be_u16 >>
                  (ParserResult::ExampleParser1Result(ExampleParser1Result {
                      value_1: value_1,
                      value_2: value_2,
                  })))
    }

    fn variant(&self) -> ParserVariant {
        ParserVariant::ExampleParser1(self.clone())
    }
}
