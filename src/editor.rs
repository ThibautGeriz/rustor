use std::cmp;
use std::io::Error;

use termion::event::Key;

use cursor::*;
use file::*;
use piece_table::PieceTable;

#[derive(Debug)]
pub struct Editor {
    piece_table: PieceTable,
    pub cursor: CursorPosition,
}

impl Editor {
    pub fn from(lines: Vec<String>) -> Editor {
        Editor {
            piece_table: PieceTable::new(lines.join("\n")),
            cursor: CursorPosition::new(),
        }
    }

    pub fn get_editor_lines(&self, terminal_height: usize) -> Vec<String> {
        let y_offset = (self.cursor.y_offset) as usize;
        let number_of_lines = self.get_number_of_lines();
        let max_line: usize =
            Editor::compute_max_line_of_editor(number_of_lines, terminal_height, y_offset);
        self.get_range_lines(y_offset, max_line)
    }

    fn compute_max_line_of_editor(
        number_of_lines: usize,
        terminal_height: usize,
        y_offset: usize,
    ) -> usize {
        cmp::min(number_of_lines, terminal_height + y_offset)
    }

    fn get_range_lines(&self, start: usize, stop: usize) -> Vec<String> {
        self.piece_table.get_range_lines(start, stop)
    }

    pub fn get_all_lines(&self) -> Vec<String> {
        self.piece_table.get_all_lines()
    }

    pub fn get_number_of_lines(&self) -> usize {
        self.piece_table.get_number_of_lines()
    }

    pub fn insert(&mut self, c: char, terminal_height: u16) {
        self.insert_in_piece_table(c, terminal_height);
    }

    fn insert_in_piece_table(&mut self, c: char, terminal_height: u16) {
        self.piece_table
            .insert(self.get_cursor_position_in_file(), c.to_string());
        if c == '\n' && self.cursor.y == terminal_height - 1 {
            self.cursor.x = 1;
            self.cursor.y_offset += 1;
        } else if c == '\n' {
            self.cursor.x = 1;
            self.cursor.y += 1;
        } else {
            self.cursor.x += 1;
        }
    }

    pub fn remove(&mut self, terminal_height: u16) {
        let y_position_in_file = self.cursor.get_y_position_in_file() as usize;
        let start_index = self.get_cursor_position_in_file();
        if self.cursor.x > 1 {
            self.piece_table.remove(start_index as u32 - 1, 1);
            self.cursor.x -= 1;
        } else if y_position_in_file > 1 {
            let lines = self.get_all_lines();
            self.piece_table.remove(start_index as u32 - 1, 1);
            self.cursor.y -= 1;
            self.cursor.x = (lines[y_position_in_file - 2].len()) as u16 + 1;
            if self.cursor.y_offset > 0
                && self.get_number_of_lines() as u16 - self.cursor.y_offset < terminal_height
            {
                self.cursor.y_offset -= 1;
                self.cursor.y += 1;
            }
        }
    }
    fn get_cursor_position_in_file(&self) -> u32 {
        let length = self.cursor.get_y_position_in_file() as usize;
        let mut lines = self.get_range_lines(0, length);
        let last_line: String = lines
            .last_mut()
            .unwrap()
            .chars()
            .take(self.cursor.x as usize - 1)
            .collect();
        lines[length - 1] = last_line;
        lines.join("\n").len() as u32
    }
}

pub fn handle_key_press(
    key: Result<Key, Error>,
    editor: &mut Editor,
    file_name_option: Option<&String>,
    terminal_height: u16,
) -> bool {
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
            editor.cursor.move_right(&editor.get_all_lines());
        }
        Key::Up => {
            editor.cursor.move_up(&editor.get_all_lines());
        }
        Key::Ctrl('s') => {
            if let Some(file_name) = file_name_option {
                save_to_file(file_name, editor.piece_table.get_text()).unwrap();
            }
        }
        Key::Down => {
            editor
                .cursor
                .move_down(&editor.get_all_lines(), terminal_height);
        }
        Key::Esc => {
            return false;
        }
        _ => (),
    }
    true
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
        let mut editor = Editor::from(vec![String::new()]);

        // When
        handle_key_press(key, &mut editor, file_name_option, terminal_height);

        // Then
        assert_eq!(editor.cursor.x, 2);
        assert_eq!(editor.cursor.y, 1);
        assert_eq!(editor.get_all_lines().len(), 1);
        assert_eq!(editor.get_editor_lines(20), vec!["t"]);
        assert_eq!(editor.get_number_of_lines(), 1);
    }

    #[test]
    fn insert_char_should_insert_first_char_with_piece_table() {
        // Given
        let mut editor = Editor::from(vec![String::new()]);

        // When
        editor.insert_in_piece_table('x', 36);

        // Then

        assert_eq!(editor.piece_table.get_text(), "x");
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
        let lines = vec![String::from("this is  test")];
        let piece_table = PieceTable::new(lines.clone().join("\n"));
        let mut editor = Editor {
            piece_table,
            cursor,
        };

        // When
        editor.insert_in_piece_table('a', 36);

        // Then
        assert_eq!(editor.piece_table.get_text(), "this is a test");
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
        let lines = vec![String::from("this is a test")];
        let piece_table = PieceTable::new(lines.clone().join("\n"));
        let mut editor = Editor {
            piece_table,
            cursor,
        };

        // When
        editor.insert_in_piece_table('\n', 36);

        // Then
        assert_eq!(editor.piece_table.get_text(), "this is a test\n");
        assert_eq!(editor.cursor.x, 1);
        assert_eq!(editor.cursor.y, 2);
        assert_eq!(editor.cursor.y_offset, 0);
        assert_eq!(editor.get_number_of_lines(), 2);
        assert_eq!(editor.get_editor_lines(20), &vec!["this is a test", ""][..]);
        assert_eq!(editor.get_editor_lines(1), &vec!["this is a test"][..]);
    }

    #[test]
    fn insert_char_should_insert_a_new_line_in_the_middle_of_line() {
        // Given
        let cursor = CursorPosition {
            x: 10,
            y: 1,
            y_offset: 0,
        };
        let lines = vec![String::from("this is a test")];
        let piece_table = PieceTable::new(lines.clone().join("\n"));
        let mut editor = Editor {
            piece_table,
            cursor,
        };

        // When
        editor.insert('\n', 36);

        // Then
        // assert_eq!(editor. vec!["this is a", " test"]);
        assert_eq!(editor.get_editor_lines(20), vec!["this is a", " test"]);
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
        let lines = vec![String::from("this is aw test")];
        let piece_table = PieceTable::new(lines.clone().join("\n"));

        let mut editor = Editor {
            piece_table,
            cursor,
        };

        // When
        editor.remove(36);

        // Then
        assert_eq!(editor.get_editor_lines(36), vec!["this is a test"]);
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
        let lines = vec![
            String::from("this is a test"),
            String::new(),
            String::from("this is a test2"),
        ];
        let piece_table = PieceTable::new(lines.clone().join("\n"));

        let mut editor = Editor {
            piece_table,
            cursor,
        };

        // When
        editor.remove(36);

        // Then
        assert_eq!(editor.cursor.x, 15);
        assert_eq!(editor.cursor.y, 1);
        assert_eq!(editor.cursor.y_offset, 0);
        assert_eq!(
            editor.get_editor_lines(36),
            vec!["this is a test", "this is a test2"]
        );
    }

    #[test]
    fn remove_should_remove_not_empty_line() {
        // Given
        let cursor = CursorPosition {
            x: 1,
            y: 2,
            y_offset: 0,
        };
        let lines = vec![
            String::from("this is a test"),
            String::from("this is a test2"),
            String::from("this is a test3"),
        ];
        let piece_table = PieceTable::new(lines.clone().join("\n"));

        let mut editor = Editor {
            piece_table,
            cursor,
        };

        // When
        editor.remove(36);

        // Then
        assert_eq!(
            editor.get_editor_lines(36),
            vec!["this is a testthis is a test2", "this is a test3"]
        );
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
        let lines = vec![
            String::from("this is a test"),
            String::from("this is a test2"),
        ];
        let piece_table = PieceTable::new(lines.clone().join("\n"));

        let mut editor = Editor {
            piece_table,
            cursor,
        };

        // When
        editor.remove(36);

        // Then
        assert_eq!(
            editor.get_editor_lines(36),
            vec!["this is a test", "this is a test2"]
        );
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
        let lines = vec![
            String::from("this is a test"),
            String::from("this is a test2"),
            String::from("this is a test3"),
            String::from("this is a test4"),
        ];
        let piece_table = PieceTable::new(lines.clone().join("\n"));

        let mut editor = Editor {
            piece_table,
            cursor,
        };

        // When
        editor.remove(3);

        // Then
        assert_eq!(
            editor.get_range_lines(0, 4),
            vec![
                "this is a test",
                "this is a test2",
                "this is a test3",
                "thi is a test4"
            ]
        );
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

        let lines = vec![
            String::from("this is a test"),
            String::from("this is a test2"),
            String::from("this is a test3"),
            String::from("this is a test4"),
        ];
        let piece_table = PieceTable::new(lines.clone().join("\n"));

        let mut editor = Editor {
            piece_table,
            cursor,
        };

        // When
        editor.remove(36);

        // Then
        assert_eq!(
            editor.get_range_lines(0, 4),
            vec![
                "this is a test",
                "thi is a test2",
                "this is a test3",
                "this is a test4"
            ]
        );
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
        let lines = vec![
            String::from("this is a test"),
            String::from("this is a test2"),
            String::from("this is a test3"),
            String::from("this is a test4"),
        ];
        let piece_table = PieceTable::new(lines.clone().join("\n"));

        let mut editor = Editor {
            piece_table,
            cursor,
        };

        // When
        editor.remove(5);

        // Then
        assert_eq!(
            editor.get_range_lines(0, 4),
            vec![
                "this is a testthis is a test2",
                "this is a test3",
                "this is a test4"
            ]
        );
        assert_eq!(editor.cursor.x, 15);
        assert_eq!(editor.cursor.y, 1);
        assert_eq!(editor.cursor.y_offset, 0);
    }

    #[test]
    fn remove_should_remove_last_empty_line() {
        // Given
        let cursor = CursorPosition {
            x: 1,
            y: 4,
            y_offset: 0,
        };
        let lines = vec![
            String::from("this is a test"),
            String::from(""),
            String::from(""),
            String::from(""),
        ];
        let piece_table = PieceTable::new(lines.clone().join("\n"));

        let mut editor = Editor {
            piece_table,
            cursor,
        };

        // When
        editor.remove(5);

        // Then
        assert_eq!(editor.get_range_lines(0, 4), vec!["this is a test", "", ""]);
        assert_eq!(editor.cursor.x, 1);
        assert_eq!(editor.cursor.y, 3);
        assert_eq!(editor.cursor.y_offset, 0);
    }

    #[test]
    fn get_cursor_position_in_file_should_compute_nb_of_characters() {
        // Given
        let cursor = CursorPosition {
            x: 1,
            y: 2,
            y_offset: 0,
        };
        let lines = vec![
            String::from("this is a test"),
            String::from("this is a test2"),
            String::from("this is a test3"),
        ];
        let piece_table = PieceTable::new(lines.clone().join("\n"));

        let editor = Editor {
            piece_table,
            cursor,
        };

        // When
        let result = editor.get_cursor_position_in_file();

        // Then
        assert_eq!(result, 15);
    }

    #[test]
    fn get_cursor_position_in_file_should_compute_nb_of_characters_with_offset() {
        // Given
        let cursor = CursorPosition {
            x: 5,
            y: 3,
            y_offset: 1,
        };
        let lines = vec![
            String::from("this is a test"),
            String::from("this is a test2"),
            String::from("this is a test3"),
            String::from("this is a test4"),
        ];
        let piece_table = PieceTable::new(lines.clone().join("\n"));

        let editor = Editor {
            piece_table,
            cursor,
        };

        // When
        let result = editor.get_cursor_position_in_file();

        // Then
        assert_eq!(result, 51);
    }
}
