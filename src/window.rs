extern crate termion;

use std::cmp;
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
) {
    let line_nb_displayed = render_line_nb(left_pad, file_line_nb);
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
    let (terminal_width, terminal_height) = termion::terminal_size().unwrap();
    let left_pad = get_number_of_chars_of_u16(editor.lines.len() as u16);
    let max_line = cmp::min(
        editor.lines.len(),
        terminal_height as usize - 1 + editor.cursor.y_offset as usize,
    );
    let white_line = (0..terminal_width).map(|_| ' ').collect::<String>();
    for (index, l) in editor.lines[editor.cursor.y_offset as usize..max_line]
        .iter()
        .enumerate()
    {
        let mut line_content = l.clone();
        line_content.push_str(&white_line);
        line_content.truncate((terminal_width - left_pad - 2) as usize);
        print_line(
            stream,
            left_pad,
            index as u16 + 1,
            index as u16 + 1 + editor.cursor.y_offset,
            &line_content,
            &editor.cursor,
        )
    }

    if editor.lines.len() < terminal_height as usize - 1 {
        write!(
            stream,
            "{}{}{}",
            termion::cursor::Goto(1, editor.lines.len() as u16 + 2),
            white_line,
            termion::cursor::Goto(left_pad + editor.cursor.x + 2, editor.cursor.y + 1),
        )
        .unwrap();
    }
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
}
