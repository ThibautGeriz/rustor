extern crate termion;

use std::cmp;
use std::io::Error;
use std::io::{stdin, stdout, Write};
use std::process::exit;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, style};

#[derive(Debug)]
struct CursorPosition {
    x: u16,
    y: u16,
}

fn render_line_nb(left_pad: &u16, line_nb: &u16) -> String {
    let nb_of_blanks_before_line_nb = left_pad - get_number_of_chars_of_u16(&line_nb);
    let mut line_nb_displayed = String::new();
    for _ in 0..nb_of_blanks_before_line_nb {
        line_nb_displayed.push(' ')
    }
    line_nb_displayed.push_str(&line_nb.to_string());
    return line_nb_displayed;
}

fn print_line(
    stream: &mut termion::raw::RawTerminal<std::io::Stdout>,
    left_pad: u16,
    line_nb: u16,
    content: &str,
    cursor: &CursorPosition,
) {
    let line_nb_displayed = render_line_nb(&left_pad, &line_nb);
    write!(
        stream,
        "{}{}{}.{} {}{}",
        termion::cursor::Goto(1, line_nb + 1),
        color::Fg(color::Blue),
        line_nb_displayed,
        style::Reset,
        content,
        termion::cursor::Goto(left_pad + cursor.x + 2, cursor.y + 1),
    )
    .unwrap();
}

fn print_first_line(stream: &mut termion::raw::RawTerminal<std::io::Stdout>) {
    write!(
        stream,
        "{}{}{}{}Rustor{}: ESC to quit{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        color::Fg(color::Red),
        style::Bold,
        style::Reset,
        termion::cursor::Goto(1, 2)
    )
    .unwrap();
}

fn get_number_of_chars_of_u16(num: &u16) -> u16 {
    let base = String::from(num.to_string());
    return base.len() as u16;
}

fn handle_key_press(key: Result<Key, Error>, lines: &mut Vec<String>, cursor: &mut CursorPosition) {
    let nb_lines = lines.len() as u16;
    let current_line = &mut lines[(cursor.y as usize) - 1];
    let nb_char_in_current_line = current_line.len() as u16;

    match key.unwrap() {
        Key::Char('\n') => {
            lines.push(String::new());
            cursor.y = cursor.y + 1;
            cursor.x = 1;
        }
        Key::Char(c) => {
            cursor.x = cursor.x + 1;
            current_line.push(c)
        }
        Key::Backspace => {
            if cursor.x != 1 {
                cursor.x = cursor.x - 1;
                current_line.truncate(current_line.len() - 1)
            } else if cursor.y > 1 {
                let nb_char_in_previous_line = lines[(cursor.y as usize) - 2].len() as u16;
                cursor.y = cursor.y - 1;
                cursor.x = nb_char_in_previous_line + 1;
                lines.remove((nb_lines as usize) - 1);
            }
        }
        Key::Left => {
            cursor.x = cmp::max(2, cursor.x) - 1;
        }
        Key::Right => {
            cursor.x = cmp::min(cursor.x + 1, nb_char_in_current_line + 1);
        }
        Key::Up => {
            if cursor.y != 1 {
                let nb_char_in_previous_line = lines[(cursor.y as usize) - 2].len() as u16;
                cursor.x = cmp::min(cursor.x, nb_char_in_previous_line + 1);
            }
            cursor.y = cmp::max(2, cursor.y) - 1;
        }
        Key::Down => {
            if cursor.y != nb_lines {
                let nb_char_in_next_line = lines[(cursor.y as usize)].len() as u16;
                cursor.x = cmp::min(cursor.x, nb_char_in_next_line + 1);
            }
            cursor.y = cmp::min(nb_lines, cursor.y + 1);
        }
        Key::Esc => exit(1),
        _ => (),
    }
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut lines: Vec<String> = vec![String::new()];

    let mut cursor = CursorPosition { x: 1, y: 1 };

    print_first_line(&mut stdout);

    print_line(&mut stdout, 1, 1, "", &cursor);

    stdout.flush().unwrap();

    for c in stdin.keys() {
        print_first_line(&mut stdout);
        handle_key_press(c, &mut lines, &mut cursor);

        let left_pad = get_number_of_chars_of_u16(&(lines.len() as u16));
        for (index, l) in lines.iter().enumerate() {
            print_line(&mut stdout, left_pad, index as u16 + 1, &l, &cursor)
        }
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
