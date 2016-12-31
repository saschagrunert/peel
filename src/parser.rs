//! General parser descriptions and traits
use std::fmt;
use nom::IResult;

/// The type which will be stored within the tree structure
pub type ParserBox<R, V, D> = Box<Parser<D, Result = R, Variant = V> + Send + Sync>;

/// The parsing trait
pub trait Parser<D> {
    /// The type for result reporting, usually an enum
    type Result;

    /// The type of the parser itself, usually an enum
    type Variant;

    /// Parse using nom and return the result
    fn parse<'a>(&mut self,
                 input: &'a [u8],
                 result: Option<&Vec<Self::Result>>,
                 data: Option<&mut D>)
                 -> IResult<&'a [u8], Self::Result>;

    /// Return the actual enum variant of the parser as a clone
    fn variant(&self) -> Self::Variant;
}

macro_rules! impl_fmt {
    ($name: ident) => {
        impl<R, V, D> fmt::$name for Parser<D, Result = R, Variant = V> + Send + Sync
            where V: fmt::Display
        {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.variant())
            }
        }
    }
}

impl_fmt!(Debug);
impl_fmt!(Display);
