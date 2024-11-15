use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::interpreter::HasStopFlag;

const PROMPT: &str = "\r\n> ";

fn handle_enter<TCallback, TState>(
    input_buffer: &mut String,
    cursor_position: &mut usize,
    callback: &mut TCallback,
    state: &mut TState,
) where
    TCallback: FnMut(&str, &mut TState),
    TState: HasStopFlag,
{
    callback(&input_buffer, state);
    input_buffer.clear();
    *cursor_position = 0;

    if !state.is_stopped() {
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

fn process_key_event<TCallback, TState>(
    key_event: KeyEvent,
    input_buffer: &mut String,
    cursor_position: &mut usize,
    callback: &mut TCallback,
    state: &mut TState,
) where
    TCallback: FnMut(&str, &mut TState),
    TState: HasStopFlag,
{
    if key_event.kind != crossterm::event::KeyEventKind::Press {
        return;
    }

    match key_event {
        KeyEvent {
            code: KeyCode::Enter,
            modifiers: _,
            kind: _,
            state: _,
        } => handle_enter(input_buffer, cursor_position, callback, state),

        KeyEvent {
            code: KeyCode::Backspace,
            modifiers: _,
            kind: _,
            state: _,
        } => handle_backspace(input_buffer, cursor_position),

        KeyEvent {
            code: KeyCode::Delete,
            modifiers: _,
            kind: _,
            state: _,
        } => handle_delete(input_buffer, cursor_position),

        KeyEvent {
            code: KeyCode::Left,
            modifiers: _,
            kind: _,
            state: _,
        } => handle_cursor_left(cursor_position),

        KeyEvent {
            code: KeyCode::Right,
            modifiers: _,
            kind: _,
            state: _,
        } => handle_cursor_right(input_buffer, cursor_position),

        KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::CONTROL,
            kind: _,
            state: _,
        } if c == 'c' || c == 'd' || c == 'z' => state.trigger_stop(),

        KeyEvent {
            code: KeyCode::Char(c),
            modifiers: _,
            kind: _,
            state: _,
        } => handle_character(input_buffer, cursor_position, c),

        _ => {}
    }
}

pub fn start<TCallback, TState>(
    callback: &mut TCallback,
    state: &mut TState,
) -> Result<(), Box<dyn std::error::Error>>
where
    TCallback: FnMut(&str, &mut TState),
    TState: HasStopFlag,
{
    print!("Welcome to your REPL! Type 'Ctrl+Z' to quit.\r\n");

    // Enable raw mode for the terminal
    // let stdin = io::stdin();
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    let mut input_buffer = String::new();
    let mut cursor_position = 0;

    print!("{}", PROMPT);
    stdout.flush()?;

    loop {
        if let Event::Key(key_event) = read().unwrap() {
            // println!("key_event: {:?}", key_event);

            process_key_event(
                key_event,
                &mut input_buffer,
                &mut cursor_position,
                callback,
                state,
            );
            stdout.flush()?;

            if state.is_stopped() {
                break;
            }
        }
    }

    stdout.flush()?;
    print!("\r\nGoodbye!\r\n");
    disable_raw_mode()?;

    Ok(())
}
