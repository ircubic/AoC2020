use std::path::Path;
use AoC2020::utils::EntryIterator;
use std::collections::{HashMap, HashSet};
use regex::Regex;


#[derive(Debug)]
struct Passport {
  ecl: String,
  pid: String,
  eyr: String,
  hcl: String,
  byr: String,
  iyr: String,
  cid: Option<String>,
  hgt: String,
}

impl Passport {
  fn validate_ecl(str: &str) -> Option<bool>
  {
    Some(match str {
      "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
      _ => false
    })
  }

  fn validate_year(str: &str, max: usize, min: usize) -> Option<bool>
  {
    Some(match str.parse::<usize>().ok() {
      Some(num) => (num <= max && num >= min),
      _ => false
    })
  }
  fn validate_byr(str: &str) -> Option<bool>
  {
    Self::validate_year(str, 2002, 1920)
  }

  fn validate_iyr(str: &str) -> Option<bool>
  {
    Self::validate_year(str, 2020, 2010)
  }

  fn validate_eyr(str: &str) -> Option<bool>
  {
    Self::validate_year(str, 2030, 2020)
  }

  fn validate_hcl(str: &str) -> Option<bool>
  {
    Some(Regex::new("^#[a-f0-9]{6}$").ok()?.is_match(str))
  }

  fn validate_hgt(str: &str) -> Option<bool>
  {
    let caps = Regex::new(r"^(\d+)(cm|in)$").ok()?.captures(str)?;

    let num = caps.get(1)?.as_str().parse::<usize>().ok()?;
    let unit = caps.get(2)?.as_str();

    Some(match unit {
      "cm" if num >= 150 && num <= 193 => true,
      "in" if num >= 59 && num <= 76 => true,
      _ => false
    })
  }

  fn validate_pid(str: &str) -> Option<bool>
  {
    Some(Regex::new(r"^\d{9}$").ok()?.is_match(str))
  }

  fn validate_entry(key: &str, value: &str) -> bool {
    let result = match key {
      "ecl" => Self::validate_ecl(&value),
      "byr" => Self::validate_byr(&value),
      "iyr" => Self::validate_iyr(&value),
      "eyr" => Self::validate_eyr(&value),
      "hgt" => Self::validate_hgt(&value),
      "hcl" => Self::validate_hcl(&value),
      "pid" => Self::validate_pid(&value),
      "cid" => Some(true),
      _ => Some(false)
    };
    if let Some(result) = result
    {
      result
    } else {
      false
    }
  }

  fn extract_entries(line: &str) -> Vec<(String, String)>
  {
    line
      .split(" ")
      .map(|e| e.split(":").collect::<Vec<_>>())
      .filter(|e| e.len() == 2)
      .map(|s| {
        (s[0].to_owned(), s[1].to_owned())
      })
      .collect()
  }

  fn from_map(mut map: HashMap<String, String>) -> Option<Self>
  {
    Some(Passport {
      ecl: map.remove("ecl")?,
      pid: map.remove("pid")?,
      eyr: map.remove("eyr")?,
      hcl: map.remove("hcl")?,
      byr: map.remove("byr")?,
      iyr: map.remove("iyr")?,
      cid: map.remove("cid"),
      hgt: map.remove("hgt")?,
    })
  }

  fn new(line: &str) -> Option<Self>
  {
    Self::from_map(Self::extract_entries(line).into_iter().collect())
  }

  fn new_validated(line: &str) -> Option<Self>
  {
    let mut seen_keys = HashSet::<String>::new();
    let mut valid = true;

    let filtered = Self::extract_entries(line).into_iter()
      .filter(|(k, v)| Self::validate_entry(k, v))
      .inspect(|(k, _)| {
        if seen_keys.contains(k) {
          valid = false;
        } else {
          seen_keys.insert(k.clone());
        }
      })
      .collect::<HashMap<_, _>>();

    if valid {
      Self::from_map(filtered)
    } else {
      None
    }
  }
}

pub fn problem1(path: &Path) -> usize {
  // let mut it = EntryIterator::new(path);
  // let string = it.next().unwrap();
  // println!("{}", string);
  // let string = it.next().unwrap();
  // println!("{}", string);
  // let _test = Passport::new(&string);
  // let passport = _test.unwrap();
  // println!("{:?}", passport);
  // passport.byr.len()
  EntryIterator::new(path)
    .map(|line| Passport::new(&line)).filter(|p| p.is_some()).count()
}

pub fn problem2(path: &Path) -> usize
{
  EntryIterator::new(path)
    .map(|line| Passport::new_validated(&line)).filter(|p| p.is_some()).count()
}

fn main() {
  let path = Path::new(r"data/4-1.txt");
  println!("Result of problem 1: {}", problem1(path));
  println!("Result of problem 2: {}", problem2(path));
}
