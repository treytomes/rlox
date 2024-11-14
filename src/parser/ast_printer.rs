use crate::debug::HasFileLocation;

use super::{BinaryOp, Expr, UnaryOp, Visitor};

pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter {}
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
        s.clone()
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
}
