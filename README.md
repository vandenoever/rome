[![Build Status](https://travis-ci.org/vandenoever/rome.svg?branch=master)](https://travis-ci.org/vandenoever/rome)
[![Current Version](http://meritbadge.herokuapp.com/rome)](https://crates.io/crates/rome)

**Rome** is an **RDF library** written in safe Rust.

[Documentation](https://www.vandenoever.info/rust/rome/)

# Features

- Access any data in a uniform way as RDF by implementing a Graph.
- Read/write Turtle and N-Triple files.
- Iterate over triples in graphs.
- Wrap a graph in code generated from an ontology.
- Use the type system to distinguish between blank nodes, IRIs and literals at
  compile time.

# Testing

The Turtle parser passes the [W3 test suite](https://www.w3.org/2013/TurtleTests/).

# License

Rome is licensed under AGPLv3.0 or any later version.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the AGPLv3.0 license, shall be licensed as above, without any additional terms or conditions.
