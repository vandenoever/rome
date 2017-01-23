[![Build Status](https://travis-ci.org/vandenoever/rome.svg?branch=master)](https://travis-ci.org/vandenoever/rome)

**Rome** is an **RDF library** written in safe Rust.

# Features

- Access any data in uniform way as RDF by implementing a Graph.
- Read/write Turtle and N-Triple files.
- Iterate over triples in graphs.
- Wrap a graph in code generated from an ontology.
- Use the type system to distinguish between blank nodes, IRIs and literals at
  compile time.

# Testing

The Turtle parser passes the [W3 test suite](https://www.w3.org/2013/TurtleTests/).
