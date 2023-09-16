use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Bool(bool),
    Integer(i64),
    String(String),
    List(Vec<Value>),
    Map(HashMap<Value, Value>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    Bool,
    Integer,
    String,
    List,
    Map,
}

impl std::hash::Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Value::Bool(b) => b.hash(state),
            Value::Integer(i) => i.hash(state),
            Value::String(s) => s.hash(state),
            Value::List(l) => {
                for item in l {
                    item.hash(state);
                }
            }
            Value::Map(m) => {
                for (k, v) in m {
                    k.hash(state);
                    v.hash(state);
                }
            }
        }
    }
}

impl Value {
    fn type_of(&self) -> Type {
        match self {
            Value::Bool(_) => Type::Bool,
            Value::Integer(_) => Type::Integer,
            Value::String(_) => Type::String,
            Value::List(_) => Type::List,
            Value::Map(_) => Type::Map,
        }
    }
}

impl From<HashMap<Value, Value>> for Value {
    fn from(value: HashMap<Value, Value>) -> Self {
        Value::Map(value)
    }
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

#[derive(Debug, Clone)]
pub enum Expr {
    Var(String, Value),
    Value(Value),
}

impl From<Value> for Expr {
    fn from(value: Value) -> Self {
        Expr::Value(value)
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expr::Var(self_name, _), Expr::Var(other_name, _)) => self_name == other_name,
            (Expr::Value(self_val), Expr::Value(other_val)) => self_val == other_val,
            _ => false,
        }
    }
}

#[allow(dead_code)]
impl Expr {
    fn type_of(&self) -> Type {
        match self {
            Expr::Var(_, value) => value.type_of(),
            Expr::Value(value) => value.type_of(),
        }
    }
}

#[derive(Debug, Clone)]
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
        while self.is_expr() {
            if self.is_var_dec() {
                exprs.push(self.consume_var());
            } else if self.is_value() {
                exprs.push(self.consume_value().into());
            }
            self.skip_whitespace();
            self.consume_char(';');
            self.skip_whitespace();
        }
        self.skip_whitespace();
        if !self.sanity_check() {
            panic!("Did not parse all the way to the end");
        }
        exprs
    }

    fn is_expr(&self) -> bool {
        self.is_var_dec() || self.is_value()
    }

    fn sanity_check(&self) -> bool {
        self.index == self.body.len()
    }

    fn is_bool(&self) -> bool {
        self.is_true() || self.is_false()
    }

    fn is_true(&self) -> bool {
        self.peek(4) == Some(&['t', 'r', 'u', 'e'])
    }

    fn is_false(&self) -> bool {
        self.peek(5) == Some(&['f', 'a', 'l', 's', 'e'])
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
            Some(&self.body[self.index..self.index + offset])
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

    fn consume_map_entry(&mut self) -> (Value, Value) {
        self.skip_whitespace();
        let key = self.consume_value();
        self.skip_whitespace();
        self.consume_char(':');
        self.skip_whitespace();
        let val = self.consume_value();
        self.skip_whitespace();
        (key, val)
    }

    fn consume_map(&mut self) -> Value {
        self.consume_char('{');
        self.skip_whitespace();
        let mut hashmap = HashMap::new();
        while self.is_value() && self.curr_char() != Some('}') {
            let (key, val) = self.consume_map_entry();
            hashmap.insert(key, val);
            self.consume_char(',');
            self.skip_whitespace();
        }
        self.consume_char('}');
        self.skip_whitespace();
        Value::from(hashmap)
    }

    fn is_var_dec(&self) -> bool {
        self.peek(3) == Some(&['l', 'e', 't'])
    }

    fn consume_var_dec(&mut self) {
        self.skip(3);
    }

    fn is_value(&self) -> bool {
        self.is_integer() || self.is_string() || self.is_bool() || self.is_list() || self.is_map()
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
        } else if self.is_map() {
            self.consume_map()
        } else {
            panic!("Could not consume value");
        }
    }

    fn is_map(&self) -> bool {
        self.curr_char() == Some('{')
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

    fn test(input: &str, expected: Vec<Expr>) {
        let mut parser = Parser::new(input);
        assert_eq!(parser.parse(), expected);
    }

    fn test_types(input: &str, expected: Vec<Type>) {
        let mut parser = Parser::new(input);
        assert_eq!(
            parser
                .parse()
                .into_iter()
                .map(|x| x.type_of())
                .collect::<Vec<Type>>(),
            expected
        );
    }

    #[test]
    fn parse_int_var() {
        let input = "let x = 10;";
        test(input, vec![Expr::Var("x".to_string(), Value::from(10))]);
    }

    #[test]
    fn parse_string_var() {
        let input = "let x = \"xd\";";
        test(
            input,
            vec![Expr::Var("x".to_string(), Value::from("xd".to_string()))],
        );
    }

    #[test]
    fn parse_true_var() {
        let input = "let x = true;";
        test(input, vec![Expr::Var("x".to_string(), Value::from(true))]);
    }

    #[test]
    fn parse_false_var() {
        let input = "let x = false;";
        test(input, vec![Expr::Var("x".to_string(), Value::from(false))]);
    }

    #[test]
    fn parse_list() {
        let input = "let x = [1,2,3];";
        test(
            input,
            vec![Expr::Var(
                "x".to_string(),
                Value::from(vec![1.into(), 2.into(), 3.into()]),
            )],
        );
    }

    #[test]
    fn parse_map() {
        let input = "{ \"key\" : 2, [1] : 3 }";
        test(
            input,
            vec![Expr::Value(Value::from(HashMap::from([
                ("key".into(), 2.into()),
                (vec![1.into()].into(), 3.into()),
            ])))],
        );
    }

    #[test]
    fn parse_sublist() {
        let input = "let x = [[1],2,3];";
        test(
            input,
            vec![Expr::Var(
                "x".to_string(),
                Value::from(vec![Value::from(vec![1.into()]), 2.into(), 3.into()]),
            )],
        );
    }

    #[test]
    fn sublist_type() {
        let input = "let x = [[1],2,3];";
        test_types(input, vec![Type::List]);
    }
}
