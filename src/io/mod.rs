mod turtle;
mod ntriples_writer;

pub type TurtleParser<'a, B> = turtle::parser::TurtleParser<'a, B>;
// NTriples is a subset of Turtle
pub type NTriplesParser<'a, B> = turtle::parser::TurtleParser<'a, B>;
pub use self::ntriples_writer::write_ntriples;
pub use self::turtle::turtle_writer::write_turtle;
