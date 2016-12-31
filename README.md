# peel
[![Build Status](https://travis-ci.org/saschagrunert/peel.svg)](https://travis-ci.org/saschagrunert/peel) [![Build status](https://ci.appveyor.com/api/projects/status/i67yq6yij2k17iwc?svg=true)](https://ci.appveyor.com/project/saschagrunert/peel) [![Coverage Status](https://coveralls.io/repos/github/saschagrunert/peel/badge.svg)](https://coveralls.io/github/saschagrunert/peel?branch=master) [![master doc peel](https://img.shields.io/badge/master_doc-peel-blue.svg)](https://saschagrunert.github.io/peel) [![License MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/saschagrunert/peel/blob/master/LICENSE) [![Crates.io](https://img.shields.io/crates/v/peel.svg)](https://crates.io/crates/peel) [![doc.rs](https://docs.rs/peel/badge.svg)](https://docs.rs/peel)
## Dynamic parsing within trees 🌲 🌳 🌴
Target of this library is to provide a flexible approach in parsing data. This will mainly be done within
[arena](https://en.wikipedia.org/wiki/Region-based_memory_management) based
[parser trees](https://en.wikipedia.org/wiki/Parse_tree) which can be modified during runtime. Every parser is using the
[nom](https://github.com/Geal/nom) framework for the actual parsing work. A complete source code example can be found
within the [`src/example`](https://github.com/saschagrunert/peel/tree/master/src/example) directory of the crate.

## Architecture and usage
Every `Peel` instance can be seen as a parsing graph structure which has different states and transitions. In the
example within the crate the structure looks like this:

![Example parser diagram](.github/example.png)

Independently of what these parser do, the creation of this structure is done within the `peel_example` function:

```rust
/// Return a `Peel` instance for the example parsers
pub fn peel_example() -> Peel<ParserResult, ParserVariant> {
    // Create a tree
    let mut p = Peel::new();

    // Create and link the parsers
    let parser_1 = p.new_parser(Parser1);

    // Append Parser2 to Parser1
    let parser_2 = p.link_new_parser(parser_1, Parser2);

    // Append Parser3 to Parser1
    let parser_3 = p.link_new_parser(parser_1, Parser3);

    // Parser 3 referse to itself
    p.link(parser_3, parser_3);

    // Parser 2 referse to Parser 3
    p.link(parser_2, parser_3);

    // Append Parser4 to Parser3
    p.link_new_parser(parser_3, Parser4);

    p
}
```

The first created parser will automatically be the root parser and the entry point for the tree traversal. Every
succeeding parser returns a certain result, which will be pushed into a vector. This means for our example that the
result is an enum of different types:

```rust
/// Return values of the parsers
pub enum ParserResult {
    /// The result of the first example parser
    Result1,

    /// The result of the second example parser
    Result2,

    /// The result of the third example parser
    Result3,

    /// The result of the fourth example parser
    Result4,
}
```

This means that the traversal method of `Peel` will try to find the deepest possible valid path within the tree
structure. After the creation of the structure the traversal can begin:

```rust
let mut peel = peel_example();
peel.set_log_level(LogLevel::Trace);
let result = peel.traverse(b"1234", vec![]).unwrap();

assert_eq!(result.len(), 4);
println!("{:?}", result);
```

With the help of the [log](https://crates.io/crates/log) crate it will output:
```
[peel] [DEBUG] Parser 1 parsing succeed, left input length: 3
[peel] [DEBUG] Failed parser: Parser 3
[peel] [DEBUG] Parser 2 parsing succeed, left input length: 2
[peel] [DEBUG] Parser 3 parsing succeed, left input length: 1
[peel] [DEBUG] Parser 4 parsing succeed, left input length: 0
[Result1, Result2, Result3, Result3, Result4]
```

A minimal parser has to implement the `Parser` trait which could look like this:
```rust
pub struct Parser1;

impl Parser for Parser1 {
    /// The result of the parser
    type Result = ParserResult;

    /// The variant of the parser
    type Variant = ParserVariant;

    /// The actual parsing entry point
    fn parse<'a>(&self,
                 input: &'a [u8],                    // The input for the parser
                 result: Option<&Vec<Self::Result>>) // The current parsing result
                 -> IResult<&'a [u8], Self::Result> {
        do_parse!(input,
            tag!("1") >>
            (ParserResult::Result1)
        )
    }

    // Returns the actual parser variant
    fn variant(&self) -> Self::Variant {
        ParserVariant::Variant1(self.clone())
    }
}
```

It is possible to access the current parsing `result` for a more advanced behavior like dependency checks during the
parsing.

## Contributing
You want to contribute to this project? Wow, thanks! So please just fork it and send me a pull request.
