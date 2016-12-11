//! General parser descriptions and traits
use nom::IResult;
use structures::{Node, Arena};

/// The type which will be stored within the tree structure
pub type ParserBox<R, V> = Box<Parser<Result = R, Variant = V>>;

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
    fn parse<'a, 'b>(&'a self,
                     input: &'b [u8],
                     node: &ParserNode<Self::Result, Self::Variant>,
                     arena: &ParserArena<Self::Result, Self::Variant>)
                     -> IResult<&'b [u8], Self::Result>;

    /// Return the actual enum variant of the parser
    fn variant(&self) -> Self::Variant;
}
