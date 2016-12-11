//! Example parsers for the dynamic data structure
use nom::{be_u8, be_u16, be_u32, be_u64, IResult};
use self::prelude::*;

pub mod exampleparser1;
pub mod exampleparser2;

/// Provides sensible imports for parsers
pub mod prelude {
    pub use super::*;
    pub use parser::*;
    pub use tree::*;

    pub use super::exampleparser1::*;
    pub use super::exampleparser2::*;
}

#[derive(Debug)]
pub enum ParserVariant {
    ExampleParser1,
    ExampleParser2,
}

#[derive(Debug)]
pub enum ParserResult {
    ExampleParser1Result(ExampleParser1Result),
    ExampleParser2Result(ExampleParser2Result),
}
