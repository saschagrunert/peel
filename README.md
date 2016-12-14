# peal 🌲🌳🌴 [![Build Status](https://travis-ci.org/saschagrunert/peal.svg)](https://travis-ci.org/saschagrunert/peal) [![Build status](https://ci.appveyor.com/api/projects/status/i8wd5t9rmtokjrmi?svg=true)](https://ci.appveyor.com/project/saschagrunert/peal) [![Coverage Status](https://coveralls.io/repos/github/saschagrunert/peal/badge.svg?branch=master)](https://coveralls.io/github/saschagrunert/peal?branch=master) [![doc peal](https://img.shields.io/badge/doc-peal-blue.svg)](https://saschagrunert.github.io/peal) [![License MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/saschagrunert/peal/blob/master/LICENSE)
# Dynamic packet parsing within trees
Target of this library is to provide a flexible parsing approach for network packets. This will be done within parser
trees which can be modified during runtime. Other stacks beside the well known TCP/IP protocol family should be parsable
too.

## Example usage
```rust
use peal::prelude::*;

// Get the default tree based on the TCP/IP stack
let mut peal = get_packet_peal();

// Traverse the parser tree. If a parser matches check for available
// child parsers. Stop parsing if there are no childs left. In this
// example no parser would match because the input is no valid Ethernet
// packet. The `vec![]` memory will be used for the resulting stack of
// `Layer`s.
let result = peal.traverse(&[0xff; 500], vec![]).unwrap();

assert_eq!(result.len(), 0);
```
