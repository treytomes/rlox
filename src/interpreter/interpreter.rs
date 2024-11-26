use crate::{
    debug::HasFileLocation,
    parser::{BinaryOp, Expr, UnaryOp, Visitor},
};

use super::{runtime_error::Interrupt, EnvironmentStack, Object, RuntimeError};

pub struct Interpreter {
    environments: EnvironmentStack,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environments: EnvironmentStack::new(),
        }
    }

    fn store_result(
        &mut self,
        loc: &dyn HasFileLocation,
        result: Object,
    ) -> Result<(), RuntimeError> {
        // TODO: Only store _ globally.
        if !self.environments.is_defined("_") {
            self.environments.define_global(loc, "_", result)?;
        } else {
            self.environments.assign(loc, "_", result)?;
        }
        Ok(())
    }

    pub fn eval(&mut self, expr: &Expr) -> Result<Object, RuntimeError> {
        expr.accept(self)
    }
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
            UnaryOp::Not => Ok(Object::Boolean(e.is_falsy())),
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

        match op {
            BinaryOp::LogicalAnd => {
                if !left.is_truthy() {
                    return Ok(left);
                }
                return Ok(e2.accept(self)?);
            }
            BinaryOp::LogicalOr => {
                if left.is_truthy() {
                    return Ok(left);
                }
                return Ok(e2.accept(self)?);
            }
            _ => {}
        }

        let right = e2.accept(self)?;

        match op {
            BinaryOp::Add => {
                if let (Object::Number(left), Object::Number(right)) = (left.clone(), right.clone())
                {
                    Ok(Object::Number(left + right))
                } else if let (Object::String(left), Object::String(right)) =
                    (left.clone(), right.clone())
                {
                    Ok(Object::String(format!("{}{}", left, right)))
                } else if let (Object::String(left), Object::Number(right)) = (left, right) {
                    Ok(Object::String(format!("{}{}", left, right)))
                } else {
                    Err(RuntimeError::new(
                        "operand mismatch; second operand must be a number if the first one is",
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
                if let (Object::Number(left), Object::Number(right)) = (left.clone(), right.clone())
                {
                    Ok(Object::Number(left * right))
                } else if let (Object::String(left), Object::Number(right)) = (left, right) {
                    // Raise a runtime error if the right operand is not an integer
                    if right.fract() != 0.0 {
                        return Err(RuntimeError::new(
                            "right operand must be an integer",
                            loc.get_line(),
                            loc.get_column(),
                        ));
                    }

                    let mut s = String::new();
                    for _ in 0..right as usize {
                        s.push_str(&left);
                    }
                    Ok(Object::String(s))
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
                    if right == 0.0 {
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
            BinaryOp::Eq => Ok(Object::Boolean(left.is_equal(&right))),
            BinaryOp::Ne => Ok(Object::Boolean(left.is_not_equal(&right))),
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
            _ => Err(RuntimeError::new(
                "binary operation expected",
                loc.get_line(),
                loc.get_column(),
            )),
        }
    }

    fn visit_print(
        &mut self,
        _loc: &dyn HasFileLocation,
        expr: &Box<Expr>,
    ) -> Result<Object, RuntimeError> {
        let value = expr.accept(self)?;
        print!("{}", value);
        Ok(Object::Nil)
    }

    fn visit_if(
        &mut self,
        _loc: &dyn HasFileLocation,
        cond: &Box<Expr>,
        then: &Box<Expr>,
        else_: &Option<Box<Expr>>,
    ) -> Result<Object, RuntimeError> {
        let cond = cond.accept(self)?;
        if cond.is_truthy() {
            then.accept(self)
        } else if let Some(else_) = else_ {
            else_.accept(self)
        } else {
            Ok(Object::Nil)
        }
    }

    fn visit_let(
        &mut self,
        loc: &dyn HasFileLocation,
        name: &String,
    ) -> Result<Object, RuntimeError> {
        self.environments.define(loc, name, Object::Nil)
    }

    fn visit_let_init(
        &mut self,
        loc: &dyn HasFileLocation,
        name: &String,
        expr: &Box<Expr>,
    ) -> Result<Object, RuntimeError> {
        let value: Object = expr.accept(self)?;
        self.environments.define(loc, &name, value)
    }

    fn visit_assign(
        &mut self,
        loc: &dyn HasFileLocation,
        name: &String,
        expr: &Box<Expr>,
    ) -> Result<Object, RuntimeError> {
        let value = expr.accept(self)?;
        self.environments.assign(loc, name, value)
    }

    fn visit_variable(
        &mut self,
        loc: &dyn HasFileLocation,
        name: &String,
    ) -> Result<Object, RuntimeError> {
        self.environments.get(loc, name)
    }

    fn visit_program(
        &mut self,
        loc: &dyn HasFileLocation,
        exprs: &Vec<Expr>,
    ) -> Result<Object, RuntimeError> {
        let mut last = Object::Nil;
        for expr in exprs {
            last = expr.accept(self)?;
            self.store_result(loc, last.clone())?;
        }
        Ok(last)
    }

    fn visit_block(
        &mut self,
        loc: &dyn HasFileLocation,
        exprs: &Vec<Expr>,
    ) -> Result<Object, RuntimeError> {
        self.environments.enter_scope();
        let mut last = Object::Nil;
        for expr in exprs {
            last = expr.accept(self)?;
            self.store_result(loc, last.clone())?;
        }
        self.environments.leave_scope(loc)?;
        Ok(last)
    }

    fn visit_while(
        &mut self,
        loc: &dyn HasFileLocation,
        cond: &Box<Expr>,
        body: &Box<Expr>,
    ) -> Result<Object, RuntimeError> {
        let mut last = Object::Nil;
        // The `cond`-ition needs to be re-accepted / re-evaluated at the end of each iteration.
        while cond.accept(self)?.is_truthy() {
            match body.accept(self) {
                Ok(value) => last = value,
                Err(e) => {
                    if let Some(int) = e.interrupt {
                        match int {
                            Interrupt::Break => break,
                            Interrupt::Continue => continue,
                        }
                    } else {
                        return Err(e);
                    }
                }
            }

            self.store_result(loc, last.clone())?;
        }

        // Return the final result.
        Ok(last)
    }

    fn visit_break(&mut self, loc: &dyn HasFileLocation) -> Result<Object, RuntimeError> {
        Err(RuntimeError::break_loop())
    }

    fn visit_continue(&mut self, loc: &dyn HasFileLocation) -> Result<Object, RuntimeError> {
        Err(RuntimeError::continue_loop())
    }
}
