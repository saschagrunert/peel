//!  Dynamic parsers within a tree
#![deny(missing_docs)]

#[macro_use]
extern crate nom;

pub mod examples;
pub mod structures;
pub mod traits;

struct Tree {
    
}

#[cfg(test)]
mod tests {
    use examples::prelude::*;

    #[test]
    fn parser() {
        let tree: &mut Arena<ParserBox<ParserResult, ParserVariant>> = &mut Arena::new();

        let p1 = Box::new(ExampleParser1);
        let p2 = Box::new(ExampleParser2);
        let p3 = Box::new(ExampleParser2);

        let root = tree.new_node(p1);
        let sub_1 = tree.new_node(p2);
        let sub_2 = tree.new_node(p3);

        root.append(sub_1, tree);
        root.append(sub_2, tree);

        for node in root.descendants(tree) {
            let input = [0xff; 12];

            let ref node = tree[node];
            let ref parser = node.data;

            println!("Parsing result: {:?}", parser.parse(&input, node, tree));
            println!("Parser variant: {:?}\n", parser.variant());
        }
    }
}
