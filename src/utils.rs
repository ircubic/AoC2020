use std::io::{self, BufRead};
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

