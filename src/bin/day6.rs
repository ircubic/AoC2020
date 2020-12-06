use std::path::Path;
use std::collections::{HashSet, BTreeSet};
use AoC2020::utils::EntryIterator;

fn get_unique_answers(entry: &str) -> Option<String>
{
  let set: BTreeSet<char> = entry.chars().filter(|&c| !c.is_whitespace()).collect();

  if set.iter().all(|&x| x >= 'a' && x <= 'z') {
    Some(set.iter().collect())
  } else {
    None
  }
}

fn get_universal_answers(entry: &str) -> Option<String>
{
  let dummy_set: BTreeSet<char> = ('a'..='z').collect();
  Some(entry
    .split(' ')
    .map(|line|
      line
        .chars()
        .collect::<BTreeSet<char>>())
    .fold(dummy_set, |acc, set| acc.intersection(&set).cloned().collect())
    .iter()
    .collect())
}

fn problem1(path: &Path) -> usize
{
  EntryIterator::new(path)
    .map(|s| get_unique_answers(&s))
    .filter(|s| s.is_some())
    .map(|s| s.unwrap().len())
    .sum()
}

fn problem2(path: &Path) -> usize
{
  EntryIterator::new(path)
    .map(|s| get_universal_answers(&s))
    .filter(|s| s.is_some())
    .map(|s| s.unwrap().len())
    .sum()
}

fn main() {
  let path = Path::new(r"data/6-1.txt");
  println!("Result of problem 1: {}", problem1(path));
  println!("Result of problem 2: {}", problem2(path));
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn test_count_uniques()
  {
    assert_eq!(get_unique_answers("abc").unwrap(), "abc");
    assert_eq!(get_unique_answers("a b c").unwrap(), "abc");
    assert_eq!(get_unique_answers("a ab bc").unwrap(), "abc");
    assert_eq!(get_unique_answers("a a a").unwrap(), "a");
    assert_eq!(get_unique_answers(r"bac").unwrap(), "abc");
  }

  #[test]
  fn test_count_universal()
  {
    assert_eq!(get_universal_answers("abc").unwrap(), "abc");
    assert_eq!(get_universal_answers("a b c").unwrap(), "");
    assert_eq!(get_universal_answers("abcd a abc").unwrap(), "a");
    assert_eq!(get_universal_answers("a a a").unwrap(), "a");
    assert_eq!(get_universal_answers("abcq cq cbq").unwrap(), "cq");
  }
}