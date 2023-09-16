#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Bool(bool),
    Integer(i64),
    String(String),
    List(Vec<Value>),
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Integer(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Value::List(value)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expr {
    Var(String, Value),
    Value(Value),
}

pub struct Parser {
    body: Vec<char>,
    index: usize,
}

impl Parser {
    pub fn new(body: &str) -> Self {
        Self {
            body: body.chars().collect(),
            index: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<Expr> {
        let mut exprs: Vec<Expr> = vec![];
        self.skip_whitespace();
        if self.is_var_dec() {
            exprs.push(self.consume_var());
        }
        self.skip_whitespace();
        if !self.sanity_check() {
            panic!("Did not parse all the way to the end");
        }
        exprs
    }

    fn sanity_check(&self) -> bool {
        self.index == self.body.len()
    }

    fn is_bool(&self) -> bool {
        self.is_true() || self.is_false()
    }

    fn is_true(&self) -> bool {
        self.peek(3) == Some(&['t', 'r', 'u', 'e'])
    }

    fn is_false(&self) -> bool {
        self.peek(4) == Some(&['f', 'a', 'l', 's', 'e'])
    }

    fn skip_whitespace(&mut self) {
        while self.is_in_bounds(0) && self.body[self.index].is_ascii_whitespace() {
            self.index += 1;
        }
    }

    fn is_in_bounds(&self, offset: usize) -> bool {
        self.index + offset < self.body.len()
    }

    fn is_list(&self) -> bool {
        self.curr_char() == Some('[')
    }

    fn consume_list(&mut self) -> Value {
        let mut values = vec![];
        self.consume_char('[');
        while self.curr_char() != Some(']') {
            self.skip_whitespace();
            values.push(self.consume_value());
            self.skip_whitespace();
            self.consume_char(',');
            self.skip_whitespace();
        }

        self.consume_char(']');

        Value::from(values)
    }

    fn peek(&self, offset: usize) -> Option<&[char]> {
        if self.is_in_bounds(offset) {
            Some(&self.body[self.index..=self.index + offset])
        } else {
            None
        }
    }

    fn is_char(&self, c: char) -> bool {
        self.curr_char() == Some(c)
    }

    fn next(&mut self) {
        self.skip(1);
    }

    fn is_string(&self) -> bool {
        self.curr_char() == Some('"')
    }

    fn consume_string(&mut self) -> Value {
        let mut s = String::new();
        self.consume_char('"');
        while !self.is_char('"') {
            s.push(self.curr_char().unwrap());
            self.next();
        }
        self.consume_char('"');
        self.consume_char(';');
        Value::from(s)
    }

    fn skip(&mut self, offset: usize) {
        self.index += offset;
    }

    fn curr_char(&self) -> Option<char> {
        if self.is_in_bounds(0) {
            Some(self.body[self.index])
        } else {
            None
        }
    }

    fn consume_bool(&mut self) -> Value {
        if self.is_true() {
            self.consume_true()
        } else if self.is_false() {
            self.consume_false()
        } else {
            dbg!(self.curr_char());
            panic!("could not consume bool");
        }
    }

    fn consume_true(&mut self) -> Value {
        self.skip(4);
        Value::from(true)
    }

    fn consume_false(&mut self) -> Value {
        self.skip(5);
        Value::from(false)
    }

    fn consume_char(&mut self, char_to_consume: char) -> bool {
        if self.curr_char() == Some(char_to_consume) {
            self.index += 1;
            true
        } else {
            false
        }
    }

    fn consume_var(&mut self) -> Expr {
        self.consume_var_dec();
        self.skip_whitespace();
        let name = self.consume_name();
        self.skip_whitespace();
        self.consume_char('=');
        self.skip_whitespace();
        let value = self.consume_value();
        self.skip_whitespace();
        self.consume_char(';');
        Expr::Var(name, value)
    }

    fn is_var_dec(&self) -> bool {
        self.peek(2) == Some(&['l', 'e', 't'])
    }

    fn consume_var_dec(&mut self) {
        self.skip(3);
    }

    fn consume_value(&mut self) -> Value {
        if self.is_integer() {
            self.consume_integer()
        } else if self.is_string() {
            self.consume_string()
        } else if self.is_bool() {
            self.consume_bool()
        } else if self.is_list() {
            self.consume_list()
        } else {
            panic!("Could not consume value");
        }
    }

    fn is_integer(&self) -> bool {
        self.curr_char().is_some_and(|x| x.is_ascii_digit())
    }

    fn consume_integer(&mut self) -> Value {
        let mut num: i64 = 0;
        let mut digits = vec![];
        while self.is_integer() {
            digits.push(self.curr_char().unwrap().to_digit(10).unwrap());
            self.next();
        }
        digits.reverse();
        for (i, digit) in digits.into_iter().enumerate() {
            num += (digit * 10_u32.pow(i.try_into().unwrap())) as i64;
        }
        Value::from(num)
    }

    fn consume_name(&mut self) -> String {
        let mut name = String::new();
        while self.curr_char().is_some_and(|x| x.is_ascii_alphabetic()) {
            name.push(self.curr_char().unwrap());
            self.next();
        }
        name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_int_var() {
        let input = "let x = 10;";
        let mut parser = Parser::new(input);
        assert_eq!(
            parser.parse(),
            vec![Expr::Var("x".to_string(), Value::from(10))]
        );
    }

    #[test]
    fn parse_string_var() {
        let input = "let x = \"xd\";";
        let mut parser = Parser::new(input);
        assert_eq!(
            parser.parse(),
            vec![Expr::Var("x".to_string(), Value::from("xd".to_string()))]
        );
    }

    #[test]
    fn parse_true_var() {
        let input = "let x = true;";
        let mut parser = Parser::new(input);
        assert_eq!(
            parser.parse(),
            vec![Expr::Var("x".to_string(), Value::from(true))]
        );
    }

    #[test]
    fn parse_false_var() {
        let input = "let x = false;";
        let mut parser = Parser::new(input);
        assert_eq!(
            parser.parse(),
            vec![Expr::Var("x".to_string(), Value::from(false))]
        );
    }

    #[test]
    fn parse_list() {
        let input = "let x = [1,2,3];";
        let mut parser = Parser::new(input);
        assert_eq!(
            parser.parse(),
            vec![Expr::Var(
                "x".to_string(),
                Value::from(vec![1.into(), 2.into(), 3.into()])
            )]
        );
    }
    #[test]
    fn parse_sublist() {
        let input = "let x = [[1],2,3];";
        let mut parser = Parser::new(input);
        assert_eq!(
            parser.parse(),
            vec![Expr::Var(
                "x".to_string(),
                Value::from(vec![Value::from(vec![1.into()]), 2.into(), 3.into()])
            )]
        );
    }
}
