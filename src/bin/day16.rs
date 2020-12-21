use std::path::Path;
use std::ops::RangeInclusive;
use std::collections::{HashMap, HashSet};
use AoC2020::utils::{split_in_two, read_lines};

struct Ticket(Vec<usize>);

impl Ticket
{
  fn from_string(string: &str) -> Ticket
  {
    Ticket(string.split(',').map(|s| s.parse::<usize>().unwrap_or(0)).collect())
  }
}

#[derive(Debug, Eq, PartialEq)]
struct Classifier
{
  left: RangeInclusive<usize>,
  right: RangeInclusive<usize>,
}

impl Classifier
{
  fn contains(&self, n: usize) -> bool
  {
    self.left.contains(&n) || self.right.contains(&n)
  }
}

impl Classifier
{
  fn from_string(string: &str) -> Classifier
  {
    let range_from_str = |s: &str| {
      let split = split_in_two(s, "-").iter().map(|s| {
        s.parse::<usize>().unwrap()
      }).collect::<Vec<_>>();
      RangeInclusive::new(split[0], split[1])
    };

    let classifiers = split_in_two(string, " or ");
    Classifier { left: range_from_str(&classifiers[0]), right: range_from_str(&classifiers[1]) }
  }
}

struct Classifiers(HashMap<String, Classifier>);

impl Classifiers
{
  fn from_lines<T, I>(lines: I) -> Classifiers
    where I: Iterator<Item=T>,
          T: AsRef<str>
  {
    Classifiers(
      lines
        .map(|line| {
          let arr = split_in_two(line.as_ref(), ": ");
          (arr[0].clone(), Classifier::from_string(&arr[1]))
        })
        .collect()
    )
  }

  fn validate_ticket(&self, ticket: &Ticket) -> Result<usize, usize>
  {
    match ticket.0.iter().filter_map(|n|
      if self.0.iter().any(|(_, c)| c.contains(*n)) {
        None
      } else {
        Some(*n)
      }
    ).next() {
      None => Ok(0),
      Some(n) => Err(n)
    }
  }
}

fn parse<I, T>(mut lines: I) -> (Classifiers, Vec<Ticket>)
  where I: Iterator<Item=T>,
        T: AsRef<str>
{
  let classifiers = lines.by_ref().take_while(|s| !s.as_ref().is_empty()).collect::<Vec<_>>();
  let your_ticket = lines.by_ref().skip(1).next().unwrap();
  let mut other_tickets = lines.skip(2).collect::<Vec<_>>();
  other_tickets.insert(0, your_ticket);
  (Classifiers::from_lines(classifiers.into_iter()), other_tickets.iter().map(|t| Ticket::from_string(t.as_ref())).collect())
}

fn parse_file(path: &Path) -> (Classifiers, Vec<Ticket>)
{
  let lines = read_lines(&path).unwrap().map(|s| s.unwrap());
  parse(lines)
}

fn problem2(path: &Path) -> usize
{
  let (classifiers, tickets) = parse_file(&path);
  let my_ticket = tickets.iter().next().unwrap();
  let valid_tickets = tickets.iter().skip(1).filter(|t| classifiers.validate_ticket(t).is_ok()).collect::<Vec<_>>();
  let possible_classifiers = classifiers.0.keys().collect::<HashSet<_>>();
  let mut key_order = Vec::<Vec<String>>::new();
  let num_keys = my_ticket.0.len();

  for i in 0..num_keys {
    let mut classifier_keys = possible_classifiers.iter().map(|&s| s.clone()).collect::<Vec<_>>();
    for t in &valid_tickets
    {
      let ticket_entry = t.0[i];
      classifier_keys = classifier_keys.into_iter().filter(|c| classifiers.0[c].contains(ticket_entry)).collect();
    }

    key_order.push(classifier_keys);
  }

  let mut found: HashMap<String, usize> = HashMap::with_capacity(num_keys);
  for _ in 0..num_keys {
    let (unique_index, unique_key) = key_order.iter()
      .enumerate()
      .find_map(|(i, v)|
        if v.len() == 1 {
          Some((i, v[0].clone()))
        } else {
          None
        })
      .unwrap();

    // Filter out found keys
    key_order = key_order.iter()
      .map(|o|
        o.iter()
          .cloned()
          .filter(|s |
            s != &unique_key
          )
          .collect())
      .collect();

    found.insert(unique_key, unique_index);
  }


  found.into_iter().filter_map(|(k,i)|
    if k.starts_with("departure") {
      Some(my_ticket.0[i])
    } else {
      None
    }
  ).product()
}

fn problem1(path: &Path) -> usize
{
  let (classifiers, tickets) = parse_file(&path);
  tickets.into_iter().skip(1).filter_map(|t| classifiers.validate_ticket(&t).err()).sum()
}

fn main() {
  let path = Path::new(r"data/16-1.txt");
  println!("Result of problem 1: {}", problem1(path));
  println!("Result of problem 2: {}", problem2(path));
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn test_parse_ticket()
  {
    assert_eq!(Ticket::from_string("7,1,14").0, vec![7, 1, 14]);
    assert_eq!(Ticket::from_string("7,3,47").0, vec![7, 3, 47]);
  }

  #[test]
  fn test_validate_ticket()
  {
    let classifiers = Classifiers::from_lines(r"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50".lines());

    assert_eq!(classifiers.validate_ticket(&Ticket::from_string("7,1,14")), Ok(0));
    assert_eq!(classifiers.validate_ticket(&Ticket::from_string("7,3,47")), Ok(0));
    assert_eq!(classifiers.validate_ticket(&Ticket::from_string("40,4,50")), Err(4));
    assert_eq!(classifiers.validate_ticket(&Ticket::from_string("55,2,20")), Err(55));
    assert_eq!(classifiers.validate_ticket(&Ticket::from_string("38,6,12")), Err(12));
  }

  #[test]
  fn test_parse_classifiers() {
    let classifiers = Classifiers::from_lines(r"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50".lines());

    assert_eq!(Classifier { left: RangeInclusive::new(1, 3), right: RangeInclusive::new(5, 7) }, classifiers.0["class"]);
    assert_eq!(Classifier { left: RangeInclusive::new(6, 11), right: RangeInclusive::new(33, 44) }, classifiers.0["row"]);
    assert_eq!(Classifier { left: RangeInclusive::new(13, 40), right: RangeInclusive::new(45, 50) }, classifiers.0["seat"]);
  }

  #[test]
  fn test_parse()
  {
    let it = r"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12".lines();

    let (classifiers, tickets) = parse(it);

    assert_eq!(Classifier { left: RangeInclusive::new(1, 3), right: RangeInclusive::new(5, 7) }, classifiers.0["class"]);
    assert_eq!(Classifier { left: RangeInclusive::new(6, 11), right: RangeInclusive::new(33, 44) }, classifiers.0["row"]);
    assert_eq!(Classifier { left: RangeInclusive::new(13, 40), right: RangeInclusive::new(45, 50) }, classifiers.0["seat"]);

    assert_eq!(tickets[0].0, vec![7, 1, 14]);
    assert_eq!(tickets[1].0, vec![7, 3, 47]);
    assert_eq!(tickets[2].0, vec![40, 4, 50]);
    assert_eq!(tickets[3].0, vec![55, 2, 20]);
    assert_eq!(tickets[4].0, vec![38, 6, 12]);
  }
}