extern crate termion;

use std::cmp;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::{stdin, stdout, Write};
use std::io::Error;
use std::path::Path;
use std::process::exit;

use termion::{color, style};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::ops::Add;

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
    terminal_line_nb: u16,
    file_line_nb: u16,
    content: &str,
    cursor: &CursorPosition,
) {
    let line_nb_displayed = render_line_nb(&left_pad, &file_line_nb);
    write!(
        stream,
        "{}{}{}.{} {}{}",
        termion::cursor::Goto(1, terminal_line_nb + 1),
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

fn handle_key_press(key: Result<Key, Error>,
                    lines: &mut Vec<String>,
                    cursor: &mut CursorPosition,
                    terminal_height_offset: usize) -> usize {
    let nb_lines = lines.len() as u16;
    let (_, terminal_height) = termion::terminal_size().unwrap();


    match key.unwrap() {
        Key::Char('\n') => {
            let current_line = lines[(cursor.y as usize) - 1].clone();
            let nb_char_in_current_line = current_line.len() as u16;
            let end_of_line =
                &current_line[cursor.x as usize - 1..nb_char_in_current_line as usize];
            lines.insert(cursor.y as usize, String::from(end_of_line));
            let current_line = &mut lines[(cursor.y as usize) - 1];
            current_line.truncate(cursor.x as usize - 1);
            if cursor.y == terminal_height - 1 {
                cursor.x = 1;
                return terminal_height_offset + 1;
            } 
            cursor.x = 1;
            cursor.y = cursor.y + 1;
            
        }
        Key::Char(c) => {
            let current_line = &mut lines[(cursor.y as usize) - 1];
            current_line.insert(cursor.x as usize - 1, c);
            cursor.x = cursor.x + 1;
        }
        Key::Backspace => {
            if cursor.x != 1 {
                let current_line = &mut lines[(cursor.y as usize) - 1];
                cursor.x = cursor.x - 1;
                current_line.remove(cursor.x as usize - 1);
            } else if cursor.y > 1 && cursor.x == 1 {
                let current_line = lines[(cursor.y as usize) - 1].clone();
                let nb_char_in_previous_line = lines[(cursor.y as usize) - 2].len() as u16;
                let previous_line = &mut lines[(cursor.y as usize) - 2];
                previous_line.push_str(&current_line);
                cursor.y = cursor.y - 1;
                cursor.x = nb_char_in_previous_line + 1;
                lines.remove(cursor.y as usize);
            } else if cursor.y > 1 {
                let nb_char_in_previous_line = lines[(cursor.y as usize) - 2].len() as u16;
                cursor.y = cursor.y - 1;
                cursor.x = nb_char_in_previous_line + 1;
                lines.remove(cursor.y as usize);
            }
        }
        Key::Left => {
            cursor.x = cmp::max(2, cursor.x) - 1;
        }
        Key::Right => {
            let current_line = &mut lines[(cursor.y as usize) - 1];
            let nb_char_in_current_line = current_line.len() as u16;
            cursor.x = cmp::min(cursor.x + 1, nb_char_in_current_line + 1);
        }
        Key::Up => {
            if cursor.y != 1 {
                let nb_char_in_previous_line = lines[(cursor.y as usize) - 2].len() as u16;
                cursor.x = cmp::min(cursor.x, nb_char_in_previous_line + 1);
            }
            if cursor.y == 1 && terminal_height_offset >= 1{
                return terminal_height_offset - 1;
            }
            cursor.y = cmp::max(2, cursor.y) - 1;
        }
        Key::Down => {
            if cursor.y as usize + terminal_height_offset == lines.len() {
                return terminal_height_offset;
            }
            if cursor.y != terminal_height - 1 {
                let nb_char_in_next_line = lines[(cursor.y as usize)].len() as u16;
                cursor.x = cmp::min(cursor.x, nb_char_in_next_line + 1);
            }
            if cursor.y == terminal_height - 1 {
                return terminal_height_offset + 1;
            }
            cursor.y = cmp::min(terminal_height - 1, cursor.y + 1);
        }
        Key::Esc => exit(1),
        _ => (),
    }
    return terminal_height_offset;
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

fn print_text(
    stream: &mut termion::raw::RawTerminal<std::io::Stdout>,
    lines: &Vec<String>,
    cursor: &CursorPosition,
    terminal_height_offset: usize,
) {
    let (terminal_width, terminal_height) = termion::terminal_size().unwrap();
    let left_pad = get_number_of_chars_of_u16(&(lines.len() as u16));
    let max_line = cmp::min(lines.len(), terminal_height as usize - 1 + terminal_height_offset);

    for (index, l) in lines[terminal_height_offset..max_line].iter().enumerate() {
        if l.len() as u16 > terminal_width - left_pad - 2 {
            let mut line_content = l.clone();
            line_content.truncate((terminal_width - left_pad - 2) as usize);
            print_line(stream, left_pad, index as u16 + 1, index as u16 + 1 + terminal_height_offset as u16, &line_content, &cursor)
        } else {
            print_line(stream, left_pad, index as u16 + 1, index as u16 + 1 + terminal_height_offset as u16, &l, &cursor)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    check_arguments(&args);

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut terminal_height_offset: usize = 0;
    let mut cursor = CursorPosition { x: 1, y: 1 };

    print_first_line(&mut stdout);

    let mut lines = init_lines(&args);

    print_text(&mut stdout, &lines, &cursor, terminal_height_offset);
    stdout.flush().unwrap();

    for c in stdin.keys() {
        print_first_line(&mut stdout);
        terminal_height_offset = handle_key_press(c, &mut lines, &mut cursor, terminal_height_offset);
        print_text(&mut stdout, &lines, &cursor, terminal_height_offset);
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_number_of_chars_of_u16_one_digit() {
        // Given
        let nb = 4 as u16;

        // When
        let result = get_number_of_chars_of_u16(&nb);

        // Then
        assert_eq!(result, 1);
    }

    #[test]
    fn test_get_number_of_chars_of_u16_two_digits() {
        // Given
        let nb = 99 as u16;

        // When
        let result = get_number_of_chars_of_u16(&nb);

        // Then
        assert_eq!(result, 2);
    }

    #[test]
    fn test_get_number_of_chars_of_u16_three_digits() {
        // Given
        let nb = 666 as u16;

        // When
        let result = get_number_of_chars_of_u16(&nb);

        // Then
        assert_eq!(result, 3);
    }

    #[test]
    fn test_render_line_nb_56_out_of_3() {
        // Given
        let padding = 3;
        let line_number = 56;

        // When
        let result = render_line_nb(&padding, &line_number);

        // Then
        assert_eq!(result, " 56");
    }

    #[test]
    fn test_render_line_nb_57_out_of_2() {
        // Given
        let padding = 2;
        let line_number = 57;

        // When
        let result = render_line_nb(&padding, &line_number);

        // Then
        assert_eq!(result, "57");
    }

    #[test]
    fn test_render_line_nb_4_out_of_5() {
        // Given
        let padding = 4;
        let line_number = 5;

        // When
        let result = render_line_nb(&padding, &line_number);

        // Then
        assert_eq!(result, "   5");
    }

    #[test]
    fn test_handle_key_press_first_char() {
        // Given
        let key = Ok(Key::Char('t'));
        let mut lines: Vec<String> = vec![String::new()];
        let mut cursor = CursorPosition { x: 1, y: 1 };
        let mut terminal_height_offset: usize = 0;

        // When
        handle_key_press(key, &mut lines, &mut cursor, terminal_height_offset);

        // Then
        assert_eq!(cursor.x, 2);
        assert_eq!(cursor.y, 1);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], "t");
    }
}
