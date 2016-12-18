// placeholder for a potential parser that has low memory overhead for large
// files

use nom::Move;
use nom::Producer;
use nom::FileProducer;
use nom::ConsumerState;
use nom::Consumer;
use nom::Input;

#[derive(PartialEq,Eq,Debug,Clone)]
pub enum State {
    Beginning,
    End,
    Done,
    Error,
}

pub struct TurtleConsumer {
    c_state: ConsumerState<usize, (), Move>,
    state: State,
}

impl TurtleConsumer {
    pub fn new() -> Self {
        TurtleConsumer {
            state: State::Beginning,
            c_state: ConsumerState::Continue(Move::Consume(0)),
        }
    }
}

impl<'a> Consumer<&'a [u8], usize, (), Move> for TurtleConsumer {
    fn state(&self) -> &ConsumerState<usize, (), Move> {
        &self.c_state
    }
    fn handle(&mut self, _: Input<&'a [u8]>) -> &ConsumerState<usize, (), Move> {
        &self.c_state
    }
}

pub struct TurtleStreamer {
    file_producer: FileProducer,
    consumer: TurtleConsumer,
}

impl TurtleStreamer {
    pub fn new(file: &str) -> Result<Self, String> {
        match FileProducer::new(file, 5000) {
            Ok(producer) => {
                Ok(TurtleStreamer {
                    file_producer: producer,
                    consumer: TurtleConsumer::new(),
                })
            }
            Err(_) => Err(format!("Could not create FileProducer for {:?}", file)),
        }
    }
}

impl Iterator for TurtleStreamer {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.file_producer.apply(&mut self.consumer);
        match self.consumer.state {
            State::Error => None,
            _ => Some(String::new()),
        }
    }
}
