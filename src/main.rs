use type_inference::*;

fn main() {
    let input = "let x: bool | str = false;";
    let mut parser = Parser::new(input);
    parser.parse();
    dbg!(parser);
}
