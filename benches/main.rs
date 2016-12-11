#![feature(test)]
extern crate parsetree;
extern crate test;
extern crate nom;

use test::Bencher;
use parsetree::Tree;
use parsetree::examples::prelude::*;

#[bench]
fn tree_traversal_parse(bencher: &mut Bencher) {
    // Create a tree
    let mut tree = Tree::new();

    // Create some parsers
    let example_parser_1 = tree.new_parser(ExampleParser1);
    let example_parser_2 = tree.new_parser(ExampleParser2);

    // Combine the parsers
    tree.link(example_parser_1, example_parser_2);

    // Traverse the tree and find the "best" parsing result
    let input = [0xff; 100];

    bencher.iter(|| {
        assert_eq!(tree.traverse(example_parser_1, &input, vec![]).len(), 2);
    });
    bencher.bytes = input.len() as u64;
}

#[bench]
fn tree_traversal_only(bencher: &mut Bencher) {
    // Create a tree
    let mut tree = Tree::new();

    // Create some parsers
    let example_parser_1 = tree.new_parser(ExampleParser1);
    let example_parser_2 = tree.new_parser(ExampleParser2);

    // Combine the parsers
    tree.link(example_parser_1, example_parser_2);

    bencher.iter(|| { for _ in example_parser_1.descendants(&tree.arena) {} });
}

#[bench]
fn tree_generation(bencher: &mut Bencher) {
    bencher.iter(|| {
        // Create a tree
        let mut tree = Tree::new();

        // Create some parsers
        let example_parser_1 = tree.new_parser(ExampleParser1);
        let example_parser_2 = tree.new_parser(ExampleParser2);

        // Combine the parsers
        tree.link(example_parser_1, example_parser_2);
    });
}
