use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn init_lines(file_name_option: Option<&String>) -> Vec<String> {
    let mut lines: Vec<String> = vec![];
    if let Some(file_name) = file_name_option {
        if let Ok(lines_in_file) = read_lines(file_name) {
            for line_in_file in lines_in_file {
                if let Ok(line) = line_in_file {
                    lines.push(line);
                }
            }
        }
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

pub fn save_to_file(file_name: &str, lines: &[String]) -> std::io::Result<()> {
    let mut file = File::create(file_name)?;
    let content: String = lines.join("\n");
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn init_lines_should_return_an_empty_line_when_there_is_no_file() {
        // Given
        let file_name_option: Option<&String> = None;
        let expected: Vec<String> = vec![String::new()];

        // When
        let result = init_lines(file_name_option);

        // Then
        assert_eq!(expected, result);
    }

    #[test]
    fn init_lines_should_return_an_empty_line_when_there_the_file_is_empty() {
        // Given
        let file_name = String::from("test_file.txt");
        let _file = File::create(&file_name);
        let file_name_option: Option<&String> = Some(&file_name);
        let expected: Vec<String> = vec![String::new()];

        // When
        let result = init_lines(file_name_option);

        // Then
        assert_eq!(expected, result);

        fs::remove_file(&file_name).unwrap();
    }
}
