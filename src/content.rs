use std::io::{Cursor, Error};

use termion::event::Key;

use cursor::*;
use file::*;

#[derive(Debug)]
pub struct Editor {
    pub lines: Vec<String>,
    pub cursor: CursorPosition,
}

impl Editor {
    pub fn new() -> Editor {
        return Editor {
            lines: vec![String::new()],
            cursor: CursorPosition { x: 1, y: 1, y_offset: 0 },
        };
    }

    pub fn from(lines: Vec<String>) -> Editor {
        return Editor {
            lines,
            cursor: CursorPosition::new(),
        };
    }

    pub fn insert(&mut self, c: char, terminal_height: u16) {
        if c == '\n' {
            self.insert_new_line(terminal_height);
        } else {
            self.insert_char(c);
        }
    }

    fn insert_new_line(&mut self, terminal_height: u16) {
        let y_position_in_file = self.cursor.get_y_position_in_file() as usize;
        let current_line = self.lines[y_position_in_file - 1].clone();
        let nb_char_in_current_line = current_line.len() as u16;
        let end_of_line = &current_line[self.cursor.x as usize - 1..nb_char_in_current_line as usize];
        self.lines.insert(y_position_in_file, String::from(end_of_line));
        let current_line = &mut self.lines[y_position_in_file - 1];
        current_line.truncate(self.cursor.x as usize - 1);
        if self.cursor.y == terminal_height - 1 {
            self.cursor.x = 1;
            self.cursor.y_offset = self.cursor.y_offset + 1;
        } else {
            self.cursor.x = 1;
            self.cursor.y = self.cursor.y + 1;
        }
    }

    fn insert_char(&mut self, c: char) {
        let current_line = &mut self.lines[self.cursor.get_y_position_in_file() as usize - 1];
        current_line.insert(self.cursor.x as usize - 1, c);
        self.cursor.x = self.cursor.x + 1;
    }

    pub fn remove(&mut self, terminal_height: u16) {
        let y_position_in_file = self.cursor.get_y_position_in_file() as usize;
        if self.cursor.x > 1 {
            self.remove_char();
        } else if y_position_in_file > 1 {
            self.remove_line(terminal_height);
        }
    }

    fn remove_char(&mut self) {
        let y_position_in_file = self.cursor.get_y_position_in_file() as usize;
        let current_line = &mut self.lines[y_position_in_file - 1];
        current_line.remove(self.cursor.x as usize - 2);
        self.cursor.move_left();
    }

    fn remove_line(&mut self, terminal_height: u16) {
        let cursor_before = self.cursor.clone();
        let y_position_in_file = cursor_before.get_y_position_in_file() as usize;
        self.cursor.move_up(&self.lines);
        self.cursor.move_to_end_of_line(&self.lines);
        let current_line = self.lines[y_position_in_file - 1].clone();
        let previous_line = &mut self.lines[y_position_in_file - 2];
        previous_line.push_str(&current_line);
        self.lines.remove(y_position_in_file - 1);
        if self.cursor.y_offset > 0 && self.lines.len() as u16 - self.cursor.y_offset < terminal_height {
            self.cursor.y_offset = self.cursor.y_offset - 1;
            self.cursor.y = cursor_before.y;
        }
    }
}


pub fn handle_key_press(
    key: Result<Key, Error>,
    editor: &mut Editor,
    file_name_option: Option<&String>,
    terminal_height: u16,
) -> bool {
    let y_position_in_file = editor.cursor.get_y_position_in_file() as usize;

    match key.unwrap() {
        Key::Char(c) => {
            editor.insert(c, terminal_height);
        }
        Key::Backspace => {
            editor.remove(terminal_height);
        }
        Key::Left => {
            editor.cursor.move_left();
        }
        Key::Right => {
            editor.cursor.move_right(&editor.lines);
        }
        Key::Up => {
            editor.cursor.move_up(&editor.lines);
        }
        Key::Ctrl('s') => {
            if let Some(file_name) = file_name_option {
                save_to_file(file_name, &editor.lines).unwrap();
            }
        }
        Key::Down => {
            editor.cursor.move_down(&editor.lines, terminal_height);
        }
        Key::Esc => {
            return false;
        }
        _ => (),
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_key_press_first_char() {
        // Given
        let key: Result<Key, Error> = Ok(Key::Char('t'));
        let terminal_height: u16 = 50;
        let file_name = String::from("toto");
        let file_name_option = Some(&file_name);
        let mut editor = Editor::new();


        // When
        handle_key_press(key, &mut editor, file_name_option, terminal_height);

        // Then
        assert_eq!(editor.cursor.x, 2);
        assert_eq!(editor.cursor.y, 1);
        assert_eq!(editor.lines.len(), 1);
        assert_eq!(editor.lines[0], "t");
        assert_eq!(editor.lines[0], "t");
    }


    #[test]
    fn insert_char_should_insert_first_char() {
        // Given
        let cursor = CursorPosition::new();
        let mut editor = Editor::new();


        // When
        editor.insert('x', 36);

        // Then
        assert_eq!(editor.lines, vec!["x"]);
        assert_eq!(editor.cursor.x, 2);
        assert_eq!(editor.cursor.y, 1);
        assert_eq!(editor.cursor.y_offset, 0);
    }

    #[test]
    fn insert_char_should_insert_in_the_middle_of_the_line() {
        // Given
        let cursor = CursorPosition {
            x: 9,
            y: 1,
            y_offset: 0,
        };
        let mut editor = Editor {
            lines: vec![String::from("this is  test")],
            cursor,
        };


        // When
        editor.insert('a', 36);

        // Then
        assert_eq!(editor.lines, vec!["this is a test"]);
        assert_eq!(editor.cursor.x, 10);
        assert_eq!(editor.cursor.y, 1);
        assert_eq!(editor.cursor.y_offset, 0);
    }

    #[test]
    fn insert_char_should_insert_a_new_line_at_the_end() {
        // Given
        let cursor = CursorPosition {
            x: 15,
            y: 1,
            y_offset: 0,
        };
        let mut editor = Editor {
            lines: vec![String::from("this is a test")],
            cursor,
        };


        // When
        editor.insert('\n', 36);

        // Then
        assert_eq!(editor.lines, vec!["this is a test", ""]);
        assert_eq!(editor.cursor.x, 1);
        assert_eq!(editor.cursor.y, 2);
        assert_eq!(editor.cursor.y_offset, 0);
    }

    #[test]
    fn insert_char_should_insert_a_new_line_in_the_middle_of_line() {
        // Given
        let cursor = CursorPosition {
            x: 10,
            y: 1,
            y_offset: 0,
        };
        let mut editor = Editor {
            lines: vec![String::from("this is a test")],
            cursor,
        };


        // When
        editor.insert('\n', 36);

        // Then
        assert_eq!(editor.lines, vec!["this is a", " test"]);
        assert_eq!(editor.cursor.x, 1);
        assert_eq!(editor.cursor.y, 2);
        assert_eq!(editor.cursor.y_offset, 0);
    }

    #[test]
    fn remove_should_remove_char() {
        // Given
        let cursor = CursorPosition {
            x: 11,
            y: 1,
            y_offset: 0,
        };
        let mut editor = Editor {
            lines: vec![String::from("this is aw test")],
            cursor,
        };


        // When
        editor.remove(36);

        // Then
        assert_eq!(editor.lines, vec!["this is a test"]);
        assert_eq!(editor.cursor.x, 10);
        assert_eq!(editor.cursor.y, 1);
        assert_eq!(editor.cursor.y_offset, 0);
    }

    #[test]
    fn remove_should_remove_empty_line() {
        // Given
        let cursor = CursorPosition {
            x: 1,
            y: 2,
            y_offset: 0,
        };
        let mut editor = Editor {
            lines: vec![String::from("this is a test"), String::new(), String::from("this is a test2")],
            cursor,
        };


        // When
        editor.remove(36);

        // Then
        assert_eq!(editor.lines, vec!["this is a test", "this is a test2"]);
        assert_eq!(editor.cursor.x, 15);
        assert_eq!(editor.cursor.y, 1);
        assert_eq!(editor.cursor.y_offset, 0);
    }

    #[test]
    fn remove_should_remove_not_empty_line() {
        // Given
        let cursor = CursorPosition {
            x: 1,
            y: 2,
            y_offset: 0,
        };
        let mut editor = Editor {
            lines: vec![String::from("this is a test"), String::from("this is a test2"), String::from("this is a test3")],
            cursor,
        };


        // When
        editor.remove(36);

        // Then
        assert_eq!(editor.lines, vec!["this is a testthis is a test2", "this is a test3"]);
        assert_eq!(editor.cursor.x, 15);
        assert_eq!(editor.cursor.y, 1);
        assert_eq!(editor.cursor.y_offset, 0);
    }

    #[test]
    fn remove_should_not_remove() {
        // Given
        let cursor = CursorPosition {
            x: 1,
            y: 1,
            y_offset: 0,
        };
        let mut editor = Editor {
            lines: vec![String::from("this is a test"), String::from("this is a test2")],
            cursor,
        };


        // When
        editor.remove(36);

        // Then
        assert_eq!(editor.lines, vec!["this is a test", "this is a test2"]);
        assert_eq!(editor.cursor.x, 1);
        assert_eq!(editor.cursor.y, 1);
        assert_eq!(editor.cursor.y_offset, 0);
    }

    #[test]
    fn remove_should_remove_char_with_offset() {
        // Given
        let cursor = CursorPosition {
            x: 5,
            y: 3,
            y_offset: 1,
        };
        let mut editor = Editor {
            lines: vec![String::from("this is a test"), String::from("this is a test2"),
                        String::from("this is a test3"),
                        String::from("this is a test4")],
            cursor,
        };


        // When
        editor.remove(36);

        // Then
        assert_eq!(editor.lines, vec!["this is a test", "this is a test2",
                                      "this is a test3", "thi is a test4"]);
        assert_eq!(editor.cursor.x, 4);
        assert_eq!(editor.cursor.y, 3);
        assert_eq!(editor.cursor.y_offset, 1);
    }

    #[test]
    fn remove_should_remove_char_with_offset_on_first_line() {
        // Given
        let cursor = CursorPosition {
            x: 5,
            y: 1,
            y_offset: 1,
        };
        let mut editor = Editor {
            lines: vec![String::from("this is a test"), String::from("this is a test2"),
                        String::from("this is a test3"),
                        String::from("this is a test4")],
            cursor,
        };


        // When
        editor.remove(36);

        // Then
        assert_eq!(editor.lines, vec!["this is a test", "thi is a test2",
                                      "this is a test3", "this is a test4"]);
        assert_eq!(editor.cursor.x, 4);
        assert_eq!(editor.cursor.y, 1);
        assert_eq!(editor.cursor.y_offset, 1);
    }

    #[test]
    fn remove_should_remove_line_with_offset_reduced() {
        // Given
        let cursor = CursorPosition {
            x: 1,
            y: 1,
            y_offset: 1,
        };
        let mut editor = Editor {
            lines: vec![String::from("this is a test"), String::from("this is a test2"),
                        String::from("this is a test3"),
                        String::from("this is a test4")],
            cursor,
        };


        // When
        editor.remove(5);

        // Then
        assert_eq!(editor.lines, vec!["this is a testthis is a test2",
                                      "this is a test3", "this is a test4"]);
        assert_eq!(editor.cursor.x, 15);
        assert_eq!(editor.cursor.y, 1);
        assert_eq!(editor.cursor.y_offset, 0);
    }
}
