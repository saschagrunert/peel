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
different types (in this case only `bool` values for simplicity):

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
    - Succeed/Failed: `ContinueWithNextSibling`
- *Parser 3*:
    - Succeed:
        - Internal pattern matched: `ContinueWithCurrent`
        - Internal Pattern not matched: `ContinueWithFirstChild`
    - Failed: Overall parsing done, because no siblings left
- *Parser 4*:
    - Failed/Succeed: Overall parsing done, because no child parsers left

This means that the traversal method of `Peel` will try to find the deepest possible path within the tree structure,
whereas the parsers itself can tell `Peel` how to continue beside the default `ContinueWithFirstChild` behavior.

After the creation of the structure the traversal can begin:

```rust
let mut peel = peel_example();
peel.set_log_level(LogLevel::Trace);
let result = peel.traverse(b"1234", vec![]).unwrap();

assert_eq!(result.len(), 5);
println!("{:?}", result);
```

With the help of the [log](https://crates.io/crates/log) crate it will output:
```
[peel] [INFO ] Log level set to: Trace
[peel] [DEBUG] Parser 1 parsing succeed, left input length: 3
[peel] [DEBUG] Continue traversal to first child of the parser
[peel] [DEBUG] Parser 2 parsing succeed, left input length: 2
[peel] [DEBUG] Continue traversal to next sibling of the parser
[peel] [DEBUG] Parser 3 parsing succeed, left input length: 1
[peel] [DEBUG] Trying the current parser again
[peel] [DEBUG] Parser 3 parsing succeed, left input length: 1
[peel] [DEBUG] Continue traversal to first child of the parser
[peel] [DEBUG] Parser 4 parsing succeed, left input length: 0
[Result1(true), Result2(true), Result3(true), Result3(true), Result4(true)]
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
                 node: Option<&ExampleNode>,         // The current node within the tree
                 arena: Option<&ExampleArena>,       // Access to possible other nodes via the arena
                 result: Option<&Vec<Self::Result>>) // The current parsing result
                 -> IResult<&'a [u8], (Self::Result, ParserState)> {
        do_parse!(input,
            tag!("1") >>
            (ParserResult::Result1(true), ParserState::ContinueWithFirstChild)
        )
    }

    // Returns the actual parser variant
    fn variant(&self) -> Self::Variant {
        ParserVariant::Variant1(self.clone())
    }
}
```

For event more advanced behavior the `node` and `arena` can be used to find out where the parser is located within the
current structure. Access to the current parsing `result` is possible as well.

## Current limitations
- Result values referencing to the actual input is currently not implemented
- Going back during traversal is not possible

## Contributing
You want to contribute to this project? Wow, thanks! So please just fork it and send me a pull request.
