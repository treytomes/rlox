mod repl;

use atty::Stream;
use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// Define a struct to represent the REPL state
struct ReplState {
    command_count: usize,
}

fn parse_line(input: &str, state: &mut ReplState, stop_flag: &repl::StopFlag) {
    state.command_count += 1;
    println!("\r\nCallback: You entered '{}', command count: {}", input, state.command_count);

    // Manually handle newline and echo the input buffer
    if input.trim().eq_ignore_ascii_case("exit") {
        // Set stop_flag to true if "exit" is entered.
        stop_flag.store(true, Ordering::Relaxed);
        return;
    }
}

fn parse_lines(lines: Vec<String>, state: &mut ReplState) {
    let stop_flag = Arc::new(AtomicBool::new(false));
    for line in lines {
        parse_line(&line, state, &stop_flag);
        if stop_flag.load(Ordering::Relaxed) {
            break;
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let version = env!("CARGO_PKG_VERSION");
    let description = env!("CARGO_PKG_DESCRIPTION");

    let matches = Command::new("Lox Language REPL")
        .version(version)
        .about(description)
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .help("Path to a file with commands to execute line-by-line")
                .num_args(1),
        )
        .get_matches();
    
    let mut state = ReplState { command_count: 0 };

    if let Some(file_path) = matches.get_one::<String>("file") {
        // If a file path is provided, read and process each line from the file
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
        parse_lines(lines, &mut state);
    } else if atty::is(Stream::Stdin) {
        // If stdin is a terminal and no file is provided, start the REPL
        repl::start(&mut parse_line, &mut state)?;
    } else {
        // If input is piped in, read lines from stdin
        let stdin = io::stdin();
        let lines: Vec<String> = stdin.lock().lines().collect::<Result<_, _>>()?;
        parse_lines(lines, &mut state);
    }

    Ok(())
}
