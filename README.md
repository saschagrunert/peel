# peel
[![Build Status](https://travis-ci.org/saschagrunert/peel.svg)](https://travis-ci.org/saschagrunert/peel) [![Build status](https://ci.appveyor.com/api/projects/status/i67yq6yij2k17iwc?svg=true)](https://ci.appveyor.com/project/saschagrunert/peel) [![Coverage Status](https://coveralls.io/repos/github/saschagrunert/peel/badge.svg?branch=master)](https://coveralls.io/github/saschagrunert/peel?branch=master) [![master doc peel](https://img.shields.io/badge/master_doc-peel-blue.svg)](https://saschagrunert.github.io/peel) [![License MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/saschagrunert/peel/blob/master/LICENSE) [![Crates.io](https://img.shields.io/crates/v/peel.svg)](https://crates.io/crates/peel) [![doc.rs](https://docs.rs/peel/badge.svg)](https://docs.rs/peel)
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
    p.link_new_parser(parser_1, Parser2);

    // Append Parser3 to Parser1
    let parser_3 = p.link_new_parser(parser_1, Parser3);

    // Append Parser4 to Parser3
    p.link_new_parser(parser_3, Parser4);

    p
}
```

The first created parser will automatically be the root parser and the entry point for the tree traversal. Every parser
returns an actual result, which will be pushed into a vector. This means for our example that the result is an enum of
different types:

```rust
/// Return values of the parsers
pub enum ParserResult {
    /// The result of the first example parser
    Result1(bool),

    /// The result of the second example parser
    Result2(bool),

    /// The result of the third example parser
    Result3(bool),

    /// The result of the fourth example parser
    Result4(bool),
}
```

Beside this result a `ParserState` is needed to make a decision about the next parsing step:

```rust
/// Possible actions to be done if a parser succeed
pub enum ParserState {
    /// Default behavior, continue traversing the Parser tree with the next child
    ContinueWithFirstChild,

    /// Continue traversing with the next sibling of the current parser
    ContinueWithNextSibling,

    /// Continue traversing with the current parser
    ContinueWithCurrent,

    /// Immediately stop the parsing
    Stop,
}
```

So in our example image above we have the following available stages:
- *Parser 1*:
    - Succeed: `ContinueWithFirstChild`
    - Failed: Return an error
- *Parser 2*:
    - Succeed: Overall parsing done, because no child parsers left
    - Failed: `ContinueWithNextSibling`
- *Parser 3*:
    - Succeed: `ContinueWithFirstChild`
    - Failed: `ContinueWithCurrent`
- *Parser 4*:
    - Failed/Succeed: Overall parsing done, because no child parsers left

