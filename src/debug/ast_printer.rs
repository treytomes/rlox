use super::HasFileLocation;
use crate::parser::{BinaryOp, Expr, UnaryOp, Visitor};

pub struct AstPrinter {
    indent_level: usize,
}

impl AstPrinter {
    pub fn new() -> Self {
        Self { indent_level: 0 }
    }

    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_number(&mut self, _loc: &dyn HasFileLocation, n: &f64) -> String {
        n.to_string()
    }

    fn visit_string(&mut self, _loc: &dyn HasFileLocation, s: &String) -> String {
        format!(
            "\"{}\"",
            s.clone()
                .replace("\r", "\\r")
                .replace("\n", "\\n")
                .replace("\t", "\\t")
        )
    }

    fn visit_boolean(&mut self, _loc: &dyn HasFileLocation, b: &bool) -> String {
        b.to_string()
    }

    fn visit_nil(&mut self, _loc: &dyn HasFileLocation) -> String {
        "nil".to_string()
    }

    fn visit_grouping(&mut self, _loc: &dyn HasFileLocation, e: &Box<Expr>) -> String {
        format!("(group {})", e.accept(self))
    }

    fn visit_unary_op(
        &mut self,
        _loc: &dyn HasFileLocation,
        op: &UnaryOp,
        e: &Box<Expr>,
    ) -> String {
        match op {
            UnaryOp::Neg => format!("(- {})", e.accept(self)),
            UnaryOp::Not => format!("(! {})", e.accept(self)),
        }
    }

    fn visit_binary_op(
        &mut self,
        _loc: &dyn HasFileLocation,
        op: &BinaryOp,
        e1: &Box<Expr>,
        e2: &Box<Expr>,
    ) -> String {
        match op {
            BinaryOp::Add => format!("(+ {} {})", e1.accept(self), e2.accept(self)),
            BinaryOp::Sub => format!("(- {} {})", e1.accept(self), e2.accept(self)),
            BinaryOp::Mul => format!("(* {} {})", e1.accept(self), e2.accept(self)),
            BinaryOp::Div => format!("(/ {} {})", e1.accept(self), e2.accept(self)),
            BinaryOp::Eq => format!("(== {} {})", e1.accept(self), e2.accept(self)),
            BinaryOp::Ne => format!("(!= {} {})", e1.accept(self), e2.accept(self)),
            BinaryOp::Lt => format!("(< {} {})", e1.accept(self), e2.accept(self)),
            BinaryOp::Le => format!("(<= {} {})", e1.accept(self), e2.accept(self)),
            BinaryOp::Gt => format!("(> {} {})", e1.accept(self), e2.accept(self)),
            BinaryOp::Ge => format!("(>= {} {})", e1.accept(self), e2.accept(self)),
        }
    }

    fn visit_print(&mut self, _loc: &dyn HasFileLocation, expr: &Box<Expr>) -> String {
        format!("(print {})", expr.accept(self))
    }

    fn visit_if(
        &mut self,
        _loc: &dyn HasFileLocation,
        cond: &Box<Expr>,
        then: &Box<Expr>,
        else_: &Option<Box<Expr>>,
    ) -> String {
        match else_ {
            Some(else_) => format!(
                "(if {} {} {})",
                cond.accept(self),
                then.accept(self),
                else_.accept(self)
            ),
            None => format!("(if {} {})", cond.accept(self), then.accept(self)),
        }
    }

    fn visit_let(&mut self, _loc: &dyn HasFileLocation, name: &String) -> String {
        format!("(let {})", name)
    }

    fn visit_let_init(
        &mut self,
        _loc: &dyn HasFileLocation,
        name: &String,
        expr: &Box<Expr>,
    ) -> String {
        format!("(let {} {})", name, expr.accept(self))
    }

    fn visit_assign(
        &mut self,
        _loc: &dyn HasFileLocation,
        name: &String,
        expr: &Box<Expr>,
    ) -> String {
        format!("(= {} {})", name, expr.accept(self))
    }

    fn visit_variable(&mut self, _loc: &dyn HasFileLocation, name: &String) -> String {
        format!("(var {})", name)
    }

    fn visit_program(&mut self, _loc: &dyn HasFileLocation, exprs: &Vec<Expr>) -> String {
        let mut s = String::new();
        s.push_str("(program \r\n");
        self.indent_level += 1;
        for expr in exprs {
            for _ in 0..self.indent_level {
                s.push_str("\t");
            }
            s.push_str(&expr.accept(self));
            s.push_str("\r\n");
        }
        self.indent_level -= 1;
        for _ in 0..self.indent_level {
            s.push_str("\t");
        }
        s.push_str(")");
        s
    }

    fn visit_block(&mut self, _loc: &dyn HasFileLocation, exprs: &Vec<Expr>) -> String {
        let mut s = String::new();
        s.push_str("(block \r\n");
        self.indent_level += 1;
        for expr in exprs {
            for _ in 0..self.indent_level {
                s.push_str("\t");
            }
            s.push_str(&expr.accept(self));
            s.push_str("\r\n");
        }
        self.indent_level -= 1;
        for _ in 0..self.indent_level {
            s.push_str("\t");
        }
        s.push_str(")");
        s
    }
}
