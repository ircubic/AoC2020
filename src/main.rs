use std::io::{self, BufRead};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::collections::HashSet;

const PATH: &str = r"data";

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
  where P: AsRef<Path>
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn read_numbers_from_lines<P>(filename: P) -> io::Result<impl Iterator<Item=isize>>
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

fn aoc01_2(path: &Path) -> isize {
  let mut numbers = HashSet::<isize>::new();

  for num in read_numbers_from_lines(path).unwrap() {
    if numbers.len() < 2 { continue; }

    for other_num in numbers.iter() {
      let target = 2020 - other_num - num;
      if target != *other_num && target != num && numbers.contains(&target) {
        return target * other_num * num;
      }
    }

    numbers.insert(num);
  }

  panic!("Unable to find a solution")
}

fn aoc01_1(path: &Path) -> isize {
  let mut numbers = HashSet::<isize>::new();

  for num in read_numbers_from_lines(path).unwrap() {
    let target = 2020 - num;
    if numbers.contains(&target) {
      return target * num;
    }
    numbers.insert(num);
  }

  panic!("Unable to find a solution")
}

fn main() {
  let path = PathBuf::from(PATH);
  println!("Result of 1-1: {}", aoc01_1(&path.join(r"1-1.txt")));
  println!("Result of 1-2: {}", aoc01_2(&path.join(r"1-1.txt")));
}
