extern crate rdfio;
use std::env::args;
use rdfio::run;

fn main() {
    let mut args = args();
    args.next();
    for arg in args {
        if let Err(e) = run(&arg) {
            println!("{:?}", e);
        }
    }
}
