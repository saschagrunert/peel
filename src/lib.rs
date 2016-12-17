//! # Dynamic packet parsing within trees 🌲 🌳 🌴
//!
//! Target of this library is to provide a flexible parsing approach for network packets. This will be done within
//! [arena](https://en.wikipedia.org/wiki/Region-based_memory_management) based [parser trees](https://en.wikipedia.org/wiki/Parse_tree)
//! which can be modified during runtime. Other stacks beside the well known TCP/IP protocol family should be parsable too.
//!
//! ## Example usage
//!
//! ```
//! use peel::prelude::*;
//!
//! // Get the default tree based on the TCP/IP stack
//! let peel = default_peel();
//!
//! let eth_header = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 8, 0];
//!
//! // Traverse the parser tree. If a parser matches check for available child parsers.
//! // Stop parsing if there are no childs left. The `vec![]` memory will be used for
//! // the resulting stack of `Layer`s.
//! let result = peel.traverse(&eth_header, vec![]).unwrap();
//!
//! // There should be one parsed EthernetPacket in:
//! assert_eq!(result.len(), 1);
//! ```
//!
//! For a more advanced usage see the [`Peel`](struct.Peel.html) structure.
//!
#![deny(missing_docs)]

#[macro_use]
extern crate nom;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

extern crate regex;
extern crate term;

#[macro_use]
pub mod error;

#[macro_use]
pub mod memcmp;
pub mod packet;
pub mod arenatree;
pub mod parser;
mod logger;

use std::{fmt, iter};
use std::collections::HashMap;
use log::LogLevelFilter;
use nom::{generate_colors, prepare_errors, print_codes, print_offsets};

use self::prelude::*;
use parser::ParserBox;
use logger::Logger;

/// Provides sensible imports at all
pub mod prelude {
    pub use super::Peel;
    pub use error::{PeelResult, PeelError, ErrorType};
    pub use arenatree::{Arena, NodeId, Node};
    pub use parser::{Parser, ParserNode, ParserArena, ParserState};

    pub use packet::prelude::*;
}

/// The main peeling structure
pub struct Peel<R, V> {
    /// The memory arena of the tree
    pub arena: Arena<ParserBox<R, V>>,

    /// The current result stack of the parsers
    pub result: Vec<R>,

    /// The first node added will be the root
    pub root: Option<NodeId>,
}

impl<R, V> Peel<R, V> {
    /// Create a new emtpy `Peel` instance
    ///
    /// # Examples
    /// ```
    /// use peel::prelude::*;
    /// let peel :PacketPeel = Peel::new();
    /// ```
    pub fn new() -> Self {
        Peel {
            arena: Arena::new(),
            result: vec![],
            root: None,
        }
    }

    /// Set the global log level for reporting
    ///
    /// # Examples
    /// ```
    /// # extern crate log;
    /// # extern crate peel;
    /// # fn main() {
    /// use log::LogLevelFilter;
    /// use peel::prelude::*;
    ///
    /// let mut peel = default_peel();
    /// peel.set_log_level(LogLevelFilter::Trace);
    /// # }
    /// ```
    pub fn set_log_level(&mut self, level: LogLevelFilter) {
        // Setup the logger if not already set
        if log::set_logger(|max_log_level| {
                max_log_level.set(level);
                Box::new(Logger)
            })
            .is_err() {
            warn!("Logger already set.");
        };
        info!("Log level set to: {:?}", level);
    }

    /// Create a new boxed Parser and return a corresponding Node
    ///
    /// # Examples
    /// ```
    /// use peel::prelude::*;
    ///
    /// let mut p = Peel::new();
    /// let eth = p.new_parser(EthernetParser);
    /// ```
    pub fn new_parser<T>(&mut self, parser: T) -> NodeId
        where T: Parser<Result = R, Variant = V> + Send + Sync + 'static
    {
        // Create a new node
        let new_node = self.arena.new_node(Box::new(parser));

        // Check if the root node is already set. If not, then this will be the root
        if let None = self.root {
            self.root = Some(new_node);
        }

        // Return the shiny new node
        new_node
    }

    /// Append the second node to the first one within the current tree structure
    ///
    /// # Examples
    /// ```
    /// use peel::prelude::*;
    ///
    /// let mut p = Peel::new();
    /// let eth = p.new_parser(EthernetParser);
    /// let ipv4 = p.new_parser(Ipv4Parser);
    /// p.link(eth, ipv4);
    /// ```
    pub fn link(&mut self, left: NodeId, right: NodeId) {
        left.append(right, &mut self.arena);
    }

    /// Create a new parser and link it with the provided node
    ///
    /// # Examples
    /// ```
    /// use peel::prelude::*;
    ///
    /// let mut p = Peel::new();
    /// let eth = p.new_parser(EthernetParser);
    /// let ipv4 = p.link_new_parser(eth, Ipv4Parser);
    /// ```
    pub fn link_new_parser<T>(&mut self, left: NodeId, parser: T) -> NodeId
        where T: Parser<Result = R, Variant = V> + Send + Sync + 'static
    {
        let new = self.new_parser(parser);
        left.append(new, &mut self.arena);
        new
    }

    /// Convenient function for recursive traversal with the root as starting point
    ///
    /// # Examples
    /// ```
    /// use peel::prelude::*;
    ///
    /// let peel = default_peel();
    /// let eth_header = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 8, 0];
    /// let result = peel.traverse(&eth_header, vec![]).unwrap();
    /// assert_eq!(result.len(), 1);
    /// ```
    pub fn traverse(&self, input: &[u8], result: Vec<R>) -> PeelResult<Vec<R>> where V: fmt::Display {
        match self.root {
            Some(node) => self.traverse_recursive(node, input, result),
            None => bail!(ErrorType::NoTreeRoot, "No tree root found"),
        }
    }

    /// Do parsing until all possible paths failed. This is equivalent in finding the deepest
    /// possible parsing result within the tree. The result will be assembled together in the
    /// given result vector, which will be returned at the end.
    ///
    /// # Examples
    /// ```
    /// use peel::prelude::*;
    ///
    /// let peel = default_peel();
    /// let eth_header = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 8, 0];
    /// let result = peel.traverse_recursive(peel.root.unwrap(), &eth_header, vec![]).unwrap();
    /// assert_eq!(result.len(), 1);
    /// ```
    pub fn traverse_recursive(&self, start_node: NodeId, input: &[u8], mut result: Vec<R>)
        -> PeelResult<Vec<R>> where V: fmt::Display {
        for node_id in start_node.following_siblings(&self.arena) {
            // Get the initial values from the arena
            let ref node = self.arena[node_id];
            let ref parser = node.data;

            // Do the actual parsing work
            match parser.parse(input, Some(node), Some(&self.arena), Some(&result)) {
                IResult::Done(input_left, (parser_result, state)) => {
                    // Adapt the result
                    debug!("{} parsing succeed, left input length: {}",
                           parser.variant(), input_left.len());
                    result.push(parser_result);

                    // Check the parser state
                    match state {
                         ParserState::ContinueWithFirstChild => {
                            // Check for further child nodes
                            match node.first_child() {
                                Some(node) => {
                                    debug!("Continue traversal to first child of the parser.");
                                    result = self.traverse_recursive(node, input_left, result)?;
                                }
                                None => debug!("No child nodes left any more, parsing done."),
                            }
                         }
                         ParserState::Stop => {}
                    }

                    // Do not test any other parsers since we already succeed
                    break;
                }
                IResult::Error(err) => if log_enabled!(log::LogLevel::Trace) {
                    debug!("Failed parser: {}", parser.variant());
                    self.display_error(input, IResult::Error(err));
                },
                IResult::Incomplete(err) => if log_enabled!(log::LogLevel::Trace) {
                    debug!("Incomplete parser: {}", parser.variant());
                    self.display_error(input, IResult::Incomplete(err));
                }
            }
        }
        Ok(result)
    }

    /// Display the trees children by recursive iteration
    ///
    /// # Examples
    /// ```
    /// use peel::prelude::*;
    ///
    /// let peel = default_peel();
    /// println!("{}", peel);
    /// ```
    fn display_children(&self, f: &mut fmt::Formatter, node: NodeId, mut level: usize)
        -> fmt::Result where V: fmt::Display {
        level += 2;
        for child in node.children(&self.arena) {
            let indent = iter::repeat(' ').take(level).collect::<String>();
            let ref parser = self.arena[child].data;
            writeln!(f, "{}- {}", indent, parser.variant())?;
            self.display_children(f, child, level)?;
        }
        Ok(())
    }

    /// Display an error from a parser
    ///
    /// # Examples
    /// ```
    /// use peel::prelude::*;
    ///
    /// let peel = default_peel();
    /// let eth_header = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 0, 0];
    ///
    /// let parser = EthernetParser;
    /// let res = parser.parse(&eth_header, None, None, None);
    /// peel.display_error(&eth_header, res);
    /// ```
    pub fn display_error(&self, input: &[u8], res: IResult<&[u8], (R, ParserState)>) {
        let mut h: HashMap<u32, &str> = HashMap::new();
        let parsers = ["Custom", "Tag", "MapRes", "MapOpt", "Alt", "IsNot", "IsA", "SeparatedList",
                       "SeparatedNonEmptyList", "Many1", "Count", "TakeUntilAndConsume",
                       "TakeUntil", "TakeUntilEitherAndConsume", "TakeUntilEither", "LengthValue",
                       "TagClosure", "Alpha", "Digit", "AlphaNumeric", "Space", "MultiSpace",
                       "LengthValueFn", "Eof", "ExprOpt", "ExprRes", "CondReduce", "Switch",
                       "TagBits", "OneOf", "NoneOf", "Char", "CrLf", "RegexpMatch",
                       "RegexpMatches", "RegexpFind", "RegexpCapture", "RegexpCaptures",
                       "TakeWhile1", "Complete", "Fix", "Escaped", "EscapedTransform", "TagStr",
                       "IsNotStr", "IsAStr", "TakeWhile1Str", "NonEmpty", "ManyMN",
                       "TakeUntilAndConsumeStr", "HexDigit", "TakeUntilStr", "OctDigit", "Many0",
                       "Not", "Permutation", "ManyTill"];

        for (i, literal) in parsers.iter().enumerate() {
            h.insert(i as u32, literal);
        }

        if let Some(v) = prepare_errors(input, res) {
            let colors = generate_colors(&v);
            println!("Colors: {}", print_codes(colors, h));
            println!("Dump: \n{}",   print_offsets(input, 0, &v));
        }
    }
}

impl<K, V> fmt::Display for Peel<K, V> where V: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.root {
            Some(node) => {
                writeln!(f, "- {}", self.arena[node].data.variant())?;
                self.display_children(f, node, 0)
            }
            None => write!(f, "(no tree root available)"),
        }
    }
}
