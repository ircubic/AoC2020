use AoC2020::utils::read_lines;
use std::path::Path;

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
    let split: Vec<&str> = line.split(&['-', ' ', ':'][..]).filter(|x| x.len() > 0).collect();
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
      return false;
    }
    [self.min, self.max].iter()
      .filter(|&&n| self.password[n as usize] == self.character)
      .count() == 1
  }
}

pub fn problem1(path: &Path) -> usize {
  read_lines(path).unwrap()
    .filter(|l|
      AoC02PasswordEntry::from_line(&l.as_ref().unwrap()).validate_01()
    )
    .count()
}

pub fn problem2(path: &Path) -> usize {
  read_lines(path).unwrap()
    .filter(|l|
      AoC02PasswordEntry::from_line(&l.as_ref().unwrap()).validate_02()
    )
    .count()
}
