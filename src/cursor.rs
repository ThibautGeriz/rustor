use std::cmp;

#[derive(Debug, Clone)]
pub struct CursorPosition {
    pub x: u16,
    pub y: u16,
    pub y_offset: u16,
}

impl CursorPosition {
    pub fn new() -> CursorPosition {
        CursorPosition {
            x: 1,
            y: 1,
            y_offset: 0,
        }
    }
    pub fn move_left(&mut self) {
        self.x = cmp::max(1, self.x - 1);
    }

    pub fn move_right(&mut self, lines: &[String]) {
        let y_position_in_file = self.get_y_position_in_file() as usize;
        let nb_char_in_current_line = lines[y_position_in_file - 1].len() as u16;
        self.x = cmp::min(self.x + 1, nb_char_in_current_line + 1);
    }

    pub fn get_y_position_in_file(&self) -> u16 {
        self.y + self.y_offset
    }

    pub fn move_up(&mut self, lines: &[String]) {
        let y_position_in_file = self.get_y_position_in_file();
        if y_position_in_file > 1 {
            let nb_char_in_previous_line = lines[y_position_in_file as usize - 2].len() as u16;
            self.x = cmp::min(self.x, nb_char_in_previous_line + 1);
        }
        if self.y == 1 && self.y_offset >= 1 {
            self.y_offset -= 1;
        } else {
            self.y = cmp::max(2, self.y) - 1;
        }
    }

    pub fn move_down(&mut self, lines: &[String], terminal_height: u16) {
        let y_position_in_file = self.get_y_position_in_file() as usize;
        if y_position_in_file == lines.len() {
            return;
        }
        if self.y != terminal_height - 1 {
            let nb_char_in_next_line = lines[y_position_in_file].len() as u16;
            self.x = cmp::min(self.x, nb_char_in_next_line + 1);
        }
        if self.y == terminal_height - 1 {
            self.y_offset += 1;
        } else {
            self.y = cmp::min(terminal_height - 1, self.y + 1);
        }
    }

    #[allow(dead_code)]
    pub fn move_to_end_of_line(&mut self, lines: &[String]) {
        let y_position_in_file = self.get_y_position_in_file() as usize;
        let number_of_char_in_line = lines[y_position_in_file - 1].len() as u16;
        self.x = number_of_char_in_line + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_move_left() {
        // Given
        let mut cursor = CursorPosition {
            x: 3,
            y: 4,
            y_offset: 0,
        };

        // When
        cursor.move_left();

        // Then
        assert_eq!(cursor.x, 2);
        assert_eq!(cursor.y, 4);
        assert_eq!(cursor.y_offset, 0);
    }

    #[test]
    fn should_not_move_left() {
        // Given
        let mut cursor = CursorPosition {
            x: 1,
            y: 4,
            y_offset: 0,
        };

        // When
        cursor.move_left();

        // Then
        assert_eq!(cursor.x, 1);
        assert_eq!(cursor.y, 4);
        assert_eq!(cursor.y_offset, 0);
    }

    #[test]
    fn should_move_right() {
        // Given
        let mut cursor = CursorPosition {
            x: 4,
            y: 1,
            y_offset: 0,
        };
        let lines: Vec<String> = vec![
            String::from("first line"),
            String::from("second line"),
            String::from("third line"),
        ];

        // When
        cursor.move_right(&lines);

        // Then
        assert_eq!(cursor.x, 5);
        assert_eq!(cursor.y, 1);
        assert_eq!(cursor.y_offset, 0);
    }

    #[test]
    fn should_not_move_right() {
        // Given
        let mut cursor = CursorPosition {
            x: 11,
            y: 1,
            y_offset: 0,
        };
        let lines: Vec<String> = vec![
            String::from("first line"),
            String::from("second line"),
            String::from("third line"),
        ];

        // When
        cursor.move_right(&lines);

        // Then
        assert_eq!(cursor.x, 11);
        assert_eq!(cursor.y, 1);
        assert_eq!(cursor.y_offset, 0);
    }

    #[test]
    fn should_get_y_position_in_file() {
        // Given
        let cursor = CursorPosition {
            x: 10,
            y: 4,
            y_offset: 2,
        };

        // When
        let result = cursor.get_y_position_in_file();

        // Then
        assert_eq!(result, 6);
    }

    #[test]
    fn should_not_move_up_when_on_the_first_line_of_the_file_and_terminal() {
        // Given
        let mut cursor = CursorPosition {
            x: 10,
            y: 1,
            y_offset: 0,
        };
        let lines: Vec<String> = vec![String::from("first line")];

        // When
        cursor.move_up(&lines);

        // Then
        assert_eq!(cursor.x, 10);
        assert_eq!(cursor.y, 1);
        assert_eq!(cursor.y_offset, 0);
    }

    #[test]
    fn should_move_up_only_the_offset() {
        // Given
        let mut cursor = CursorPosition {
            x: 10,
            y: 1,
            y_offset: 1,
        };
        let lines: Vec<String> = vec![String::from("first"), String::from("we are here")];

        // When
        cursor.move_up(&lines);

        // Then
        assert_eq!(cursor.x, 6);
        assert_eq!(cursor.y, 1);
        assert_eq!(cursor.y_offset, 0);
    }

    #[test]
    fn should_move_up() {
        // Given
        let mut cursor = CursorPosition {
            x: 10,
            y: 2,
            y_offset: 0,
        };
        let lines: Vec<String> = vec![String::from("first"), String::from("we are here")];

        // When
        cursor.move_up(&lines);

        // Then
        assert_eq!(cursor.x, 6);
        assert_eq!(cursor.y, 1);
        assert_eq!(cursor.y_offset, 0);
    }

    #[test]
    fn should_not_move_down_when_on_the_last_line_of_the_file() {
        // Given
        let mut cursor = CursorPosition {
            x: 7,
            y: 3,
            y_offset: 0,
        };
        let lines: Vec<String> = vec![
            String::from("first line"),
            String::from("second line"),
            String::from("third line"),
        ];
        let terminal_size: u16 = 4;

        // When
        cursor.move_down(&lines, terminal_size);

        // Then
        assert_eq!(cursor.x, 7);
        assert_eq!(cursor.y, 3);
        assert_eq!(cursor.y_offset, 0);
    }

    #[test]
    fn should_move_only_the_offset() {
        // Given
        let mut cursor = CursorPosition {
            x: 7,
            y: 4,
            y_offset: 0,
        };
        let lines: Vec<String> = vec![
            String::from("first line"),
            String::from("second line"),
            String::from("third line"),
            String::from("fourth line"),
            String::from("fifth line"),
        ];
        let terminal_size: u16 = 5;

        // When
        cursor.move_down(&lines, terminal_size);

        // Then
        assert_eq!(cursor.x, 7);
        assert_eq!(cursor.y, 4);
        assert_eq!(cursor.y_offset, 1);
    }

    #[test]
    fn should_move_down() {
        // Given
        let mut cursor = CursorPosition {
            x: 7,
            y: 2,
            y_offset: 0,
        };
        let lines: Vec<String> = vec![
            String::from("first line"),
            String::from("second line"),
            String::from("third line"),
            String::from("fourth line"),
            String::from("fifth line"),
        ];
        let terminal_size: u16 = 5;

        // When
        cursor.move_down(&lines, terminal_size);

        // Then
        assert_eq!(cursor.x, 7);
        assert_eq!(cursor.y, 3);
        assert_eq!(cursor.y_offset, 0);
    }

    #[test]
    fn should_move_to_end_file() {
        // Given
        let mut cursor = CursorPosition {
            x: 4,
            y: 3,
            y_offset: 0,
        };
        let lines: Vec<String> = vec![
            String::from("first"),
            String::from("second"),
            String::from("third line"),
            String::from("fourth"),
            String::from("fifth"),
        ];

        // When
        cursor.move_to_end_of_line(&lines);

        // Then
        assert_eq!(cursor.x, 11);
        assert_eq!(cursor.y, 3);
        assert_eq!(cursor.y_offset, 0);
    }
}
