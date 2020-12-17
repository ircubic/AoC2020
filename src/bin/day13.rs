use std::path::Path;

// Data is so small, I'll just include it right here
const NUM: usize = 1011416;
const LINE: &str = "41,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,37,x,x,x,x,x,911,x,x,x,x,x,x,x,x,x,x,x,x,13,17,x,x,x,x,x,x,x,x,23,x,x,x,x,x,29,x,827,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,19";

fn problem1() -> usize
{
  let nums = LINE
    .split(',')
    .filter(|&c| c != "x")
    .map(|s| {
      let n = s.parse::<usize>().unwrap_or(1);
      let res = (n, n - (NUM % n));
      res
    })
    .min_by(|(_, n), (_, b)| n.cmp(b))
    .unwrap();
  nums.0 * nums.1
}

// values = (m, t) where t is the order of departure, and m is the bus ID / modulo
fn chinese_remainder_theorem(enumerated: Vec<(usize, usize)>) -> usize
{
  let mut values = enumerated.into_iter().map(|(m, t)| (m, (m - (t % m)) % m)).collect::<Vec<_>>();
  values.sort_by(|(m1, _), (m2, _)| m1.cmp(m2));
  let (mut inc, mut acc) = values.pop().unwrap();
  for (m, t) in values.into_iter().rev() {
    while (acc % m) != t {
      acc += inc;
    }
    inc *= m
  }
  acc
}

fn problem2() -> usize
{
  let nums = LINE
    .split(',')
    .enumerate()
    .filter(|(i, v)| *v != "x")
    .map(|(i, m)| (m.parse::<usize>().unwrap(), i))
    .collect::<Vec<_>>();
  chinese_remainder_theorem(nums)
}

fn main() {
  println!("Result of problem 1: {}", problem1());
  println!("Result of problem 2: {}", problem2());
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn test_chinese_remainder_theorem()
  {
    let values: Vec<(usize, usize)> = vec![(7, 0), (13, 1), (59, 4), (31, 6), (19, 7)];
    assert_eq!(chinese_remainder_theorem(values), 1068781);
  }
}