use examples::*;

#[derive(Debug)]
pub struct ExampleParser1;

#[derive(Debug)]
pub struct ExampleParser1Result {
    pub value_1: u8,
    pub value_2: u16,
}

impl Parser for ExampleParser1 {
    type Result = ParserResult;
    type Variant = ParserVariant;

    fn parse<'a, 'b>(&'a self,
                     input: &'b [u8],
                     _: &ParserNode<ParserResult, ParserVariant>,
                     _: &ParserTree<ParserResult, ParserVariant>)
                     -> IResult<&'b [u8], ParserResult> {
        do_parse!(input,
                  value_1: be_u8 >> value_2: be_u16 >>
                  (ParserResult::ExampleParser1Result(ExampleParser1Result {
                      value_1: value_1,
                      value_2: value_2,
                  })))
    }

    fn variant(&self) -> ParserVariant {
        ParserVariant::ExampleParser1
    }
}
