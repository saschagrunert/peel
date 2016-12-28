//! # Dynamic parsing within trees 🌲 🌳 🌴
//!
//! Target of this library is to provide a flexible approach in parsing data. This will mainly be
//! done within [arena](https://en.wikipedia.org/wiki/Region-based_memory_management) based
//! [parser trees](https://en.wikipedia.org/wiki/Parse_tree) which can be modified during runtime.
//! Every parser is using the [nom](https://github.com/Geal/nom) framework for the actual parsing
//! work. A complete source code example can be found within the
//! [`src/example`](https://github.com/saschagrunert/peel/tree/master/src/example) directory of the
//! crate.
#![deny(missing_docs)]

#[macro_use]
extern crate nom;

#[macro_use]
extern crate log;
extern crate indextree;
extern crate mowl;

#[macro_use]
pub mod error;
pub mod parser;
pub mod example;

use std::{iter, fmt};
use std::collections::HashMap;
use log::LogLevel;
use indextree::{Arena, NodeId};
use nom::{IResult, generate_colors, prepare_errors, print_codes, print_offsets};

use prelude::*;
use parser::ParserBox;

/// Provides sensible imports at all
pub mod prelude {
    pub use super::Peel;
    pub use error::{PeelResult, PeelError, ErrorType};
    pub use parser::{Parser, ParserNode, ParserArena, ParserState};
}

/// The main peeling structure
pub struct Peel<R, V> {
    /// The memory arena of the tree
    pub arena: Arena<ParserBox<R, V>>,

    /// The first node added will be the root
    pub root: Option<NodeId>,
}

impl<R, V> Peel<R, V> where V: fmt::Display {
    /// Create a new empty `Peel` instance
    pub fn new() -> Self {
        Peel {
            arena: Arena::new(),
            root: None,
        }
    }

    /// Set the global log level for reporting
    pub fn set_log_level(&mut self, level: LogLevel) {
        // Setup the logger if not already set
        if mowl::init_with_level(level).is_err() {
            warn!("Logger already set.");
        };
        info!("Log level set to: {:?}", level);
    }

    /// Create a new boxed Parser and return a corresponding Node
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
    pub fn link(&mut self, left: NodeId, right: NodeId) {
        left.append(right, &mut self.arena);
    }

    /// Create a new parser and link it with the provided node
    pub fn link_new_parser<T>(&mut self, left: NodeId, parser: T) -> NodeId
        where T: Parser<Result = R, Variant = V> + Send + Sync + 'static
    {
        // Create a new node
        let new_parser = self.new_parser(parser);

        // Append the node to the given node
        left.append(new_parser, &mut self.arena);

        // Return the parser
        new_parser
    }

    /// Convenient function for recursive traversal with the root as starting point
    ///
    /// # Errors
    /// When no tree root was found or the first parser already fails.
    pub fn traverse(&self, input: &[u8], result: Vec<R>) -> PeelResult<Vec<R>> {
        match self.root {
            Some(node) => self.traverse_recursive(node, input, result),
            None => bail!(ErrorType::NoTreeRoot, "No tree root found"),
        }
    }

    /// Do parsing until all possible paths failed. This is equivalent in finding the deepest
    /// possible parsing result within the tree. The result will be assembled together in the
    /// given result vector, which will be returned at the end.
    ///
    /// # Errors
    /// When the first parser already fails.
    pub fn traverse_recursive(&self, start_node: NodeId, input: &[u8], mut result: Vec<R>)
        -> PeelResult<Vec<R>> {

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
                    let next_node = match state {
                        // Continue with the first child parser
                         ParserState::ContinueWithFirstChild => {
                            debug!("Continue traversal to first child of the parser");
                            node.first_child()
                         },

                         // Try the next sibling before parsing deeper
                         ParserState::ContinueWithNextSibling => {
                            debug!("Continue traversal to next sibling of the parser");
                            node.next_sibling()
                         },

                         // Try the next sibling before parsing deeper
                         ParserState::ContinueWithCurrent => {
                            debug!("Trying the current parser again");
                            Some(node_id)
                         },

                         // Immediately stop the parser here
                         ParserState::Stop => None,
                    };

                    // Continue traversal if needed
                    if let Some(node) = next_node {
                        result = self.traverse_recursive(node, input_left, result)?;
                    }

                    // Stop here since we already succeed
                    break;
                }
                error @ _ => if log_enabled!(log::LogLevel::Trace) {
                    if Some(start_node) == self.root {
                        bail!(ErrorType::RootParserFailed, "No parser succeed at all");
                    }
                    debug!("Failed parser: {}", parser.variant());
                    self.display_error(input, error);
                },
            }
        }
        Ok(result)
    }

    /// Display the trees children by recursive iteration
    fn display_children(&self, f: &mut fmt::Formatter, node: NodeId, mut level: usize)
        -> fmt::Result {
        level += 2;
        for child in node.children(&self.arena) {
            let indent = iter::repeat(' ').take(level).collect::<String>();
            let ref parser = self.arena[child].data;
            write!(f, "{}- {}: {:?}", indent, parser.variant(), child)?;
            writeln!(f, " ({})", self.arena[child])?;
            self.display_children(f, child, level)?;
        }
        Ok(())
    }

    /// Display an error from a parser
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
                write!(f, "- {}: {:?}", self.arena[node].data.variant(), node)?;
                writeln!(f, " ({})", self.arena[node])?;
                self.display_children(f, node, 0)
            }
            None => write!(f, "(no tree root available)"),
        }
    }
}
