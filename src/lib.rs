//! # Dynamic parsing within trees 🌲 🌳 🌴
//!
//! Target of this library is to provide a flexible approach in parsing data.
//! This will mainly be done within
//! [arena](https://en.wikipedia.org/wiki/Region-based_memory_management) based
//! [parser trees](https://en.wikipedia.org/wiki/Parse_tree) which can be modified
//! during runtime.
//! Every parser is using the [nom](https://github.com/Geal/nom) framework for the
//! actual parsing work. A complete source code example can be found within the
//! [`src/example`](https://github.com/saschagrunert/peel/tree/master/src/example)
//! directory of the crate.
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

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

use log::LogLevel;
use nom::{IResult, generate_colors, prepare_errors, print_codes, print_offsets};

use petgraph::{Graph, Direction};
use petgraph::dot::{Dot, Config};
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::visit::{EdgeRef, IntoEdgeReferences};

use prelude::*;
use parser::Parser;

/// Provides sensible imports at all
pub mod prelude {
    pub use super::Peel;
    pub use error::{PeelResult, PeelError, ErrorType};
    pub use parser::{Parsable, ParserResult, ParserResultVec};
}

/// The main peeling structure
pub struct Peel<D> {
    /// The memory arena of the tree
    pub graph: StableGraph<Parser<D>, ()>,

    /// The first node added will be the root
    pub root: Option<NodeIndex>,

    /// Additional data for which can be shared accross the parsers
    pub data: Option<D>,
}

impl<D> Peel<D> {
    /// Create a new empty `Peel` instance
    pub fn new() -> Self {
        Peel {
            graph: StableGraph::new(),
            root: None,
            data: None,
        }
    }

    /// Set the global log level for reporting
    pub fn set_log_level(&mut self, level: LogLevel) {
        // Setup the logger if not already set
        if mowl::init_with_level(level).is_err() {
            warn!("Logger already set.");
        } else {
            info!("Log level set to: {:?}", level);
        }
    }

    /// Create a new boxed Parser and return a corresponding Node
    pub fn new_parser<T>(&mut self, parser: T) -> NodeIndex
        where T: Parsable<D> + 'static
    {
        info!("New parser: {}", parser);

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
        info!("Link: {} → {}", self.graph[left], self.graph[right]);
        self.graph.add_edge(left, right, ());
    }

    /// Remove a parser from the graph and return if existing.
    pub fn remove(&mut self, node: NodeIndex) -> Option<Parser<D>> {
        info!("Removed: {}", self.graph[node]);
        self.graph.remove_node(node)
    }

    /// Link multiple nodes together
    pub fn link_nodes(&mut self, edges: &[(NodeIndex, NodeIndex)]) {
        for &(left, right) in edges {
            self.link(left, right);
        }
    }

    /// Create a new parser and link it with the provided node
    pub fn link_new_parser<T>(&mut self, left: NodeIndex, parser: T) -> NodeIndex
        where T: Parsable<D> + 'static
    {
        // Create a new node
        let new_parser = self.new_parser(parser);

        // Append the node to the given node
        self.link(left, new_parser);

        // Return the parser
        new_parser
    }

    /// Convenient function for recursive traversal with the root as starting point
    ///
    /// # Errors
    /// When no tree root was found or the first parser already fails.
    pub fn traverse(&mut self, input: &[u8], result: ParserResultVec) -> PeelResult<ParserResultVec> {
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
    pub fn traverse_recursive(&mut self,
                              node_id: NodeIndex,
                              input: &[u8],
                              mut result: ParserResultVec)
                              -> PeelResult<ParserResultVec> {

        let (left_input, error) = {
            // Get the values from the graph structure
            let parser = &mut self.graph[node_id];

            // Do the actual parsing work
            match parser.parse(input,
                               Some(&result),
                               if let Some(ref mut data) = self.data {
                                   Some(data)
                               } else {
                                   None
                               }) {
                // Parsing succeed
                IResult::Done(left_input, parser_result) => {
                    debug!("{} parsing succeed, left input length: {}",
                           parser,
                           left_input.len());
                    // Adapt the result
                    result.push(parser_result);
                    (left_input, None)
                }

                // Parsing failed
                error => {
                    trace!("Failed parser: {}", parser);
                    if result.is_empty() {
                        bail!(ErrorType::RootParserFailed, "No parser succeed at all");
                    }
                    (input, Some(error))
                }
            }
        };

        match error {
            // Display the parsing error if needed
            Some(error) => {
                if log_enabled!(log::LogLevel::Trace) {
                    self.display_error(input, error);
                }
            }

            // Continue traversal if needed
            _ => {
                let mut edges = self.graph.neighbors_directed(node_id, Direction::Outgoing).detach();
                while let Some(node) = edges.next_node(&self.graph) {
                    // Save the previous result length
                    let prev_len = result.len();

                    // Do the recursion
                    result = self.traverse_recursive(node, left_input, result)?;

                    // Stop going deeper if something was added to the result
                    if prev_len < result.len() {
                        break;
                    }
                }
            }
        }

        // Return the current result
        Ok(result)
    }

    /// Create a graphviz `graph.dot` file representation in the current directory
    pub fn create_dot_file(&mut self) -> PeelResult<()> {
        // Create a temporarily graph for conversion
        let mut graph = Graph::<_, ()>::new();

        // Convert the nodes
        for node_id in self.graph.node_indices() {
            let parser = &self.graph[node_id];
            graph.add_node(format!("{}", parser));
        }

        // Convert the edges
        for edge in self.graph.edge_references() {
            graph.add_edge(edge.source(), edge.target(), ());
        }

        let mut f = File::create("graph.dot")?;
        f.write_all(format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel])).as_bytes())?;
        Ok(())
    }

    /// Display an error from a parser
    pub fn display_error(&self, input: &[u8], res: IResult<&[u8], ParserResult>) {
        let mut h: HashMap<u32, &str> = HashMap::new();
        let parsers = ["Custom",
                       "Tag",
                       "MapRes",
                       "MapOpt",
                       "Alt",
                       "IsNot",
                       "IsA",
                       "SeparatedList",
                       "SeparatedNonEmptyList",
                       "Many1",
                       "Count",
                       "TakeUntilAndConsume",
                       "TakeUntil",
                       "TakeUntilEitherAndConsume",
                       "TakeUntilEither",
                       "LengthValue",
                       "TagClosure",
                       "Alpha",
                       "Digit",
                       "AlphaNumeric",
                       "Space",
                       "MultiSpace",
                       "LengthValueFn",
                       "Eof",
                       "ExprOpt",
                       "ExprRes",
                       "CondReduce",
                       "Switch",
                       "TagBits",
                       "OneOf",
                       "NoneOf",
                       "Char",
                       "CrLf",
                       "RegexpMatch",
                       "RegexpMatches",
                       "RegexpFind",
                       "RegexpCapture",
                       "RegexpCaptures",
                       "TakeWhile1",
                       "Complete",
                       "Fix",
                       "Escaped",
                       "EscapedTransform",
                       "TagStr",
                       "IsNotStr",
                       "IsAStr",
                       "TakeWhile1Str",
                       "NonEmpty",
                       "ManyMN",
                       "TakeUntilAndConsumeStr",
                       "HexDigit",
                       "TakeUntilStr",
                       "OctDigit",
                       "Many0",
                       "Not",
                       "Permutation",
                       "ManyTill"];

        for (i, literal) in parsers.iter().enumerate() {
            h.insert(i as u32, literal);
        }

        if let Some(v) = prepare_errors(input, res) {
            let colors = generate_colors(&v);
            println!("Colors: {}", print_codes(colors, h));
            println!("Dump: \n{}", print_offsets(input, 0, &v));
        }
    }
}
