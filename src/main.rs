use type_inference::*;

fn main() {
    let input = "{ \"key\" : 2, [1] : 3 }";
    let mut parser = Parser::new(input);
    parser.parse();
    dbg!(parser);
}
