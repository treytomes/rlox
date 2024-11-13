mod ast_printer;
mod binary_op;
mod expr;
mod parser;
mod parser_error;
mod unary_op;
mod visitor;

pub use ast_printer::AstPrinter;
pub use binary_op::BinaryOp;
pub use expr::Expr;
pub use parser::parse;
pub use parser_error::ParserError;
pub use unary_op::UnaryOp;
pub use visitor::Visitor;

// pub enum Stmt {
//     Expr(Expr),
//     Print(Expr),
//     Var(String, Expr),
//     Block(Vec<Stmt>),
//     If(Expr, Box<Stmt>, Option<Box<Stmt>>),
//     While(Expr, Box<Stmt>),
//     Break,
//     Function(String, Vec<String>, Box<Stmt>),
//     Return(Expr),
// }
