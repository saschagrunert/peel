use examples::*;

#[derive(Debug)]
pub struct ExampleParser2;

#[derive(Debug)]
pub struct ExampleParser2Result {
    pub value_1: u32,
    pub value_2: u64,
}

impl Parser for ExampleParser2 {
    type Result = ParserResult;
    type Variant = ParserVariant;

    fn parse<'a, 'b>(&'a self,
                     input: &'b [u8],
                     _: &ParserNode<ParserResult, ParserVariant>,
                     _: &ParserTree<ParserResult, ParserVariant>)
                     -> IResult<&'b [u8], ParserResult> {
        do_parse!(input,
                  value_1: be_u32 >> value_2: be_u64 >>
                  (ParserResult::ExampleParser2Result(ExampleParser2Result {
                      value_1: value_1,
                      value_2: value_2,
                  })))
    }

    fn variant(&self) -> ParserVariant {
        ParserVariant::ExampleParser2
    }
}
