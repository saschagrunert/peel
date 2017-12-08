#![feature(test)]
extern crate peel;
use peel::example::prelude::*;

extern crate test;
use test::Bencher;

#[bench]
fn tree_generation(bencher: &mut Bencher) {
    bencher.iter(|| {
        peel_example();
    });
}

#[bench]
fn tree_parsing(bencher: &mut Bencher) {
    let mut peel = peel_example();
    let input = b"1234";
    bencher.iter(|| {
        peel.traverse(input, vec![], vec![]);
    });
    bencher.bytes = input.len() as u64;
}
