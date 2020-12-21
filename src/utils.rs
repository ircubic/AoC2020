use std::io::{self, BufRead, Lines, BufReader};
use std::fs::File;
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
  where P: AsRef<Path>
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

pub fn read_numbers_from_lines<P>(filename: P) -> io::Result<impl Iterator<Item=isize>>
  where P: AsRef<Path>
{
  Ok(read_lines(filename)?.map(|l| {
    let parse_result = l.expect("Unable to read line").trim().parse::<isize>();
    match parse_result {
      Ok(num) => num,
      Err(_) => panic!("Unable to parse integer")
    }
  }))
}

/// Iterator that reads from a file entries that may span over multiple lines and are separated by
/// empty lines.
///
/// The iterator will return one entry at a time as a String, with each line in the original entry
/// now separated by a custom separator.
pub struct EntryIterator {
  lines: Lines<BufReader<File>>,
  separator: String
}

impl EntryIterator {
  /// Create an iterator with the default separator (a space)
  pub fn new(path: &Path) -> Self {
    Self::new_with_separator(path, " ")
  }

  pub fn new_with_separator(path: &Path, separator: &str) -> Self
  {
    EntryIterator { lines: read_lines(path).unwrap(), separator: separator.to_string() }
  }
}

impl Iterator for EntryIterator
{
  type Item = String;

  fn next(&mut self) -> Option<Self::Item> {
    let line = self.lines.by_ref()
      .take_while(|x| x.is_ok() && x.as_ref().unwrap().trim().len() > 0)
      .map(|s| s.unwrap())
      .collect::<Vec<_>>()
      .join(&self.separator);
    match line.len() {
      0 => None,
      _ => Some(line)
    }
  }
}

pub fn split_in_two(s:&str, separator:&str) -> [String;2]
{
  let mut s : Vec<String> = s.split(separator).take(2).map(|s|s.to_string()).collect();
  let back = s.pop().unwrap();
  [s.pop().unwrap(), back]
}
