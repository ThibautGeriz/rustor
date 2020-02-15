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

pub fn init_lines(file_name_option: &Option<&String>) -> Vec<String> {
  let mut lines: Vec<String> = vec![];
  if let Some(file_name) = file_name_option {
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

pub fn save_to_file(file_name: &String, lines: &Vec<String>) -> std::io::Result<()> {
  let mut file = File::create(file_name)?;
  let content: String = lines.join("\n");
  file.write_all(content.as_bytes())?;
  Ok(())
}
