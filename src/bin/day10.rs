use std::collections::HashMap;
use std::cmp::max;
use std::path::Path;
use AoC2020::utils::read_lines;

fn get_numbers(path: &Path) -> Vec<usize>
{
  let mut numbers: Vec<usize> = vec![0];
  numbers.extend(read_lines(path).unwrap().map(|l| l.unwrap().parse::<usize>().unwrap()));
  numbers.sort();
  numbers.push(numbers.last().unwrap() + 3);
  numbers
}

fn count_possibilities(list: Vec<usize>) -> usize
{
  let highest = list.last().unwrap();
  let mut map = HashMap::<usize, usize>::with_capacity(list.len());
  map.insert(0, 1);

  for &e in list.iter().skip(1) {
    let mut count = 0;
    if e > 0 {
      count += map.get(&(e - 1)).unwrap_or(&0usize);
    }

    if e > 1 {
      count += map.get(&(e - 2)).unwrap_or(&0usize);
    }

    if e > 2 {
      count += map.get(&(e - 3)).unwrap_or(&0usize);
    }

    map.insert(e, count);
  }

  map[highest]
}

fn problem2(path: &Path) -> usize
{
  count_possibilities(get_numbers(path))
}

fn problem1(path: &Path) -> usize
{
  let list = get_numbers(path);
  let mut previous = list.last().unwrap();
  let mut three_diff = 0;
  let mut one_diff = 0;

  for l in list.iter().rev().skip(1) {
    match previous - l {
      3 => three_diff += 1,
      1 => one_diff += 1,
      _ => ()
    };
    previous = l;
  }
  three_diff * one_diff
}

fn main() {
  let path = Path::new(r"data/10-1.txt");
  println!("Result of problem 1: {}", problem1(path));
  println!("Result of problem 2: {}", problem2(path));
}

#[cfg(test)]
mod tests
{
  use crate::count_possibilities;

  #[test]
  fn test_count_possibilities()
  {
    let mut nums: Vec<usize> = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3, 0, 52];
    nums.sort();

    assert_eq!(count_possibilities(nums), 19208);
  }
}