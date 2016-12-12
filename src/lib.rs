//!  Dynamic parsers within a tree
#![deny(missing_docs)]

#[macro_use]
extern crate nom;

#[macro_use]
extern crate log;

extern crate term;

pub mod packet;
pub mod structures;
pub mod traits;
mod logger;

use std::fmt::Display;

use nom::IResult;
use log::LogLevelFilter;

use structures::{Arena, NodeId};
use traits::{ParserBox, Parser};
use logger::Logger;

/// Provides sensible imports at all
pub mod prelude {
    pub use traits::{Parser, ParserNode, ParserArena};
    pub use structures::{Arena, NodeId, Node};
    pub use super::Tree;
}

/// A parser tree description for a result R and a variant V
pub struct Tree<R, V> {
    /// The memory arena of the tree
    pub arena: Arena<ParserBox<R, V>>,

    /// The current result stack of the parsers
    pub result: Vec<R>,

    /// The first node added will be the root
    pub root: Option<NodeId>,
}

impl<R, V> Tree<R, V> {
    /// Create a new emtpy `Tree`
    pub fn new() -> Self {
        Tree {
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
        where T: Parser<Result = R, Variant = V> + 'static
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
    pub fn traverse(&self, input: &[u8], result: Vec<R>) -> Vec<R> where V: Display {
        match self.root {
            Some(node) => self.traverse_recursive(node, input, result),
            None => result // TODO: Error handling
        }
    }

    /// Do parsing until all possible paths failed. This is equivalent in finding the deepest
    /// possible parsing result within the tree. The result will be assembled together in the
    /// given result vector, which will be returned at the end.
    pub fn traverse_recursive(&self, start_node: NodeId, input: &[u8], mut result: Vec<R>) -> Vec<R> where V: Display {
        for node_id in start_node.following_siblings(&self.arena) {
            // Get the initial values from the arena
            let ref node = self.arena[node_id];
            let ref parser = node.data;

            // Do the actual parsing work
            match parser.parse(input, node, &self.arena, &result) {
                IResult::Done(input_left, parser_result) => {
                    // Adapt the result
                    trace!("{} parsing succeed, left input length: {}",
                           parser.variant(), input_left.len());
                    result.push(parser_result);

                    // Check for further child nodes
                    match node.first_child() {
                        Some(node) => {
                            trace!("Continue traversal to first child of the parser.");
                            result = self.traverse_recursive(node, input_left, result);
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
        result
    }
}
