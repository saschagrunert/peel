# peel
[![Build Status](https://travis-ci.org/saschagrunert/peel.svg)](https://travis-ci.org/saschagrunert/peel) [![Build status](https://ci.appveyor.com/api/projects/status/i67yq6yij2k17iwc?svg=true)](https://ci.appveyor.com/project/saschagrunert/peel) [![Coverage Status](https://coveralls.io/repos/github/saschagrunert/peel/badge.svg?branch=master)](https://coveralls.io/github/saschagrunert/peel?branch=master) [![master doc peel](https://img.shields.io/badge/master_doc-peel-blue.svg)](https://saschagrunert.github.io/peel) [![License MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/saschagrunert/peel/blob/master/LICENSE) [![Crates.io](https://img.shields.io/crates/v/peel.svg)](https://crates.io/crates/peel) [![doc.rs](https://docs.rs/peel/badge.svg)](https://docs.rs/peel)
## Dynamic parsing within trees 🌲 🌳 🌴
Target of this library is to provide a flexible parsing approach for network packets. This will be done within
[arena](https://en.wikipedia.org/wiki/Region-based_memory_management) based [parser trees](https://en.wikipedia.org/wiki/Parse_tree)
which can be modified during runtime.

An example included within this crate is the parsing of the 
[Internet Protocol Suite](https://en.wikipedia.org/wiki/Internet_protocol_suite). Beside this, it is possible to build
own parser trees or include a custom parser within an already existing tree.

### Example usage
```rust
use peel::prelude::*;

// Get the default tree based on the TCP/IP stack
let peel = peel_tcp_ip();

let eth_header = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 8, 0];

// Traverse the parser tree. If a parser matches check for available child parsers.
// Stop parsing if there are no childs left. The `vec![]` memory will be used for
// the resulting stack of `Layer`s.
let result = peel.traverse(&eth_header, vec![]).unwrap();

// There should be one parsed EthernetPacket in:
assert_eq!(result.len(), 1);
```
