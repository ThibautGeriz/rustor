extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{stdin, stdout, Write};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut line = String::new();

    write!(
        stdout,
        "{}{}Rustor: ESC to quit{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    ).unwrap();
    stdout.flush().unwrap();

    let mut position = 1;

    for c in stdin.keys() {
        write!(
            stdout,
            "{}{}Rustor: ESC to quit{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide
        ).unwrap();

        match c.unwrap() {
            Key::Char(c) => line.push(c),
            Key::Backspace => line.truncate(line.len() - 1),
            Key::Esc => break,
            _ => {}
        }
        println!("{}{}", termion::cursor::Goto(1, 2), line);
        stdout.flush().unwrap();
        position = position + 1;
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
