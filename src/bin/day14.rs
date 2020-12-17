use std::path::Path;
use std::collections::HashMap;
use AoC2020::utils::read_lines;

// Represents a mask of the form "0XX10XX0110"
//
// This is used to mask a value like so: (V & filter) + values
//
// The example above results in the following values:
// - filter: 01100110000
// - values: 00010000110
struct Mask
{
  filter: usize,
  values: usize,
}

impl Mask {
  fn new() -> Self
  {
    Mask { filter: 0xFFFFFFFF, values: 0 }
  }

  fn from_string(s: &str) -> Self
  {
    let mut filter = 0usize;
    let mut values = 0usize;
    for c in s.chars()
    {
      let (f, v) = match c {
        '0' => (0, 0),
        '1' => (0, 1),
        'X' => (1, 0),
        _ => (1, 0)
      };
      filter = (filter << 1) + f;
      values = (values << 1) + v
    }
    Mask { filter, values }
  }

  fn mask_value(&self, n: usize) -> usize
  {
    (n & self.filter) + self.values
  }
}

struct MaskV2
{
  filter: u64,
  junctions: Vec<u64>,
}

impl MaskV2
{
  fn from_string(s: &str) -> Self
  {
    let mut junctions = Vec::<u64>::new();
    let mut filter = 0u64;
    for (i, c) in s.chars().rev().enumerate() {
      let bit = match (i, c) {
        (_, '1') => 1,
        (_, '0') => 0,
        (i, 'X') => {
          junctions.push(i as u64);
          0
        }
        _ => unreachable!()
      };
      filter = filter | (bit << i);
    }

    MaskV2 { filter, junctions }
  }

  fn mask_address(&self, a: u64) -> Vec<u64>
  {
    let mut addresses = vec![self.filter | a];
    for j in &self.junctions {
      let mut new_addresses = vec![];
      for a in addresses {
        let mask = 1 << j;
        new_addresses.push(a & (!mask));
        new_addresses.push(a | mask);
      }
      addresses = new_addresses
    }
    addresses
  }
}

fn parse_memory_assignment(s: &str) -> (u64, usize)
{
  let left_bracket = s.find("[").unwrap();
  let right_bracket = s.find("]").unwrap();
  let equals = s.find("=").unwrap();
  (
    s[(left_bracket + 1)..right_bracket].trim().parse::<u64>().unwrap(),
    s[(equals + 1)..].trim().parse::<usize>().unwrap()
  )
}

fn problem1(path: &Path) -> usize
{
  let mut memory = HashMap::<u64, usize>::new();
  let mut mask = Mask::new();
  for l in read_lines(&path).unwrap().map(|s| s.unwrap()) {
    if l.starts_with("mem") {
      let (a, v) = parse_memory_assignment(&l);
      memory.insert(a, mask.mask_value(v));
    } else if l.starts_with("mask")
    {
      mask = Mask::from_string(l[6..].trim());
    }
  }

  memory.values().sum()
}

fn problem2(path: &Path) -> usize
{
  let mut memory = HashMap::<u64, usize>::new();
  let mut mask = MaskV2 { filter: 0, junctions: vec![] };
  for l in read_lines(&path).unwrap().map(|s| s.unwrap()) {
    if l.starts_with("mem") {
      let (a, v) = parse_memory_assignment(&l);
      let addresses = mask.mask_address(a);
      for a in addresses {
        memory.insert(a, v);
      }
    } else if l.starts_with("mask")
    {
      mask = MaskV2::from_string(l[6..].trim());
    }
  }

  memory.values().sum()
}

fn main() {
  let path = Path::new(r"data/14-1.txt");
  println!("Result of problem 1: {}", problem1(path));
  println!("Result of problem 2: {}", problem2(path));
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn test_read_mask()
  {
    let mask = Mask::from_string("0XX10XX0110");
    assert_eq!(mask.filter, 0b01100110000);
    assert_eq!(mask.values, 0b00010000110);
  }

  #[test]
  fn test_mask_values()
  {
    let mask = Mask::from_string("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");

    assert_eq!(mask.mask_value(11), 73);
    assert_eq!(mask.mask_value(101), 101);
    assert_eq!(mask.mask_value(0), 64);
  }

  #[test]
  fn test_parse_mem_assignment()
  {
    assert_eq!(parse_memory_assignment("mem[10] = 47892"), (10, 47892));
    assert_eq!(parse_memory_assignment("mem[2450] = 68156"), (2450, 68156));
    assert_eq!(parse_memory_assignment("mem[123230] = 4741892"), (123230, 4741892));
  }

  #[test]
  fn test_mask_address()
  {
    let mask = MaskV2::from_string("000000000000000000000000000000X1001X");
    assert_eq!(mask.filter, 0b10010);
    assert_eq!(mask.junctions, vec![0, 5]);

    let mut addresses = mask.mask_address(42);
    addresses.sort();

    assert_eq!(addresses, vec![26, 27, 58, 59]);
  }
}