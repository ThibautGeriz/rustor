extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{stdin, stdout, Stdout, Write};
use termion::{color, style};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut text = String::new();

    write!(
        stdout,
        "{}{}{}{}Rustor{}: ESC to quit{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        color::Fg(color::Red),
        style::Bold,
        style::Reset,
        termion::cursor::Hide
    ).unwrap();

    stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(
            stdout,
            "{}{}{}{}Rustor{}: ESC to quit{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            color::Fg(color::Red),
            style::Bold,
            style::Reset,
            termion::cursor::Hide
        ).unwrap();

        match c.unwrap() {
            Key::Char(c) => text.push(c),
            Key::Backspace => text.truncate(text.len() - 1),
            Key::Esc => break,
            _ => (),
        }
        let lines: Vec<&str> = text.split('\n').collect();
        for (index, l) in lines.iter().enumerate() {
            println!(
                "{}{}{}.{} {}",
                termion::cursor::Goto(1, index as u16 + 2),
                color::Fg(color::Blue),
                index + 1,
                style::Reset,
                l
            );
        }
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
