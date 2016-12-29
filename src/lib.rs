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
extern crate petgraph;
extern crate mowl;

#[macro_use]
pub mod error;
pub mod parser;
pub mod example;

use std::fmt;
use std::collections::HashMap;

use log::LogLevel;
use nom::{IResult, generate_colors, prepare_errors, print_codes, print_offsets};

use petgraph::Direction;
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::visit::EdgeRef;

use prelude::*;
use parser::ParserBox;

/// Provides sensible imports at all
pub mod prelude {
    pub use super::Peel;
    pub use error::{PeelResult, PeelError, ErrorType};
    pub use parser::{Parser, ParserNode, ParserGraph};
}

/// The main peeling structure
pub struct Peel<R, V> {
    /// The memory arena of the tree
    pub graph: StableGraph<ParserBox<R, V>, u8>,

    /// The first node added will be the root
    pub root: Option<NodeIndex>,
}

impl<R, V> Peel<R, V> where V: fmt::Display {
    /// Create a new empty `Peel` instance
    pub fn new() -> Self {
        Peel {
            graph: StableGraph::new(),
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
    pub fn new_parser<T>(&mut self, parser: T) -> NodeIndex
        where T: Parser<Result = R, Variant = V> + Send + Sync + 'static
    {
        // Create a new node
        let new_node = self.graph.add_node(Box::new(parser));

        // Check if the root node is already set. If not, then this will be the root
        if self.root.is_none() {
            self.root = Some(new_node);
        }

        // Return the shiny new node
        new_node
    }

    /// Append the second node to the first one within the current tree structure
    pub fn link(&mut self, left: NodeIndex, right: NodeIndex) {
        self.graph.add_edge(left, right, 0);
    }

    /// Create a new parser and link it with the provided node
    pub fn link_new_parser<T>(&mut self, left: NodeIndex, parser: T) -> NodeIndex
        where T: Parser<Result = R, Variant = V> + Send + Sync + 'static
    {
        // Create a new node
        let new_parser = self.new_parser(parser);

        // Append the node to the given node
        self.graph.add_edge(left, new_parser, 0);

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
    pub fn traverse_recursive(&self, node_id: NodeIndex, input: &[u8], mut result: Vec<R>)
        -> PeelResult<Vec<R>> {

        // Get the values from the graph structure
        let parser = &self.graph[node_id];

        // Do the actual parsing work
        match parser.parse(input, Some(&node_id), Some(&self.graph), Some(&result)) {
            IResult::Done(input_left, parser_result) => {
                // Adapt the result
                debug!("{} parsing succeed, left input length: {}", parser.variant(), input_left.len());
                result.push(parser_result);

                // Continue traversal if needed
                for edge in self.graph.edges_directed(node_id, Direction::Outgoing) {
                    let prev_len = result.len();
                    result = self.traverse_recursive(edge.target(), input_left, result)?;

                    // Stop going deeper if something was added to the result
                    if prev_len < result.len() {
                        break;
                    }
                }
            },
            error => if log_enabled!(log::LogLevel::Trace) {
                if Some(node_id) == self.root {
                    bail!(ErrorType::RootParserFailed, "No parser succeed at all");
                }
                debug!("Failed parser: {}", parser.variant());
                self.display_error(input, error);
            },
        }
        Ok(result)
    }

    /// Display an error from a parser
    pub fn display_error(&self, input: &[u8], res: IResult<&[u8], R>) {
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
