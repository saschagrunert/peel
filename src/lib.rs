//! # Dynamic packet parsing within trees 🌲 🌳 🌴
//!
//! Target of this library is to provide a flexible parsing approach for network packets. This will
//! be done within parser trees which can be modified during runtime. Other stacks beside the well
//! known TCP/IP protocol family should be parsable too.
//!
//! ## Example usage
//!
//! ```
//! use peal::prelude::*;
//!
//! // Get the default tree based on the TCP/IP stack
//! let peal = get_packet_peal();
//!
//! // Traverse the parser tree. If a parser matches check for available
//! // child parsers. Stop parsing if there are no childs left. In this
//! // example no parser would match because the input is no valid Ethernet
//! // packet. The `vec![]` memory will be used for the resulting stack of
//! // `Layer`s.
//! let result = peal.traverse(&[0xff; 500], vec![]).unwrap();
//!
//! assert_eq!(result.len(), 0);
//! ```
//!
#![deny(missing_docs)]

#[macro_use]
extern crate nom;

#[macro_use]
extern crate log;

extern crate term;

#[macro_use]
pub mod error;
pub mod packet;
pub mod arenatree;
pub mod parser;
mod logger;

use std::{fmt, iter};

use nom::IResult;
use log::LogLevelFilter;

use self::prelude::*;
use parser::ParserBox;
use logger::Logger;

/// Provides sensible imports at all
pub mod prelude {
    pub use super::Peal;
    pub use error::{PealResult, PealError, ErrorType};
    pub use arenatree::{Arena, NodeId, Node};
    pub use parser::{Parser, ParserNode, ParserArena};

    pub use packet::prelude::*;
}

/// The main pealing structure
pub struct Peal<R, V> {
    /// The memory arena of the tree
    pub arena: Arena<ParserBox<R, V>>,

    /// The current result stack of the parsers
    pub result: Vec<R>,

    /// The first node added will be the root
    pub root: Option<NodeId>,
}

impl<R, V> Peal<R, V> {
    /// Create a new emtpy `Peal` instance
    pub fn new() -> Self {
        Peal {
            arena: Arena::new(),
            result: vec![],
            root: None,
        }
    }

    /// Set the global log level for reporting
    pub fn set_log_level(&mut self, level: LogLevelFilter) {
        // Setup the logger if not already set
        if log::set_logger(|max_log_level| {
                max_log_level.set(level);
                Box::new(Logger)
            })
            .is_err() {
            warn!("Logger already set.");
        };
        trace!("Log level set to: {:?}", level);
    }

    /// Create a new boxed Parser and return a corresponding Node
    pub fn new_parser<T>(&mut self, parser: T) -> NodeId
        where T: Parser<Result = R, Variant = V> + Send + Sync +  'static
    {
        // Create a new Parser in a Box
        let parser_box = Box::new(parser);

        // Create a new node
        let new_node = self.arena.new_node(parser_box);

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

    /// Convenient function for recursive traversal with the root as starting point
    pub fn traverse(&self, input: &[u8], result: Vec<R>) -> PealResult<Vec<R>> where V: fmt::Display {
        match self.root {
            Some(node) => self.traverse_recursive(node, input, result),
            None => bail!(ErrorType::NoTreeRoot, "No tree root found"),
        }
    }

    /// Do parsing until all possible paths failed. This is equivalent in finding the deepest
    /// possible parsing result within the tree. The result will be assembled together in the
    /// given result vector, which will be returned at the end.
    pub fn traverse_recursive(&self, start_node: NodeId, input: &[u8], mut result: Vec<R>)
        -> PealResult<Vec<R>> where V: fmt::Display {
        for node_id in start_node.following_siblings(&self.arena) {
            // Get the initial values from the arena
            let ref node = self.arena[node_id];
            let ref parser = node.data;

            // Do the actual parsing work
            match parser.parse(input, Some(node), Some(&self.arena), Some(&result)) {
                IResult::Done(input_left, parser_result) => {
                    // Adapt the result
                    trace!("{} parsing succeed, left input length: {}",
                           parser.variant(), input_left.len());
                    result.push(parser_result);

                    // Check for further child nodes
                    match node.first_child() {
                        Some(node) => {
                            trace!("Continue traversal to first child of the parser.");
                            result = self.traverse_recursive(node, input_left, result)?;
                        }
                        None => {
                            trace!("No child nodes left any more, parsing done.");
                        }
                    }
                    // Do not test any other parsers since we already succeed
                    break;
                }
                IResult::Error(err) => trace!("{} parser failed with error: {:?}", parser.variant(), err),
                IResult::Incomplete(err) => trace!("{} parser failed: {:?}", parser.variant(), err),
            }
        }
        Ok(result)
    }

    /// Display the trees children by recursive iteration
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
}

impl<K, V> fmt::Display for Peal<K, V> where V: fmt::Display {
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
