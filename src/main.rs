mod app_info;
mod debug;
mod interpreter;
mod lexer;
mod parser;
mod repl;

use app_info::AppInfo;
use atty::Stream;
use clap::{Arg, Command};
use debug::{HasFileLocation, LocatableError};
use interpreter::{HasStopFlag, Interpreter, Object};
use lexer::scan_tokens;
use parser::{parse, AstPrinter, Expr};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const REPORT_COUNT: bool = false;
const REPORT_TOKENS: bool = false;
const REPORT_AST: bool = false;

// Define a struct to represent the REPL state
struct LoxState {
    interpreter: Interpreter,
    stop_flag: Arc<AtomicBool>,
}

impl HasStopFlag for LoxState {
    fn trigger_stop(&mut self) {
        self.stop_flag.store(true, Ordering::Relaxed);
    }

    fn is_stopped(&self) -> bool {
        self.stop_flag.load(Ordering::Relaxed)
    }
}

fn print_tokens(tokens: &Vec<lexer::Token>) {
    print!("Tokens:\r\n");
    for token in tokens {
        print!("\t{:?}\r\n", token);
    }
}

fn parse_line(input: &str) -> Result<Expr, anyhow::Error> {
    let tokens = scan_tokens(input);
    match tokens {
        Ok(tokens) => {
            if REPORT_TOKENS {
                print_tokens(&tokens);
            }
            let expr = parse(&tokens);
            match expr {
                Ok(expr) => {
                    if REPORT_AST {
                        print!("\r\nexpr: {}\r\n", AstPrinter::new().print(&expr));
                    }
                    return Ok(expr);
                }
                Err(err) => {
                    err.report(input);
                    return Err(anyhow::Error::new(err));
                    //).context("parsing error"));
                }
            }
        }
        Err(err) => {
            err.report(input);
            return Err(anyhow::Error::new(err).context("lexing error"));
        }
    }
}

fn exec_line(input: &str, state: &mut LoxState) {
    if input.trim().is_empty() {
        return;
    }

    let expr = parse_line(input);
    match expr {
        Ok(expr) => {
            let result = state.interpreter.eval(&expr);
            match result {
                Ok(value) => {
                    // print!("result: {}\r\n", value);
                    print!("\r\n{}\r\n", value);
                }
                Err(err) => {
                    err.report(input);
                }
            }
        }
        Err(_err) => {
            // The lexing/parsing errors were reported in the previous stages.
            // eprint!("error: {}\r\n", err);
            return;
        }
    }
}

fn parse_lines(lines: &Vec<String>) -> Result<Vec<Expr>, anyhow::Error> {
    let mut exprs: Vec<Expr> = vec![];
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        exprs.push(parse_line(&line)?);
    }
    Ok(exprs)
}

// TODO: This one won't function correctly until line separators are implemented.
fn exec_lines(lines: &Vec<String>, state: &mut LoxState) {
    let stop_flag = Arc::new(AtomicBool::new(false));
    match parse_lines(lines) {
        Ok(exprs) => {
            let mut final_result = Object::Nil;
            for expr in exprs {
                let result = state.interpreter.eval(&expr);
                match result {
                    Ok(value) => {
                        final_result = value;
                    }
                    Err(err) => {
                        err.report(lines[err.get_line() - 1].as_str());
                        break;
                    }
                }
                if stop_flag.load(Ordering::Relaxed) {
                    break;
                }
            }

            // print!("result: {}\r\n", final_result);
            print!("\r\n{}\r\n", final_result);
        }
        Err(err) => {
            eprint!("\r\nerror: {}\r\n", err);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let expression = Expr::binary_op(
    //     Expr::unary_op(UnaryOp::Neg, Expr::number(123.0)),
    //     BinaryOp::Mul,
    //     Expr::grouping(Expr::number(45.67)),
    // );
    // print!("{}\r\n\r\n", AstPrinter::new().print(&expression));

    let app_info = AppInfo::from_env();

    let matches = Command::new(app_info.name)
        .version(app_info.version)
        .about(app_info.description)
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .help("Path to a file with commands to execute line-by-line")
                .num_args(1),
        )
        .get_matches();

    let mut state = LoxState {
        interpreter: Interpreter::new(),
        stop_flag: Arc::new(AtomicBool::new(false)),
    };

    if let Some(file_path) = matches.get_one::<String>("file") {
        // If a file path is provided, read and process each line from the file
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
        exec_lines(&lines, &mut state);
    } else if atty::is(Stream::Stdin) {
        // If stdin is a terminal and no file is provided, start the REPL
        repl::start(&mut exec_line, &mut state)?;
    } else {
        // If input is piped in, read lines from stdin
        let stdin = io::stdin();
        let lines: Vec<String> = stdin.lock().lines().collect::<Result<_, _>>()?;
        exec_lines(&lines, &mut state);
    }

    Ok(())
}
