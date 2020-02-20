extern crate termion;

use std::env;
use std::io::{stdin, stdout, Write};

use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::*;

use content::*;
use cursor::*;
use file::*;
use window::*;

mod content;
mod cursor;
mod file;
mod window;

fn main() {
    let args: Vec<String> = env::args().collect();
    check_arguments(&args);
    let file_name_option = get_file_name(&args);

    let stdin = stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    let mut cursor = CursorPosition {
        x: 1,
        y: 1,
        y_offset: 0,
    };

    print_first_line(&mut stdout);

    let mut lines = init_lines(&file_name_option);

    print_text(&mut stdout, &lines, &cursor);
    stdout.flush().unwrap();

    for c in stdin.keys() {
        let (_, terminal_height) = termion::terminal_size().unwrap();
        let should_continue = handle_key_press(c, &mut lines, &mut cursor, file_name_option, terminal_height);
        if !should_continue {
            break;
        }
        print_text(&mut stdout, &lines, &cursor);
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

fn check_arguments(args: &Vec<String>) {
    if args.len() > 2 {
        panic!("Too many arguments")
    }
}

fn get_file_name(args: &Vec<String>) -> Option<&String> {
    if args.len() == 1 {
        return None;
    }
    return Some(&args[1]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_file_name_should_return_nothing_if_no_args() {
        // Given
        let args = vec![String::from("rustor")];

        // When
        let result = get_file_name(&args);

        // Then
        assert_eq!(None, result);
    }

    #[test]
    fn get_file_name_should_return_the_name_of_the_file() {
        // Given
        let args = vec![String::from("rustor"), String::from("stuff.txt")];

        // When
        let result = get_file_name(&args);

        // Then
        assert_eq!(Some(&String::from("stuff.txt")), result);
    }
}
