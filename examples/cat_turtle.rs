use rome::graph::GraphWriter;
use rome::graphs::tel;
use rome::io::write_pretty_turtle;
use rome::io::TurtleParser;
use rome::namespaces::Namespaces;
use std::env::args;
use std::fs;
use std::io;
use std::io::Read;

fn read_file(path: &str) -> io::Result<String> {
    let mut f = fs::File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn load_file(input: &str) -> rome::Result<(Namespaces, tel::Graph128)> {
    let mut writer = tel::GraphCreator::with_capacity(65000);
    let mut prefixes = Namespaces::new();
    let data = read_file(input)?;
    let mut base = String::from("file:");
    base.push_str(input);
    {
        let mut triples = TurtleParser::new(data.as_str(), &base, &mut writer)?;
        while let Some(step) = triples.next() {
            step?;
        }
        for ns in triples.prefixes().iter() {
            prefixes.set(ns.prefix(), ns.namespace());
        }
    }
    let graph = writer.collect();
    Ok((prefixes, graph))
}

fn main() {
    let mut args = args();
    args.next();
    let input = args.next().unwrap();
    let (mut ns, graph) = load_file(&input).expect(&format!("cannot read graph {}", &input));
    let mut base = String::from("file:");
    base.push_str(&input);
    base.push('#');
    ns.insert("", base);
    write_pretty_turtle(&ns, &graph, &mut ::std::io::stdout()).expect("Cannot write graph.");
}
