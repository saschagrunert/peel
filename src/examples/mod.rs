//! Example parsers for the dynamic data structure
use nom::{be_u8, be_u16, be_u32, be_u64, IResult};
use self::prelude::*;

pub mod exampleparser1;
pub mod exampleparser2;

/// Provides sensible imports for parsers
pub mod prelude {
    pub use super::*;
    pub use structures::*;
    pub use traits::*;

    pub use super::exampleparser1::*;
    pub use super::exampleparser2::*;
}

#[derive(Debug)]
/// The return value for the variant retrieval of the Parser trait
pub enum ParserVariant {
    /// First example parser
    ExampleParser1,

    /// Second example parser
    ExampleParser2,
}

#[derive(Debug)]
/// The return value for the actual parsing
pub enum ParserResult {
    /// First example parser result
    ExampleParser1Result(ExampleParser1Result),

    /// Second example parser result
    ExampleParser2Result(ExampleParser2Result),
}
