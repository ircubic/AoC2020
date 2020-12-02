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

fn aoc01_1(path: &Path) -> usize {
  let mut numbers = HashSet::<isize>::new();

  for num in read_numbers_from_lines(path).unwrap() {
    let target = 2020 - num;
    if numbers.contains(&target) {
      return (target * num) as usize;
    }
    numbers.insert(num);
  }

  panic!("Unable to find a solution")
}

fn aoc01_2(path: &Path) -> usize {
  let mut numbers = HashSet::<isize>::new();

  for num in read_numbers_from_lines(path).unwrap() {
    for other_num in numbers.iter() {
      let target = 2020 - other_num - num;
      if target != *other_num && target != num && numbers.contains(&target) {
        return (target * other_num * num) as usize;
      }
    }

    numbers.insert(num);
  }

  panic!("Unable to find a solution")
}

#[derive(Debug)]
struct AoC02PasswordEntry {
  min: i32,
  max: i32,
  character: char,
  password: Vec<char>,
}

impl AoC02PasswordEntry {
  fn from_line(line: &str) -> Self
  {
    let split = line.split(&['-', ' ', ':'][..]).filter(|x| x.len() > 0).collect::<Vec<_>>();
    let min = split[0].parse::<i32>().expect("Unable to parse min from string") - 1;
    let max = split[1].parse::<i32>().expect("Unable to parse max from string") - 1;
    let character = split[2].chars().nth(0).expect("Unable to parse char from string");
    AoC02PasswordEntry { min, max, character, password: split[3].chars().collect() }
  }

  fn validate_01(&self) -> bool
  {
    let amount = self.password.iter().filter(|&&c| c == self.character).count() as i32;
    amount >= self.min && amount <= self.max
  }

  fn validate_02(&self) -> bool
  {
    if self.max >= self.password.len() as i32 {
      return false
    }
    [self.min, self.max].iter()
      .filter(|&&n| self.password[n as usize] == self.character)
      .count() == 1
  }
}

fn aoc02_1(path: &Path) -> usize {
  read_lines(path).unwrap()
    .filter(|l|
      AoC02PasswordEntry::from_line(&l.as_ref().unwrap()).validate_01()
    )
    .count()
}

fn aoc02_2(path: &Path) -> usize {
  read_lines(path).unwrap()
    .filter(|l|
      AoC02PasswordEntry::from_line(&l.as_ref().unwrap()).validate_02()
    )
    .count()
}


fn main() {
  let path = PathBuf::from(PATH);
  println!("Result of 1-1: {}", aoc01_1(&path.join(r"1-1.txt")));
  println!("Result of 1-2: {}", aoc01_2(&path.join(r"1-1.txt")));
  println!("Result of 2-1: {}", aoc02_1(&path.join(r"2-1.txt")));
  println!("Result of 2-2: {}", aoc02_2(&path.join(r"2-1.txt")));
}
