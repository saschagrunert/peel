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
    let peel = peel_example();
    let input = b"1234";
    bencher.iter(|| {
        assert!(peel.traverse(input, vec![]).is_ok());
    });
    bencher.bytes = input.len() as u64;
}
