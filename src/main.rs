mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let uri = args.get(1).unwrap();
    dbg!(uri);
    parser::parse(uri);
}
