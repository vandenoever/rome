mod turtle;
mod ntriples_writer;

pub type TurtleParser<'a> = turtle::parser::TripleIterator<'a>;
// NTriples is a subset of Turtle
pub type NTriplesParser<'a> = turtle::parser::TripleIterator<'a>;
pub use self::turtle::turtle_writer::write_turtle;
pub use self::ntriples_writer::write_ntriples;
