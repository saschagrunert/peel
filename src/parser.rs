//! General parser descriptions and traits
use nom::IResult;
use arenatree::{Node, Arena};

/// The type which will be stored within the tree structure
pub type ParserBox<R, V> = Box<Parser<Result = R, Variant = V> + Send + Sync>;

/// Arena tree for parsers
pub type ParserArena<R, V> = Arena<ParserBox<R, V>>;

/// A node within a `ParserArena`
pub type ParserNode<R, V> = Node<ParserBox<R, V>>;

/// The parsing trait
pub trait Parser {
    /// The type for result reporting, usually an enum
    type Result;

    /// The type of the parser itself, usually an enum
    type Variant;

    /// Parse using nom and return the result
    fn parse<'a>(&self,
                 input: &'a [u8],
                 node: Option<&ParserNode<Self::Result, Self::Variant>>,
                 arena: Option<&ParserArena<Self::Result, Self::Variant>>,
                 result: Option<&Vec<Self::Result>>)
                 -> IResult<&'a [u8], Self::Result>;

    /// Return the actual enum variant of the parser
    fn variant(&self) -> Self::Variant;
}