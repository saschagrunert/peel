//! Second example parser
use examples::prelude::*;

#[derive(Debug, Clone)]
/// Could hold some parser data like configuration parameters
pub struct ExampleParser2;

#[derive(Debug)]
/// The actual parser returning result
pub struct ExampleParser2Result {
    /// Some value
    pub value_1: u32,

    /// Another value
    pub value_2: u64,
}

impl Parser for ExampleParser2 {
    type Result = ParserResult;
    type Variant = ParserVariant;

    fn parse<'a>(&self,
                 input: &'a [u8],
                 _: &ParserNode<ParserResult, ParserVariant>,
                 _: &ParserArena<ParserResult, ParserVariant>)
                 -> IResult<&'a [u8], ParserResult> {
        do_parse!(input,
                  value_1: be_u32 >>
                  value_2: be_u64 >>
                  (ParserResult::ExampleParser2Result(ExampleParser2Result {
                      value_1: value_1,
                      value_2: value_2,
                  })))
    }

    fn variant(&self) -> ParserVariant {
        ParserVariant::ExampleParser2(self.clone())
    }
}
