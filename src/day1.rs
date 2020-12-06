use AoC2020::utils::read_numbers_from_lines;
use std::path::Path;
use std::collections::HashSet;

pub fn problem1(path: &Path) -> usize {
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

pub fn problem2(path: &Path) -> usize {
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

