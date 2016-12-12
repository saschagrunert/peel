#[macro_use]
extern crate log;
extern crate parsetree;

use log::LogLevelFilter;

use parsetree::Tree;
use parsetree::packet::prelude::*;

#[test]
fn tree_example() {
    // Create a tree
    let mut tree = Tree::new();
    tree.set_log_level(LogLevelFilter::Trace);

    // Create some parsers
    let eth = tree.new_parser(EthernetParser);

    // Combine the parsers
    // tree.link(eth, ipv4);

    // Traverse the tree and find the "best" parsing result
    let input = [1, 2, 3, 4, 5, 6, 1, 2, 3, 4, 5, 6, 8, 0];
    let result = tree.traverse(eth, &input, vec![]);
    info!("Result: {:?}", result);
}
