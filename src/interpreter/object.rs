use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    String(String),
    Number(f64),
    Boolean(bool),
    NaN,
    Nil,
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::String(s) => !s.is_empty(),
            Object::Number(n) => *n != 0.0,
            Object::Boolean(b) => *b,
            Object::NaN => false,
            Object::Nil => false,
        }
    }

    pub fn is_falsy(&self) -> bool {
        !self.is_truthy()
    }

    pub fn is_equal(&self, other: &Object) -> bool {
        match (self, other) {
            (Object::String(s1), Object::String(s2)) => s1 == s2,
            (Object::Number(n1), Object::Number(n2)) => n1 == n2,
            (Object::Boolean(b1), Object::Boolean(b2)) => b1 == b2,
            (Object::NaN, Object::NaN) => false,
            (Object::Nil, Object::Nil) => true,
            _ => false,
        }
    }

    pub fn is_not_equal(&self, other: &Object) -> bool {
        !self.is_equal(other)
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::String(s) => write!(f, "{}", s),
            Object::Number(n) => write!(f, "{}", n),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::NaN => write!(f, "NaN"),
            Object::Nil => write!(f, "nil"),
        }
    }
}
