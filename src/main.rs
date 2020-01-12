extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{stdin, stdout, Write};
use termion::{color, style};

#[derive(Debug)]
struct CursorPosition {
    x: u16,
    y: u16,
}

fn get_number_of_chars_of_u16(num: usize) -> u16 {
    let base = String::from(num.to_string());
    return base.len() as u16;
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut text = String::new();

    let mut cursor = CursorPosition { x: 1, y: 1 };

    write!(
        stdout,
        "{}{}{}{}Rustor{}: ESC to quit{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        color::Fg(color::Red),
        style::Bold,
        style::Reset,
        termion::cursor::Goto(1, 2)
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
            termion::cursor::Goto(1, 2)
        ).unwrap();

        match c.unwrap() {
            Key::Char('\n') => {
                cursor.y = cursor.y + 1;
                cursor.x = 1;
                text.push('\n')
            }
            Key::Char(c) => {
                cursor.x = cursor.x + 1;
                text.push(c)
            }
            Key::Backspace => {
                cursor.x = cursor.x - 1;
                text.truncate(text.len() - 1)
            }
            Key::Esc => break,
            _ => (),
        }
        let lines: Vec<&str> = text.split('\n').collect();
        let leftPad = get_number_of_chars_of_u16(lines.len()) as u16;
        for (index, l) in lines.iter().enumerate() {
            println!(
                "{}{}{}.{} {}{}",
                termion::cursor::Goto(1, index as u16 + 2),
                color::Fg(color::Blue),
                index + 1,
                style::Reset,
                l,
                termion::cursor::Goto(leftPad + cursor.x + 2, cursor.y),
            );
        }
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
