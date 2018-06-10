//! Functions for reading and writing RDF files.
mod ntriples_writer;
mod turtle;

/// A parser for Turtle files.
pub type TurtleParser<'a, B> = turtle::parser::TurtleParser<'a, B>;
/// A parser for `NTriples` files.
pub type NTriplesParser<'a, B> = turtle::parser::TurtleParser<'a, B>;
pub use self::ntriples_writer::write_ntriples;
pub use self::turtle::pretty_turtle_writer::write_pretty_turtle;
pub use self::turtle::turtle_writer::write_turtle;
