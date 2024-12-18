use crate::debug::HasFileLocation;

use super::{BinaryOp, Expr, UnaryOp};

pub trait Visitor<R> {
    fn visit_number(&mut self, loc: &dyn HasFileLocation, n: &f64) -> R;
    fn visit_string(&mut self, loc: &dyn HasFileLocation, s: &String) -> R;
    fn visit_boolean(&mut self, loc: &dyn HasFileLocation, b: &bool) -> R;
    fn visit_nil(&mut self, loc: &dyn HasFileLocation) -> R;
    fn visit_grouping(&mut self, loc: &dyn HasFileLocation, e: &Box<Expr>) -> R;
    fn visit_unary_op(&mut self, loc: &dyn HasFileLocation, op: &UnaryOp, e: &Box<Expr>) -> R;
    fn visit_binary_op(
        &mut self,
        loc: &dyn HasFileLocation,
        op: &BinaryOp,
        e1: &Box<Expr>,
        e2: &Box<Expr>,
    ) -> R;
    fn visit_print(&mut self, loc: &dyn HasFileLocation, expr: &Box<Expr>) -> R;
    fn visit_if(
        &mut self,
        loc: &dyn HasFileLocation,
        cond: &Box<Expr>,
        then: &Box<Expr>,
        else_: &Option<Box<Expr>>,
    ) -> R;
    fn visit_let(&mut self, loc: &dyn HasFileLocation, name: &String) -> R;
    fn visit_let_init(&mut self, loc: &dyn HasFileLocation, name: &String, expr: &Box<Expr>) -> R;
    fn visit_assign(&mut self, loc: &dyn HasFileLocation, name: &String, expr: &Box<Expr>) -> R;
    fn visit_variable(&mut self, loc: &dyn HasFileLocation, name: &String) -> R;
    fn visit_program(&mut self, loc: &dyn HasFileLocation, exprs: &Vec<Expr>) -> R;
    fn visit_block(&mut self, loc: &dyn HasFileLocation, exprs: &Vec<Expr>) -> R;
    fn visit_while(&mut self, loc: &dyn HasFileLocation, cond: &Box<Expr>, body: &Box<Expr>) -> R;
    fn visit_break(&mut self, loc: &dyn HasFileLocation) -> R;
    fn visit_continue(&mut self, loc: &dyn HasFileLocation) -> R;
}
