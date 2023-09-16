use type_inference::*;

fn main() {
    let input = "let x = false;";
    let mut parser = Parser::new(input);
    parser.parse();
}
