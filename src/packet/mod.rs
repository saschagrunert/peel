//! Example parsers for the dynamic data structure
use self::prelude::*;

pub mod ethernet;

/// Provides sensible imports for parsers
pub mod prelude {
    pub use nom::{be_u16, IResult};
    pub use traits::{Parser, ParserNode, ParserArena};
    pub use super::{ParserResult, ParserVariant};

    pub use super::ethernet::*;
}

#[derive(Debug)]
/// The return value for the variant retrieval of the Parser trait
pub enum ParserVariant {
    /// Ethernet protocol for layer 1
    Ethernet(EthernetParser),
}

#[derive(Debug)]
/// The return value for the actual parsing
pub enum ParserResult {
    /// Ethernet parsing result
    Ethernet(EthernetPacket),
}
