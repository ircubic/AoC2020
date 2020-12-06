use std::path::Path;
use AoC2020::utils::read_lines;
use std::cmp::{max, Ordering};

/// These "boarding pass numbers" are actually just binary numbers.
/// Both B and R represent 1s, F and L represent 0.
/// Ensuring that these are in the correct places should be done before calling this.
fn bp_str_to_num(bp_str: &str) -> Option<u16>
{
  if bp_str.len() > 16 {
    return None;
  }
  let mut num = 0u16;
  for c in bp_str.chars() {
    num = (num << 1) + match c {
      'B' | 'R' => 1,
      'F' | 'L' => 0,
      _ => return None
    }
  }
  Some(num)
}

#[derive(Debug, Eq)]
struct Seating {
  row: u16,
  col: u16,
  seat_id: u16,
}

impl PartialEq for Seating {
  fn eq(&self, other: &Self) -> bool {
    self.seat_id == other.seat_id
  }
}

impl PartialOrd for Seating
{
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(&other))
  }
}

impl Ord for Seating
{
  fn cmp(&self, other: &Self) -> Ordering {
    self.seat_id.cmp(&other.seat_id)
  }
}

fn decode_boarding_pass(str: &str) -> Option<Seating>
{
  if !(str.len() == 10
    && str[0..7].chars().all(|c| c == 'F' || c == 'B')
    && str[7..10].chars().all(|c| c == 'L' || c == 'R'))
  {
    return None;
  }

  let num = bp_str_to_num(str)?;
  Some(Seating { row: (num >> 3), col: (num & 0x7), seat_id: num })
}

pub fn problem1(path: &Path) -> usize {
  // Fairly easy, just parse the pseudo-binary code, then find the max value
  read_lines(path).unwrap()
    .map(|l| decode_boarding_pass(&l.unwrap()))
    .filter(|bp| bp.is_some())
    .fold(0, |acc, bp| max(acc, bp.unwrap().seat_id as usize))
}

pub fn problem2(path: &Path) -> usize {
  // This one basically means to find the place where the sequence would stop in a sorted list of
  // IDs, so create the sorted list of Seats
  let mut passes = read_lines(path).unwrap()
    .map(|l| decode_boarding_pass(&l.unwrap()))
    .filter(|bp| bp.is_some())
    .map(|bp| bp.unwrap())
    .collect::<Vec<_>>();
  passes.sort_unstable();

  // Then reduce, keeping the last value that was in a valid sequence, and add 1 to it at the end
  1 + passes.into_iter()
    .fold(None, |acc, bp| {
      let seat_id = bp.seat_id as usize;
      match acc {
        None => Some(seat_id),
        Some(previous) if previous == seat_id - 1 => Some(seat_id),
        _ => acc
      }
    })
    .unwrap()
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn bp_str_to_num_test()
  {
    assert_eq!(bp_str_to_num("FBFBBFFRLR"), Some(357));
  }

  #[test]
  fn binary_space_partition()
  {
    assert_eq!(decode_boarding_pass("FBFBBFFFRLR"), None);
    assert_eq!(decode_boarding_pass("FBFBBFFFLR"), None);
    assert_eq!(decode_boarding_pass("FBFBBFRRLR"), None);
    assert_eq!(decode_boarding_pass("FBFBBFFRLR").unwrap(), Seating { row: 44, col: 5, seat_id: 357 });
    assert_eq!(decode_boarding_pass("BFFFBBFRRR").unwrap(), Seating { row: 70, col: 7, seat_id: 567 });
    assert_eq!(decode_boarding_pass("FFFBBBFRRR").unwrap(), Seating { row: 14, col: 7, seat_id: 119 });
    assert_eq!(decode_boarding_pass("BBFFBBFRLL").unwrap(), Seating { row: 102, col: 4, seat_id: 820 });
  }
}

fn main() {
  let path = Path::new(r"data/5-1.txt");
  println!("Result of problem 1: {}", problem1(path));
  println!("Result of problem 2: {}", problem2(path));
}
