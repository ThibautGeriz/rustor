use std::io::Error;

use termion::event::Key;

use cursor::*;
use file::*;

pub fn handle_key_press(
  key: Result<Key, Error>,
  lines: &mut Vec<String>,
  cursor: &mut CursorPosition,
  file_name_option: Option<&String>,
  terminal_height: u16
) -> bool {
  let y_position_in_file = cursor.get_y_position_in_file() as usize;

  match key.unwrap() {
    Key::Char('\n') => {
      let current_line = lines[y_position_in_file - 1].clone();
      let nb_char_in_current_line = current_line.len() as u16;
      let end_of_line = &current_line[cursor.x as usize - 1..nb_char_in_current_line as usize];
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
        backspace_remove_line_break_when_not_on_first_line(y_position_in_file, cursor, lines);
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
    Key::Ctrl('s') => {
      if let Some(file_name) = file_name_option {
        save_to_file(file_name, lines).unwrap();
      }
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_handle_key_press_first_char() {
    // Given
    let key: Result<Key, Error> = Ok(Key::Char('t'));
    let terminal_height: u16 = 50;
    let mut lines: Vec<String> = vec![String::new()];
    let file_name = String::from("toto");
    let file_name_option = Some(&file_name);
    let mut cursor = CursorPosition {
      x: 1,
      y: 1,
      y_offset: 0
    };

    // When
    handle_key_press(key, &mut lines, &mut cursor, file_name_option, terminal_height);

    // Then
    assert_eq!(cursor.x, 2);
    assert_eq!(cursor.y, 1);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0], "t");
  }
}
