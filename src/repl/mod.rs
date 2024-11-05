use std::io::{self, Write};
use signal_hook::consts::signal::SIGTSTP;
use signal_hook::consts::SIGINT;
use signal_hook::flag;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::event::Key;

pub type StopFlag = Arc<AtomicBool>;

const PROMPT: &str = "\r\n> ";

// Handle signal registration in its own function.
fn setup_signal_handler() -> io::Result<StopFlag> {
    let stop_flag = Arc::new(AtomicBool::new(false));
    flag::register(SIGTSTP, Arc::clone(&stop_flag)).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    flag::register(SIGINT, Arc::clone(&stop_flag)).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(stop_flag)
}

fn handle_enter<TCallback, TState>(input_buffer: &mut String, cursor_position: &mut usize, stop_flag: &StopFlag, callback: &mut TCallback, state: &mut TState) where TCallback: FnMut(&str, &mut TState, &StopFlag) {
    callback(&input_buffer, state, stop_flag);
    input_buffer.clear();
    *cursor_position = 0;

    if !stop_flag.load(Ordering::Relaxed) {
        print!("{}", PROMPT);
    }
}

fn handle_backspace(input_buffer: &mut String, cursor_position: &mut usize) {
    if *cursor_position <= 0 {
        return;
    }

    input_buffer.remove(*cursor_position - 1);
    (*cursor_position) -= 1;
    print!("\x08"); // Move back one space
    // Clear the line and reprint the buffer from the cursor position
    print!("{} ", &input_buffer[*cursor_position..]);
    // Move the cursor back to the current position
    for _ in *cursor_position..input_buffer.len() {
        print!("\x08");
    }
    print!("\x08");
}

fn handle_delete(input_buffer: &mut String, cursor_position: &mut usize) {
    if *cursor_position >= input_buffer.len() {
        return;
    }

    // Remove character at cursor position
    input_buffer.remove(*cursor_position);
    // Clear the line from cursor position and reprint the buffer
    print!("{} ", &input_buffer[*cursor_position..]);
    // Move the cursor back to the original position
    for _ in *cursor_position..input_buffer.len() {
        print!("\x08");
    }
    print!("\x08");
}

fn handle_cursor_left(cursor_position: &mut usize) {
    if *cursor_position <= 0 {
        return;
    }

    *cursor_position -= 1;
    print!("\x08");
}

fn handle_cursor_right(input_buffer: &mut String, cursor_position: &mut usize) {
    if *cursor_position >= input_buffer.len() {
        return;
    }

    print!("{}", &input_buffer[*cursor_position..*cursor_position + 1]);
    *cursor_position += 1;
}

fn handle_character(input_buffer: &mut String, cursor_position: &mut usize, c: char) {
    input_buffer.insert(*cursor_position, c);
    *cursor_position += 1;
    print!("{}", &input_buffer[*cursor_position - 1..]);
    // Move the cursor back to the current position
    for _ in *cursor_position..input_buffer.len() {
        print!("\x08");
    }
}

fn process_key_event<TCallback, TState>(key: Key, input_buffer: &mut String, cursor_position: &mut usize, stop_flag: &StopFlag, callback: &mut TCallback, state: &mut TState) where TCallback: FnMut(&str, &mut TState, &StopFlag) {
    match key {
        Key::Char('\n') => handle_enter(input_buffer, cursor_position, stop_flag, callback, state),
        Key::Backspace => handle_backspace(input_buffer, cursor_position),
        Key::Delete => handle_delete(input_buffer, cursor_position),
        Key::Left => handle_cursor_left(cursor_position),
        Key::Right => handle_cursor_right(input_buffer, cursor_position),
        Key::Char(c) => handle_character(input_buffer, cursor_position, c),
        Key::Ctrl('z') | Key::Ctrl('c') => stop_flag.store(true, Ordering::Relaxed),
        _ => {},
    }
}

pub fn start<TCallback, TState>(callback: &mut TCallback, state: &mut TState) -> Result<(), Box<dyn std::error::Error>> where TCallback: FnMut(&str, &mut TState, &StopFlag) {
    print!("Welcome to your REPL! Type 'Ctrl+Z' to quit.\r\n");

    let stop_flag = setup_signal_handler()?;

    // Enable raw mode for the terminal
    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode()?;

    let mut input_buffer = String::new();
    let mut cursor_position = 0;

    print!("{}", PROMPT);
    stdout.flush()?;

    for c in stdin.keys() {
        match c {
            Ok(key) => process_key_event(key, &mut input_buffer, &mut cursor_position, &stop_flag, callback, state),
            Err(e) => eprintln!("Error reading input: {:?}", e),
        }
        stdout.flush()?;

        // Check for Ctrl+Z signal flag and handle it.
        if stop_flag.load(Ordering::Relaxed) {
            break;
        }
    }

    stdout.flush()?;
    print!("\r\nGoodbye!\r\n");

    Ok(())
}
