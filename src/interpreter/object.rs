use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::String(s) => write!(f, "{}", s),
            Object::Number(n) => write!(f, "{}", n),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Nil => write!(f, "nil"),
        }
    }
}
