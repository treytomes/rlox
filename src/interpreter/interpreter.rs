use crate::{
    debug::HasFileLocation,
    parser::{BinaryOp, Expr, UnaryOp, Visitor},
};

use super::{Object, RuntimeError};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn eval(&mut self, expr: &Expr) -> Result<Object, RuntimeError> {
        expr.accept(self)
    }
}

fn is_truthy(object: &Object) -> bool {
    match object {
        Object::Nil => false,
        Object::Boolean(b) => *b,
        _ => true,
    }
}

fn is_falsy(object: &Object) -> bool {
    !is_truthy(object)
}

fn is_equal(a: &Object, b: &Object) -> bool {
    match (a, b) {
        (Object::Nil, Object::Nil) => true,
        (Object::Boolean(a), Object::Boolean(b)) => a == b,
        (Object::Number(a), Object::Number(b)) => a == b,
        (Object::String(a), Object::String(b)) => a == b,
        _ => false,
    }
}

fn is_not_equal(a: &Object, b: &Object) -> bool {
    !is_equal(a, b)
}

impl Visitor<Result<Object, RuntimeError>> for Interpreter {
    fn visit_number(
        &mut self,
        _loc: &dyn HasFileLocation,
        n: &f64,
    ) -> Result<Object, RuntimeError> {
        Ok(Object::Number(*n))
    }

    fn visit_string(
        &mut self,
        _loc: &dyn HasFileLocation,
        s: &String,
    ) -> Result<Object, RuntimeError> {
        Ok(Object::String(s.clone()))
    }

    fn visit_boolean(
        &mut self,
        _loc: &dyn HasFileLocation,
        b: &bool,
    ) -> Result<Object, RuntimeError> {
        Ok(Object::Boolean(*b))
    }

    fn visit_nil(&mut self, _loc: &dyn HasFileLocation) -> Result<Object, RuntimeError> {
        Ok(Object::Nil)
    }

    fn visit_grouping(
        &mut self,
        _loc: &dyn HasFileLocation,
        e: &Box<Expr>,
    ) -> Result<Object, RuntimeError> {
        e.accept(self)
    }

    fn visit_unary_op(
        &mut self,
        loc: &dyn HasFileLocation,
        op: &UnaryOp,
        e: &Box<Expr>,
    ) -> Result<Object, RuntimeError> {
        let e = e.accept(self)?;

        match op {
            UnaryOp::Neg => {
                if let Object::Number(n) = e {
                    Ok(Object::Number(-n))
                } else {
                    Err(RuntimeError::new(
                        "operand must be a number",
                        loc.get_line(),
                        loc.get_column(),
                    ))
                }
            }
            UnaryOp::Not => Ok(Object::Boolean(is_falsy(&e))),
        }
    }

    fn visit_binary_op(
        &mut self,
        loc: &dyn HasFileLocation,
        op: &BinaryOp,
        e1: &Box<Expr>,
        e2: &Box<Expr>,
    ) -> Result<Object, RuntimeError> {
        let left = e1.accept(self)?;
        let right = e2.accept(self)?;

        match op {
            BinaryOp::Add => {
                if let (Object::Number(left), Object::Number(right)) = (left.clone(), right.clone())
                {
                    Ok(Object::Number(left + right))
                } else if let (Object::String(left), Object::String(right)) = (left, right) {
                    Ok(Object::String(format!("{}{}", left, right)))
                } else {
                    Err(RuntimeError::new(
                        "operands must be two numbers or two strings",
                        loc.get_line(),
                        loc.get_column(),
                    ))
                }
            }
            BinaryOp::Sub => {
                if let (Object::Number(left), Object::Number(right)) = (left, right) {
                    Ok(Object::Number(left - right))
                } else {
                    Err(RuntimeError::new(
                        "operands must be numbers",
                        loc.get_line(),
                        loc.get_column(),
                    ))
                }
            }
            BinaryOp::Mul => {
                if let (Object::Number(left), Object::Number(right)) = (left, right) {
                    Ok(Object::Number(left * right))
                } else {
                    Err(RuntimeError::new(
                        "operands must be numbers",
                        loc.get_line(),
                        loc.get_column(),
                    ))
                }
            }
            BinaryOp::Div => {
                if let (Object::Number(left), Object::Number(right)) = (left, right) {
                    if (right == 0.0) {
                        Ok(Object::NaN)
                    } else {
                        Ok(Object::Number(left / right))
                    }
                } else {
                    Err(RuntimeError::new(
                        "operands must be numbers",
                        loc.get_line(),
                        loc.get_column(),
                    ))
                }
            }
            BinaryOp::Eq => Ok(Object::Boolean(is_equal(&left, &right))),
            BinaryOp::Ne => Ok(Object::Boolean(is_not_equal(&left, &right))),
            BinaryOp::Lt => {
                if let (Object::Number(left), Object::Number(right)) = (left, right) {
                    Ok(Object::Boolean(left < right))
                } else {
                    Err(RuntimeError::new(
                        "operands must be numbers",
                        loc.get_line(),
                        loc.get_column(),
                    ))
                }
            }
            BinaryOp::Le => {
                if let (Object::Number(left), Object::Number(right)) = (left, right) {
                    Ok(Object::Boolean(left <= right))
                } else {
                    Err(RuntimeError::new(
                        "operands must be numbers",
                        loc.get_line(),
                        loc.get_column(),
                    ))
                }
            }
            BinaryOp::Gt => {
                if let (Object::Number(left), Object::Number(right)) = (left, right) {
                    Ok(Object::Boolean(left > right))
                } else {
                    Err(RuntimeError::new(
                        "operands must be numbers",
                        loc.get_line(),
                        loc.get_column(),
                    ))
                }
            }
            BinaryOp::Ge => {
                if let (Object::Number(left), Object::Number(right)) = (left, right) {
                    Ok(Object::Boolean(left >= right))
                } else {
                    Err(RuntimeError::new(
                        "operands must be numbers",
                        loc.get_line(),
                        loc.get_column(),
                    ))
                }
            }
        }
    }
}
