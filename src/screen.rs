use termion::{clear, cursor};
use std::{io::{stdout, Write, stdin}, default};

pub fn clear_screen() {
    let mut stdout = stdout();
    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();
}

pub fn read_line(prompt: &'static str, defaultValue: &'static str) -> String {
    let mut buffer = String::new();
    let stdin = stdin();

    let result = stdin.read_line(&mut buffer);

    match result {
        Ok(_) => return trimmed(buffer),
        Err(_) => return defaultValue.to_string(),
    }
}

fn trimmed(value: String) -> String {
    let value = value.trim();
    return value.to_string();
}