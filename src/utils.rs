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

pub struct EntryIterator {
  lines: Lines<BufReader<File>>,
}

impl EntryIterator {
  pub fn new(path: &Path) -> Self {
    EntryIterator { lines: read_lines(path).unwrap() }
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
      .join(" ");
    match line.len() {
      0 => None,
      _ => Some(line)
    }
  }
}
