use std::collections::{HashSet, VecDeque};
use std::path::Path;
use AoC2020::utils::read_lines;
use std::borrow::Borrow;
use std::cmp::{max,min};

struct ValidNumberCollector
{
  valid_sums: HashSet<usize>,
  contained_numbers: VecDeque<usize>,
}

impl ValidNumberCollector
{
  fn create_valid_sums(numbers: &[usize]) -> HashSet<usize>
  {
    let mut valid_sums = HashSet::<usize>::new();

    for i in 0..(numbers.len() - 1) {
      for j in (i + 1)..numbers.len() {
        valid_sums.insert(numbers[i] + numbers[j]);
      }
    }

    valid_sums
  }

  fn new(preamble: &[usize]) -> Self
  {
    ValidNumberCollector {
      valid_sums: Self::create_valid_sums(preamble),
      contained_numbers: preamble.iter().cloned().collect(),
    }
  }

  fn is_number_valid(&self, num: usize) -> bool
  {
    self.valid_sums.contains(&num)
  }

  fn insert(&mut self, num: usize)
  {
    self.contained_numbers.pop_front();
    self.contained_numbers.push_back(num);
    self.contained_numbers.make_contiguous();
    self.valid_sums = Self::create_valid_sums(&self.contained_numbers.as_slices().0)
  }
}

fn find_invalid_number<'a, U, I>(n: usize, lines: I) -> usize
  where U: Borrow<usize>,
        I: Iterator<Item=U>
{
  let mut it = lines.map(|l|*l.borrow());
  let initial: Vec<usize> = it.by_ref().take(n).collect();
  let mut collector = ValidNumberCollector::new(&initial);
  assert_eq!(initial.len(), n);

  it.find(|&x| {
      let is_valid = collector.is_number_valid(x);
      collector.insert(x);
      !is_valid
    }).unwrap_or(0)
}

fn problem1(path: &Path) -> usize
{
  find_invalid_number(25, read_lines(path).unwrap().map(|x| x.unwrap().parse::<usize>().unwrap()))
}

fn problem2(path: &Path) -> usize
{
  let numbers = read_lines(path)
    .unwrap()
    .map(|x| x.unwrap().parse::<usize>().unwrap())
    .collect::<Vec<_>>();

  let invalid = find_invalid_number(25, numbers.iter());

  for (i,n) in numbers.iter().enumerate() {
    let mut sum = *n;
    let mut j = i+1;
    let mut high = sum;
    let mut low = sum;
    while sum < invalid && j < numbers.len() {
      let inner = numbers[j];
      sum += inner;
      high = max(high, inner);
      low = min(low, inner);
      j+=1
    }

    if sum == invalid {
      return low + high;
    }
  }

  0
}

fn main() {
  let path = Path::new(r"data/9-1.txt");
  println!("Result of problem 1: {}", problem1(path));
  println!("Result of problem 2: {}", problem2(path));
}

#[cfg(test)]
mod tests
{
  use crate::{ValidNumberCollector, find_invalid_number};

  #[test]
  fn test_collect_valid_numbers()
  {
    let collector = ValidNumberCollector::new(&vec![3, 4, 8]);

    assert!(collector.is_number_valid(7));
    assert!(collector.is_number_valid(11));
    assert!(collector.is_number_valid(12));
    assert!(!collector.is_number_valid(3));
    assert!(!collector.is_number_valid(4));
    assert!(!collector.is_number_valid(8));
    assert!(!collector.is_number_valid(42));
  }

  #[test]
  fn test_insert_number()
  {
    let mut collector = ValidNumberCollector::new(&vec![3, 4, 8]);

    collector.insert(5);

    assert!(collector.is_number_valid(12));
    assert!(collector.is_number_valid(9));
    assert!(collector.is_number_valid(13));
    assert!(!collector.is_number_valid(5));
    assert!(!collector.is_number_valid(4));
    assert!(!collector.is_number_valid(8));
    assert!(!collector.is_number_valid(11));
    assert!(!collector.is_number_valid(7));
  }

  #[test]
  fn test_find_invalid_number()
  {
    let nums: Vec<usize> = r"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576".split("\n").map(|n| n.parse::<usize>().unwrap()).collect();

    assert_eq!(find_invalid_number(5, nums.iter()), 127);
  }
}