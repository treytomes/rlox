mod app_info;
mod debug;
mod interpreter;
mod lexer;
mod parser;
mod repl;

use app_info::AppInfo;
use atty::Stream;
use clap::{Arg, Command};
use debug::{AstPrinter, LocatableError};
use interpreter::{HasStopFlag, Interpreter};
use lexer::scan_tokens;
use parser::{parse, Expr};
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        let mut reader = BufReader::new(file);
        let mut input = String::new();
        reader.read_to_string(&mut input)?;
        exec_line(&input, &mut state);
    } else if atty::is(Stream::Stdin) {
        // If stdin is a terminal and no file is provided, start the REPL
        repl::start(&mut exec_line, &mut state)?;
    } else {
        // If input is piped in, read lines from stdin
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.lock().read_to_string(&mut input)?;
        exec_line(&input, &mut state);
    }

    Ok(())
}
