#[macro_use]
extern crate log;
extern crate parsetree;

use log::LogLevelFilter;

use parsetree::Tree;
use parsetree::examples::prelude::*;

#[test]
fn tree_example() {
    // Create a tree
    let mut tree = Tree::new();
    tree.set_log_level(LogLevelFilter::Trace);

    // Create some parsers
    let example_parser_1 = tree.new_parser(ExampleParser1);
    let example_parser_2 = tree.new_parser(ExampleParser2);

    // Combine the parsers
    tree.link(example_parser_1, example_parser_2);

    // Traverse the tree and find the "best" parsing result
    let input = [0xff; 17];
    let result = tree.traverse(example_parser_1, &input, vec![]);
    info!("Result: {:?}", result);
}
