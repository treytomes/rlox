mod app_info;
mod repl;
mod lexer;

use app_info::AppInfo;
use atty::Stream;
use clap::{Arg, Command};
use lexer::scan_tokens;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// Define a struct to represent the REPL state
struct LoxState {
    command_count: usize,
    had_error: bool,
}

fn print_tokens(tokens: Vec<lexer::Token>) {
    print!("Tokens:\r\n");
    for token in tokens {
        print!("\t{:?}\r\n", token);
    }
}

fn parse_line(input: &str, state: &mut LoxState, stop_flag: &repl::StopFlag) {
    state.command_count += 1;
    print!("\r\nCallback: You entered '{}', command count: {}\r\n", input, state.command_count);

    // Manually handle newline and echo the input buffer
    if input.trim().eq_ignore_ascii_case("exit") {
        // Set stop_flag to true if "exit" is entered.
        stop_flag.store(true, Ordering::Relaxed);
        return;
    }

    let tokens = scan_tokens(input);
    match tokens {
        Ok(tokens) => print_tokens(tokens),
        Err(err) => {
            eprint!("Error scanning tokens: {}\r\n", err);
            state.had_error = true;
        },
    }

    // TODO: In REPL mode, this should get executed and the had_error flag reset.
}

fn exec_line(input: &str, state: &mut LoxState, stop_flag: &repl::StopFlag) {
    parse_line(input, state, stop_flag);
    
    if !state.had_error {
        // TODO: Execute the line here.
    }
    state.had_error = false;
}

fn parse_lines(lines: Vec<String>, state: &mut LoxState) {
    let stop_flag = Arc::new(AtomicBool::new(false));
    for line in lines {
        parse_line(&line, state, &stop_flag);
        if stop_flag.load(Ordering::Relaxed) {
            break;
        }
    }
}

fn exec_lines(lines: Vec<String>, state: &mut LoxState) {
    parse_lines(lines, state);
    if (!state.had_error) {
        // TODO: Now execute it.
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
    
    let mut state = LoxState { command_count: 0, had_error: false };

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
