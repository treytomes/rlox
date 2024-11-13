use super::{BinaryOp, Expr, UnaryOp};

pub trait Visitor<R> {
    fn visit_number(&mut self, n: &f64) -> R;
    fn visit_string(&mut self, s: &String) -> R;
    fn visit_boolean(&mut self, b: &bool) -> R;
    fn visit_nil(&mut self) -> R;
    fn visit_grouping(&mut self, e: &Box<Expr>) -> R;
    fn visit_unary_op(&mut self, op: &UnaryOp, e: &Box<Expr>) -> R;
    fn visit_binary_op(&mut self, op: &BinaryOp, e1: &Box<Expr>, e2: &Box<Expr>) -> R;
}
