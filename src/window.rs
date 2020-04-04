extern crate termion;

use std::io::Write;

use termion::{color, style};

use cursor::*;
use editor::Editor;

pub fn print_line<W: Write>(
    stream: &mut W,
    left_pad: u16,
    terminal_line_nb: u16,
    file_line_nb: u16,
    content: &str,
    cursor: &CursorPosition,
    y_prev_line_offset: u16,
) -> (u16, u16) {
    let (terminal_width, _) = termion::terminal_size().unwrap();
    let line_nb_displayed = render_line_nb(left_pad, file_line_nb);
    let (x_current_line, y_current_line) =
        if content.len() + left_pad as usize > terminal_width as usize {
            let rest = content.len() as u16 + left_pad - terminal_width;
            (
                cursor.y + y_prev_line_offset + rest / terminal_width + 1,
                rest % terminal_width + 3,
            )
        } else {
            (cursor.x + left_pad + 2, cursor.y + y_prev_line_offset)
        };
    // required to pad the rest of the line with whitespace
    let whitespaces = (x_current_line - 1..terminal_width)
        .map(|_| ' ')
        .collect::<String>();

    write!(
        stream,
        "{}{}{}.{} {}{}{}",
        termion::cursor::Goto(1, terminal_line_nb + y_prev_line_offset),
        color::Fg(color::Blue),
        line_nb_displayed,
        style::Reset,
        content,
        whitespaces,
        termion::cursor::Goto(x_current_line, y_current_line),
    )
    .unwrap();
    (x_current_line, y_current_line)
}

pub fn print_first_line<W: Write>(stream: &mut W) {
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

pub fn get_number_of_chars_of_u16(num: u16) -> u16 {
    let base = num.to_string();
    base.len() as u16
}

pub fn print_text<W: Write>(stream: &mut W, editor: &Editor) {
    let number_of_lines = editor.get_number_of_lines();
    let (terminal_width, terminal_height) = termion::terminal_size().unwrap();
    let left_pad = get_number_of_chars_of_u16(number_of_lines as u16);
    let editor_lines = editor.get_editor_lines(terminal_height as usize - 1);
    let mut y_line_offset = 1;
    let mut x_current_line = 1;
    let mut y_current_line = 1;
    for (index, l) in editor_lines.iter().enumerate() {
        let actual_position = print_line(
            stream,
            left_pad,
            index as u16 + 1,
            index as u16 + 1 + editor.cursor.y_offset,
            &l,
            &editor.cursor,
            y_line_offset,
        );
        x_current_line = actual_position.0;
        y_current_line = actual_position.1;

        //        if l.len() + left_pad as usize > terminal_width as usize {
        //            y_line_offset += (l.len() as u16 + left_pad - terminal_width) / terminal_width + 1
        //        }
        y_line_offset = get_number_of_lines_under_which_the_text_line_should_be_displayed(
            &l,
            left_pad,
            terminal_width,
        );
    }
    let white_line = (0..terminal_width).map(|_| ' ').collect::<String>();

    let number_of_lines = editor.get_number_of_lines();
    if number_of_lines as u16 + y_line_offset < terminal_height - 1 {
        write!(
            stream,
            "{}{}{}",
            termion::cursor::Goto(1, number_of_lines as u16 + y_line_offset + 1),
            white_line,
            termion::cursor::Goto(x_current_line, y_current_line),
        )
        .unwrap();
    }
}

fn get_number_of_lines_under_which_the_text_line_should_be_displayed(
    line: &str,
    left_pad: u16,
    terminal_width: u16,
) -> u16 {
    if line.len() as u16 + left_pad < terminal_width {
        return 1;
    }
    (line.len() as u16 + left_pad - terminal_width) / terminal_width + 2
}

fn render_line_nb(left_pad: u16, line_nb: u16) -> String {
    let nb_of_blanks_before_line_nb = left_pad - get_number_of_chars_of_u16(line_nb);
    let mut line_nb_displayed = String::new();
    for _ in 0..nb_of_blanks_before_line_nb {
        line_nb_displayed.push(' ')
    }
    line_nb_displayed.push_str(&line_nb.to_string());
    line_nb_displayed
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
        let result = get_number_of_chars_of_u16(nb);

        // Then
        assert_eq!(result, 1);
    }

    #[test]
    fn test_get_number_of_chars_of_u16_two_digits() {
        // Given
        let nb = 99 as u16;

        // When
        let result = get_number_of_chars_of_u16(nb);

        // Then
        assert_eq!(result, 2);
    }

    #[test]
    fn test_get_number_of_chars_of_u16_three_digits() {
        // Given
        let nb = 666 as u16;

        // When
        let result = get_number_of_chars_of_u16(nb);

        // Then
        assert_eq!(result, 3);
    }

    #[test]
    fn test_render_line_nb_56_out_of_3() {
        // Given
        let padding = 3;
        let line_number = 56;

        // When
        let result = render_line_nb(padding, line_number);

        // Then
        assert_eq!(result, " 56");
    }

    #[test]
    fn test_render_line_nb_57_out_of_2() {
        // Given
        let padding = 2;
        let line_number = 57;

        // When
        let result = render_line_nb(padding, line_number);

        // Then
        assert_eq!(result, "57");
    }

    #[test]
    fn test_render_line_nb_4_out_of_5() {
        // Given
        let padding = 4;
        let line_number = 5;

        // When
        let result = render_line_nb(padding, line_number);

        // Then
        assert_eq!(result, "   5");
    }

    #[test]
    fn should_be_on_1_line_when_length_less_than_terminal_width() {
        // Given
        let line = String::from("this is a test");
        let left_pad = 3;
        let terminal_width = 30;

        // When
        let result = get_number_of_lines_under_which_the_text_line_should_be_displayed(
            &line,
            left_pad,
            terminal_width,
        );

        // Then
        assert_eq!(result, 1);
    }

    #[test]
    fn should_be_on_2_lines_when_length_is_more_than_terminal_width() {
        // Given
        let line = String::from("this is a longer line test.");
        let left_pad = 3;
        let terminal_width = 20;

        // When
        let result = get_number_of_lines_under_which_the_text_line_should_be_displayed(
            &line,
            left_pad,
            terminal_width,
        );

        // Then
        assert_eq!(result, 2);
    }

    #[test]
    fn should_be_on_3_lines_when_length_is_much_more_than_terminal_width() {
        // Given
        let line = String::from("this is a longer line test. More. More. More");
        let left_pad = 3;
        let terminal_width = 20;

        // When
        let result = get_number_of_lines_under_which_the_text_line_should_be_displayed(
            &line,
            left_pad,
            terminal_width,
        );

        // Then
        assert_eq!(result, 3);
    }

    #[test]
    fn should_be_on_2_lines_with_left_pad() {
        // Given
        let line = String::from("123456");
        let left_pad = 5;
        let terminal_width = 10;

        // When
        let result = get_number_of_lines_under_which_the_text_line_should_be_displayed(
            &line,
            left_pad,
            terminal_width,
        );

        // Then
        assert_eq!(result, 2);
    }
}
