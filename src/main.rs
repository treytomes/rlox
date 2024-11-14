mod app_info;
mod interpreter;
mod lexer;
mod parser;
mod repl;

use app_info::AppInfo;
use atty::Stream;
use clap::{Arg, Command};
use interpreter::{Interpreter, Object};
use lexer::scan_tokens;
use parser::{parse, AstPrinter, Expr};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// TODO: Implement comma-separated expression parsing.
// \-- Only return the result of the right-most expression to the user for that sequence.
//      \-- Unless this is a function argument list.
// TODO: Implement bitwise and/or operators.
// \-- Replace ! / && / || with not/and/or.  Use boolean operations with true/false.  With numbers, flatten to integer and use bitwise ops.
// TODO: Implement the ternary operator.
// \-- I expect this will be above the precedence of equality.

const REPORT_COUNT: bool = false;
const REPORT_TOKENS: bool = false;
const REPORT_AST: bool = false;

// Define a struct to represent the REPL state
struct LoxState {
    command_count: usize,
    had_error: bool,
}

fn print_tokens(tokens: &Vec<lexer::Token>) {
    print!("Tokens:\r\n");
    for token in tokens {
        print!("\t{:?}\r\n", token);
    }
}

fn parse_line(input: &str, state: &mut LoxState) -> Result<Expr, anyhow::Error> {
    state.command_count += 1;
    if REPORT_COUNT {
        print!(
            "\r\ncallback: you entered '{}', command count: {}\r\n",
            input, state.command_count
        );
    }

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
                    eprint!("\r\nerror: {}\r\n", err);

                    // Take the 3rd line out the input text.
                    let lines: Vec<&str> = input.split('\n').collect();
                    let line = lines[err.line - 1];

                    // Convert line to a string and get the length of it.
                    let len = err.line.to_string().len();

                    eprint!("\r\n");
                    eprint!("{} | {}\r\n", err.line, line);
                    eprint!("{:>width$}-- Here.\r\n", "^", width = err.column + len + 3);

                    state.had_error = true;
                    return Err(anyhow::Error::new(err).context("parsing error"));
                }
            }
        }
        Err(err) => {
            eprint!("Error: {}\r\n", err);

            // Take the 3rd line out the input text.
            let lines: Vec<&str> = input.split('\n').collect();
            let line = lines[err.line - 1];

            // Convert line to a string and get the length of it.
            let len = err.line.to_string().len();

            eprint!("\r\n");
            eprint!("{} | {}\r\n", err.line, line);
            eprint!("{:>width$}-- Here.\r\n", "^", width = err.column + len + 3);

            state.had_error = true;
            return Err(anyhow::Error::new(err).context("lexing error"));
        }
    }

    // TODO: In REPL mode, this should get executed and the had_error flag reset.
}

fn exec_line(input: &str, state: &mut LoxState, _stop_flag: &repl::StopFlag) {
    // TODO: Maybe move the stop flag into the state?
    let expr = parse_line(input, state);
    match expr {
        Ok(expr) => {
            if !state.had_error {
                // TODO: had_error will always be flipped if the result is an error?
                let result = Interpreter::new().eval(&expr);
                match result {
                    Ok(value) => {
                        // print!("result: {}\r\n", value);
                        print!("{}\r\n", value);
                    }
                    Err(err) => {
                        eprint!("runtime error: {}\r\n", err);
                    }
                }
            }
        }
        Err(_err) => {
            // The lexing/parsing errors were reported in the previous stages.
            // eprint!("error: {}\r\n", err);
            return;
        }
    }

    state.had_error = false;
}

fn parse_lines(lines: Vec<String>, state: &mut LoxState) -> Result<Vec<Expr>, anyhow::Error> {
    let mut exprs: Vec<Expr> = vec![];
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        exprs.push(parse_line(&line, state)?);
    }
    Ok(exprs)
}

fn exec_lines(lines: Vec<String>, state: &mut LoxState) {
    let stop_flag = Arc::new(AtomicBool::new(false));
    match parse_lines(lines, state) {
        Ok(exprs) => {
            if !state.had_error {
                let mut interpreter = Interpreter::new();

                let mut final_result = Object::Nil;
                for expr in exprs {
                    let result = interpreter.eval(&expr);
                    match result {
                        Ok(value) => {
                            final_result = value;
                        }
                        Err(err) => {
                            state.had_error = true;
                            eprint!("runtime error: {}\r\n", err);
                            break;
                        }
                    }
                    if stop_flag.load(Ordering::Relaxed) {
                        break;
                    }
                }
                if !state.had_error {
                    // print!("result: {}\r\n", final_result);
                    print!("{}\r\n", final_result);
                }
            }
        }
        Err(err) => {
            eprint!("error: {}\r\n", err);
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
        command_count: 0,
        had_error: false,
    };

    if let Some(file_path) = matches.get_one::<String>("file") {
        // If a file path is provided, read and process each line from the file
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
        exec_lines(lines, &mut state);
    } else if atty::is(Stream::Stdin) {
        // If stdin is a terminal and no file is provided, start the REPL
        repl::start(&mut exec_line, &mut state)?;
    } else {
        // If input is piped in, read lines from stdin
        let stdin = io::stdin();
        let lines: Vec<String> = stdin.lock().lines().collect::<Result<_, _>>()?;
        exec_lines(lines, &mut state);
    }

    Ok(())
}
