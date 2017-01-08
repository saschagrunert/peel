//! General parser descriptions and traits
use nom::IResult;

use std::any::Any;
use std::fmt::Display;

/// The type which will be stored within the tree structure
pub type Parser<D> = Box<Parsable<D>>;

/// A generic parser result
pub type ParserResult = Box<Any>;

/// A collection of parser results
pub type ParserResultVec = Vec<ParserResult>;

/// The parsing trait
pub trait Parsable<D>: Display {
    /// Parse using nom and return the result
    fn parse<'a>(&mut self,
                 input: &'a [u8],
                 result: Option<&ParserResultVec>,
                 data: Option<&mut D>)
                 -> IResult<&'a [u8], ParserResult>;
}
