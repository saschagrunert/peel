//!  Dynamic parsers within a tree
#![deny(missing_docs)]

#[macro_use]
extern crate nom;

#[macro_use]
extern crate log;

extern crate term;

pub mod examples;
pub mod structures;
pub mod traits;
mod logger;

use nom::IResult;
use log::LogLevelFilter;

use structures::{Arena, NodeId};
use traits::{ParserBox, Parser};
use logger::Logger;

/// A parser tree description for a result R and a variant V
pub struct Tree<R, V> {
    /// The memory arena of the tree
    pub arena: Arena<ParserBox<R, V>>,

    /// The current result stack of the parsers
    pub result: Vec<R>,
}

impl<R, V> Tree<R, V> {
    /// Create a new emtpy `Tree`
    pub fn new() -> Self {
        Tree {
            arena: Arena::new(),
            result: vec![],
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

        // Create a new node and return the node ID
        self.arena.new_node(parser_box)
    }

    /// Append the second node to the first one within the current tree structure
    pub fn link(&mut self, left: NodeId, right: NodeId) {
        left.append(right, &mut self.arena);
    }

    /// Do parsing until all possible paths failed. This is equivalent in finding the deepest
    /// possible parsing result within the tree. The result will be assembled together in the
    /// given result vector, which will be returned at the end.
    pub fn traverse(&self, start_node: NodeId, input: &[u8], mut result: Vec<R>) -> Vec<R> {
        for node_id in start_node.following_siblings(&self.arena) {
            // Get the initial values from the arena
            let ref node = self.arena[node_id];
            let ref parser = node.data;
            trace!("Parsing: {:?}", node_id);

            // Do the actual parsing work
            match parser.parse(input, node, &self.arena) {
                IResult::Done(input_left, parser_result) => {
                    // Adapt the result
                    trace!("Parsing succeed, left input length: {}", input_left.len());
                    result.push(parser_result);

                    // Check for further child nodes
                    match node.first_child() {
                        Some(node) => {
                            trace!("Continue traversal at first child of the node.");
                            result = self.traverse(node, input_left, result);
                        }
                        None => trace!("No child left any more, parsing done."),
                    }
                }
                IResult::Error(err) => trace!("Parser failed with error: {:?}", err),
                IResult::Incomplete(err) => trace!("Parser failed: {:?}", err),
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use examples::prelude::*;
    use super::*;

    #[test]
    fn tree() {
        // Create a tree
        let mut tree = Tree::new();
        tree.set_log_level(LogLevelFilter::Trace);

        // Create some parsers
        let example_parser_1 = tree.new_parser(ExampleParser1);
        let example_parser_2 = tree.new_parser(ExampleParser2);

        // Combine the parsers
        tree.link(example_parser_1, example_parser_2);

        // Traverse the tree and find the "best" parsing result
        let input = [0xff; 20];
        let result = tree.traverse(example_parser_1, &input, vec![]);
        println!("Result: {:?}", result);
    }
}
