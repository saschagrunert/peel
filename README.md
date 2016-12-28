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


