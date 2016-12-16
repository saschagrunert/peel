# peel [![Build Status](https://travis-ci.org/saschagrunert/peel.svg)](https://travis-ci.org/saschagrunert/peel) [![Coverage Status](https://coveralls.io/repos/github/saschagrunert/peel/badge.svg?branch=master)](https://coveralls.io/github/saschagrunert/peel?branch=master) [![doc peel](https://img.shields.io/badge/doc-peel-blue.svg)](https://saschagrunert.github.io/peel) [![License MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/saschagrunert/peel/blob/master/LICENSE)
## Dynamic packet parsing within trees 🌲 🌳 🌴
Target of this library is to provide a flexible parsing approach for network packets. This will be done within
[arena](https://en.wikipedia.org/wiki/Region-based_memory_management) based [parser trees](https://en.wikipedia.org/wiki/Parse_tree)
which can be modified during runtime. Other stacks beside the well known TCP/IP protocol family should be parsable too.

### Example usage
```rust
use peel::prelude::*;

// Get the default tree based on the TCP/IP stack
let peel = default_peel();

// Traverse the parser tree. If a parser matches check for available
// child parsers. Stop parsing if there are no childs left. In this
// example no parser would match because the input is no valid Ethernet
// packet. The `vec![]` memory will be used for the resulting stack of
// `Layer`s.
let result = peel.traverse(&[0xff; 500], vec![]).unwrap();

assert_eq!(result.len(), 0);
```
