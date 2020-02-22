use std::io::Error;

use termion::event::Key;

use cursor::*;
use file::*;

#[derive(Debug)]
pub struct Content {
    pub lines: Vec<String>,
}

impl Content {
    pub fn new() -> Content {
        return Content {
            lines: vec![String::new()]
        };
    }

    pub fn insert(&mut self, cursor: &CursorPosition, c: char) {
        if c == '\n' {
            self.insert_new_line(cursor);
        } else {
            self.insert_char(cursor, c);
        }
    }

    fn insert_new_line(&mut self, cursor: &CursorPosition) {
        let y_position_in_file = cursor.get_y_position_in_file() as usize;
        let current_line = self.lines[y_position_in_file - 1].clone();
        let nb_char_in_current_line = current_line.len() as u16;
        let end_of_line = &current_line[cursor.x as usize - 1..nb_char_in_current_line as usize];
        self.lines.insert(y_position_in_file, String::from(end_of_line));
        let current_line = &mut self.lines[y_position_in_file - 1];
        current_line.truncate(cursor.x as usize - 1);
    }

    fn insert_char(&mut self, cursor: &CursorPosition, c: char) {
        let current_line = &mut self.lines[cursor.get_y_position_in_file() as usize - 1];
        current_line.insert(cursor.x as usize - 1, c);
    }

    pub fn remove(&mut self, cursor: &CursorPosition) {
        let y_position_in_file = cursor.get_y_position_in_file() as usize;
        if cursor.x > 1 {
            let current_line = &mut self.lines[y_position_in_file - 1];
            current_line.remove(cursor.x as usize - 2);
        } else if y_position_in_file > 1 {
            let current_line = self.lines[y_position_in_file - 1].clone();
            let previous_line = &mut self.lines[y_position_in_file - 2];
            previous_line.push_str(&current_line);
            self.lines.remove(y_position_in_file - 1);
        }
    }
}


pub fn handle_key_press(
    key: Result<Key, Error>,
    content: &mut Content,
    cursor: &mut CursorPosition,
    file_name_option: Option<&String>,
    terminal_height: u16,
) -> bool {
    let y_position_in_file = cursor.get_y_position_in_file() as usize;

    match key.unwrap() {
        Key::Char('\n') => {
            content.insert(&cursor, '\n');
            if cursor.y == terminal_height - 1 {
                cursor.x = 1;
                cursor.y_offset = cursor.y_offset + 1;
            } else {
                cursor.x = 1;
                cursor.y = cursor.y + 1;
            }
        }
        Key::Char(c) => {
            content.insert(&cursor, c);
            cursor.x = cursor.x + 1;
        }
        Key::Backspace => {
            if cursor.x != 1 {
                content.remove(&cursor);
                cursor.move_left();
            } else if y_position_in_file > 1 {
                let cursor_before = cursor.clone();
                cursor.move_up(&content.lines);
                cursor.move_to_end_of_line(&content.lines);
                content.remove(& cursor_before);
                if cursor.y_offset > 0 && content.lines.len() as u16 - cursor.y_offset < terminal_height {
                    cursor.y_offset = cursor.y_offset - 1;
                    cursor.y = cursor_before.y;
                }
            }
        }
        Key::Left => {
            cursor.move_left();
        }
        Key::Right => {
            cursor.move_right(&mut content.lines);
        }
        Key::Up => {
            cursor.move_up(&mut content.lines);
        }
        Key::Ctrl('s') => {
            if let Some(file_name) = file_name_option {
                save_to_file(file_name, &mut content.lines).unwrap();
            }
        }
        Key::Down => {
            cursor.move_down(&mut content.lines, terminal_height);
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
        let mut cursor = CursorPosition {
            x: 1,
            y: 1,
            y_offset: 0,
        };
        let mut content = Content::new();


        // When
        handle_key_press(key,  &mut content, &mut cursor, file_name_option, terminal_height);

        // Then
        assert_eq!(cursor.x, 2);
        assert_eq!(cursor.y, 1);
        assert_eq!(content.lines.len(), 1);
        assert_eq!(content.lines[0], "t");
    }


    #[test]
    fn insert_char_should_insert_first_char() {
        // Given
        let cursor = CursorPosition::new();
        let mut content = Content::new();


        // When
        content.insert(&cursor, 'x');

        // Then
        assert_eq!(content.lines, vec!["x"]);
    }

    #[test]
    fn insert_char_should_insert_in_the_middle_of_the_line() {
        // Given
        let cursor = CursorPosition {
            x: 9,
            y: 1,
            y_offset: 0,
        };
        let mut content = Content {
            lines: vec![String::from("this is  test")]
        };


        // When
        content.insert(&cursor, 'a');

        // Then
        assert_eq!(content.lines, vec!["this is a test"]);
    }

    #[test]
    fn insert_char_should_insert_a_new_line_at_the_end() {
        // Given
        let cursor = CursorPosition {
            x: 15,
            y: 1,
            y_offset: 0,
        };
        let mut content = Content {
            lines: vec![String::from("this is a test")]
        };


        // When
        content.insert(&cursor, '\n');

        // Then
        assert_eq!(content.lines, vec!["this is a test", ""]);
    }

    #[test]
    fn insert_char_should_insert_a_new_line_in_the_middle_of_line() {
        // Given
        let cursor = CursorPosition {
            x: 10,
            y: 1,
            y_offset: 0,
        };
        let mut content = Content {
            lines: vec![String::from("this is a test")]
        };


        // When
        content.insert(&cursor, '\n');

        // Then
        assert_eq!(content.lines, vec!["this is a", " test"]);
    }

    #[test]
    fn remove_should_remove_char() {
        // Given
        let cursor = CursorPosition {
            x: 11,
            y: 1,
            y_offset: 0,
        };
        let mut content = Content {
            lines: vec![String::from("this is aw test")]
        };


        // When
        content.remove(&cursor);

        // Then
        assert_eq!(content.lines, vec!["this is a test"]);
    }

    #[test]
    fn remove_should_remove_empty_line() {
        // Given
        let cursor = CursorPosition {
            x: 1,
            y: 2,
            y_offset: 0,
        };
        let mut content = Content {
            lines: vec![String::from("this is a test"), String::new(), String::from("this is a test2")]
        };


        // When
        content.remove(&cursor);

        // Then
        assert_eq!(content.lines, vec!["this is a test", "this is a test2"]);
    }

    #[test]
    fn remove_should_remove_not_empty_line() {
        // Given
        let cursor = CursorPosition {
            x: 1,
            y: 2,
            y_offset: 0,
        };
        let mut content = Content {
            lines: vec![String::from("this is a test"), String::from("this is a test2"), String::from("this is a test3")]
        };


        // When
        content.remove(&cursor);

        // Then
        assert_eq!(content.lines, vec!["this is a testthis is a test2", "this is a test3"]);
    }

    #[test]
    fn remove_should_not_remove() {
        // Given
        let cursor = CursorPosition {
            x: 1,
            y: 1,
            y_offset: 0,
        };
        let mut content = Content {
            lines: vec![String::from("this is a test"), String::from("this is a test2")]
        };


        // When
        content.remove(&cursor);

        // Then
        assert_eq!(content.lines, vec!["this is a test", "this is a test2"]);
    }

    #[test]
    fn remove_should_remove_char_with_offset() {
        // Given
        let cursor = CursorPosition {
            x: 5,
            y: 3,
            y_offset: 1,
        };
        let mut content = Content {
            lines: vec![String::from("this is a test"), String::from("this is a test2"),
                        String::from("this is a test3"),
                        String::from("this is a test4")]
        };


        // When
        content.remove(&cursor);

        // Then
        assert_eq!(content.lines, vec!["this is a test", "this is a test2",
                                       "this is a test3", "thi is a test4"]);
    }

    #[test]
    fn remove_should_remove_char_with_offset_on_first_line() {
        // Given
        let cursor = CursorPosition {
            x: 5,
            y: 1,
            y_offset: 1,
        };
        let mut content = Content {
            lines: vec![String::from("this is a test"), String::from("this is a test2"),
                        String::from("this is a test3"),
                        String::from("this is a test4")]
        };


        // When
        content.remove(&cursor);

        // Then
        assert_eq!(content.lines, vec!["this is a test", "thi is a test2",
                                       "this is a test3", "this is a test4"]);
    }
}
