extern crate termion;

use std::env;
use std::fs::File;
use std::io::Error;
use std::io::{self, BufRead};
use std::io::{stdin, stdout, Write};
use std::path::Path;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::*;

use cursor::*;
use window::*;

mod cursor;
mod window;

fn backspace_remove_characters_in_line(
    y_position_in_file: usize,
    cursor: &mut CursorPosition,
    lines: &mut Vec<String>,
) {
    let current_line = &mut lines[y_position_in_file - 1];
    cursor.move_left();
    current_line.remove(cursor.x as usize - 1);
}

fn backspace_remove_line_break_when_not_on_first_line(
    y_position_in_file: usize,
    cursor: &mut CursorPosition,
    lines: &mut Vec<String>,
) {
    let cursor_x = cursor.x;
    cursor.move_up(&lines);
    cursor.move_to_end_of_line(&lines);
    if cursor_x == 1 {
        move_content_from_current_line_to_previous_line(y_position_in_file, lines);
    }
    lines.remove(y_position_in_file - 1);
}

fn move_content_from_current_line_to_previous_line(
    y_position_in_file: usize,
    lines: &mut Vec<String>,
) {
    let current_line = lines[y_position_in_file - 1].clone();
    let previous_line = &mut lines[y_position_in_file - 2];
    previous_line.push_str(&current_line);
}

fn handle_key_press(
    key: Result<Key, Error>,
    lines: &mut Vec<String>,
    cursor: &mut CursorPosition,
) -> bool {
    let (_, terminal_height) = termion::terminal_size().unwrap();
    let y_position_in_file = cursor.get_y_position_in_file() as usize;

    match key.unwrap() {
        Key::Char('\n') => {
            let current_line = lines[y_position_in_file - 1].clone();
            let nb_char_in_current_line = current_line.len() as u16;
            let end_of_line =
                &current_line[cursor.x as usize - 1..nb_char_in_current_line as usize];
            lines.insert(y_position_in_file, String::from(end_of_line));
            let current_line = &mut lines[y_position_in_file - 1];
            current_line.truncate(cursor.x as usize - 1);
            if cursor.y == terminal_height - 1 {
                cursor.x = 1;
                cursor.y_offset = cursor.y_offset + 1;
            } else {
                cursor.x = 1;
                cursor.y = cursor.y + 1;
            }
        }
        Key::Char(c) => {
            let current_line = &mut lines[y_position_in_file - 1];
            current_line.insert(cursor.x as usize - 1, c);
            cursor.x = cursor.x + 1;
        }
        Key::Backspace => {
            if cursor.x != 1 {
                backspace_remove_characters_in_line(y_position_in_file, cursor, lines);
            } else if cursor.y > 1 {
                backspace_remove_line_break_when_not_on_first_line(
                    y_position_in_file,
                    cursor,
                    lines,
                );
                if cursor.y_offset > 0 && lines.len() as u16 - cursor.y_offset < terminal_height {
                    cursor.y_offset = cursor.y_offset - 1;
                }
            } else if cursor.y_offset > 0 && cursor.y == 1 {
                cursor.move_to_end_of_line(&lines);
                lines.remove(y_position_in_file - 1);
                cursor.move_up(&lines);
            }
        }
        Key::Left => {
            cursor.move_left();
        }
        Key::Right => {
            cursor.move_right(lines);
        }
        Key::Up => {
            cursor.move_up(lines);
        }
        Key::Down => {
            cursor.move_down(lines, terminal_height);
        }
        Key::Esc => {
            return false;
        }
        _ => (),
    }
    return true;
}

fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}

fn init_lines(args: &Vec<String>) -> Vec<String> {
    let mut lines: Vec<String> = vec![];
    if args.len() > 1 {
        let file_name = &args[1];
        if let Ok(lines_in_file) = read_lines(file_name) {
            for line_in_file in lines_in_file {
                if let Ok(line) = line_in_file {
                    lines.push(line);
                }
            }
        }
    } else {
        lines.push(String::new());
    }
    return lines;
}

fn check_arguments(args: &Vec<String>) {
    if args.len() > 2 {
        panic!("Too many arguments")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    check_arguments(&args);

    let stdin = stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    let mut cursor = CursorPosition {
        x: 1,
        y: 1,
        y_offset: 0,
    };

    print_first_line(&mut stdout);

    let mut lines = init_lines(&args);

    print_text(&mut stdout, &lines, &cursor);
    stdout.flush().unwrap();

    for c in stdin.keys() {
        print_first_line(&mut stdout);
        let should_continue = handle_key_press(c, &mut lines, &mut cursor);
        if !should_continue {
            break;
        }
        print_text(&mut stdout, &lines, &cursor);
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_key_press_first_char() {
        // Given
        let key = Ok(Key::Char('t'));
        let mut lines: Vec<String> = vec![String::new()];
        let mut cursor = CursorPosition {
            x: 1,
            y: 1,
            y_offset: 0,
        };

        // When
        handle_key_press(key, &mut lines, &mut cursor);

        // Then
        assert_eq!(cursor.x, 2);
        assert_eq!(cursor.y, 1);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], "t");
    }
}
