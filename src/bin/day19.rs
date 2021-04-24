use std::path::Path;
use std::ops::RangeInclusive;
use std::collections::{HashSet, HashMap};
use std::cmp::{min, max};
use AoC2020::utils::{read_lines, split_in_two};
use std::hash::Hash;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Rule
{
  Sequence(Vec<Rule>),
  Alternative(Vec<Rule>),
  Reference(usize),
  Character(char),
}

fn parse_rule(line: &str) -> (usize, Rule)
{
  let [id, sequence] = split_in_two(line, ": ");

  let id = id.parse::<usize>().unwrap();
  let sequence = sequence
    .split(" | ")
    .map(|l| {
      let mut seq = l.split(" ")
        .map(|s| {
          let mut c = s.chars();
          match c.next().unwrap()
          {
            '"' => Rule::Character(c.next().unwrap()),
            '0'..='9' => Rule::Reference(s.parse::<usize>().unwrap()),
            _ => panic!("Invalid element {}", s)
          }
        }).collect::<Vec<Rule>>();

      if seq.len() == 1
      {
        seq.pop().unwrap()
      } else {
        Rule::Sequence(seq)
      }
    }).collect::<Vec<Rule>>();

  (id,
   if sequence.len() == 1 {
     sequence[0].clone()
   } else {
     Rule::Alternative(sequence)
   })
}

fn expand_rule(rule: &Rule, rules: &HashMap<usize, Rule>) -> Rule
{
  match rule {
    Rule::Alternative(a) => Rule::Alternative(a.iter().map(|s| expand_rule(s, &rules)).collect()),
    Rule::Sequence(v) => Rule::Sequence(v.iter().map(|m| expand_rule(m, &rules)).collect()),
    Rule::Reference(id) => expand_rule(rules.get(id).unwrap(), &rules),
    Rule::Character(_) => rule.clone()
  }
}

fn expand_rules(rules: &HashMap<usize, Rule>) -> HashMap<usize, Rule>
{
  rules.iter().map(|(id, rule)| (*id, expand_rule(rule, rules))).collect()
}

fn match_child(rule: &Rule, s: &str, rules: &HashMap<usize, Rule>) -> Option<usize>
{
  let result = match rule {
    Rule::Alternative(a) => {
      a.iter().find_map(|r| match_child(r, s, rules))
    }
    Rule::Sequence(v) => {
      v.iter().fold(Some(0usize), |acc, r| {
        let count = acc?;
        if count >= s.len() {
          None
        } else {
          Some(count + match_child(r, &s[count..], rules)?)
        }
      })
    }
    Rule::Reference(id) => match_child(rules.get(id)?, s, rules),
    Rule::Character(c) => {
      if s.chars().next()? == *c {
        Some(1)
      } else {
        None
      }
    }
  };

  result
}

fn match_rule(rule: &Rule, s: &str, rules: &HashMap<usize, Rule>) -> bool
{
  match_child(rule, s, rules).unwrap_or(0) == s.len()
}

fn match_two_rules(rule1: &Rule, rule2: &Rule, s: &str, rules: &HashMap<usize, Rule>) -> bool
{
  let mut stack: Vec<usize> = vec![];
  let mut i = 0usize;

  while let Some(len) = match_child(rule1, &s[i..], rules) {
    stack.push(len);
    i += len;
    if i >= s.len() {
      return false;
    }
  }

  loop {
     if stack.len() < 2 {
       return false;
     }

    let mut j = i;
    let mut rule_two_amount = 0usize;
    while let Some(len) = match_child(rule2, &s[j..], rules) {
      j += len;
      rule_two_amount += 1;
      // The rules 8 and 11 essentially expands to any amount of rule 42, then followed by an equal
      // amount of 42s and 31s. Ergo, a valid rule is one where the amount of rule 42 matches are at
      // least one more than the amount of rule 31 matches
      if j == s.len() && stack.len() > rule_two_amount {
        return true;
      }
    }

    if let Some(down) = stack.pop() {
      i -= down;
    } else {
      return false;
    }
  }

}

fn problem1(path: &Path) -> usize
{
  let mut it = read_lines(path).unwrap();
  let rules = it.by_ref().take_while(|s| !s.as_ref().unwrap().is_empty())
    .map(|s| parse_rule(&s.unwrap()))
    .collect::<HashMap<usize, Rule>>();

  let rule0 = rules.get(&0).unwrap();
  it.map(|s| if match_rule(&rule0, &s.unwrap(), &rules) { 1 } else { 0 }).sum()
}

fn problem2(path: &Path) -> usize
{
  let mut it = read_lines(path).unwrap();
  let mut rules = it.by_ref().take_while(|s| !s.as_ref().unwrap().is_empty())
    .map(|s| parse_rule(&s.unwrap()))
    .collect::<HashMap<usize, Rule>>();

  rules.insert(8, Rule::Alternative(vec![Rule::Sequence(vec![Rule::Reference(42), Rule::Reference(8)]), Rule::Reference(42)]));
  rules.insert(11, Rule::Alternative(
    vec![Rule::Sequence(vec![Rule::Reference(42), Rule::Reference(11), Rule::Reference(31)]),
         Rule::Sequence(vec![Rule::Reference(42), Rule::Reference(31)])]));

  let rule42 = rules.get(&42).unwrap();
  let rule31 = rules.get(&31).unwrap();
  it.map(|s| if match_two_rules(&rule42, &rule31, &s.unwrap(), &rules) { 1 } else { 0 }).sum()
  // read_lines(path)
  //   .unwrap()
}

fn main() {
  let path = Path::new(r"data/19-1.txt");
  println!("Result of problem 1: {}", problem1(path));
  println!("Result of problem 2: {}", problem2(path));
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn test_parse() {
    assert_eq!(parse_rule("0: 4 1 5"),
               (0usize, Rule::Sequence(vec![Rule::Reference(4), Rule::Reference(1), Rule::Reference(5)])));

    assert_eq!(parse_rule("3: 4 | 1 5"),
               (3usize, Rule::Alternative(vec![
                 Rule::Reference(4),
                 Rule::Sequence(vec![Rule::Reference(1), Rule::Reference(5)])])));
  }

  #[test]
  fn test_expand_rules() {
    let rules = vec!["0: 4 1 5",
                     "1: 2 3 | 3 2",
                     "2: 4 4 | 5 5",
                     "3: 4 5 | 5 4",
                     "4: \"a\"",
                     "5: \"b\""]
      .into_iter()
      .map(|s| parse_rule(s))
      .collect::<HashMap<usize, Rule>>();
    let expanded = expand_rules(&rules);

    let rule2 = Rule::Alternative(vec![
      Rule::Sequence(vec![Rule::Character('a'), Rule::Character('a')]),
      Rule::Sequence(vec![Rule::Character('b'), Rule::Character('b')]),
    ]);
    let rule3 = Rule::Alternative(vec![
      Rule::Sequence(vec![Rule::Character('a'), Rule::Character('b')]),
      Rule::Sequence(vec![Rule::Character('b'), Rule::Character('a')]),
    ]);

    assert_eq!(*expanded.get(&0).unwrap(),
               Rule::Sequence(vec![
                 Rule::Character('a'),
                 Rule::Alternative(vec![
                   Rule::Sequence(vec![rule2.clone(), rule3.clone()]),
                   Rule::Sequence(vec![rule3.clone(), rule2.clone()])
                 ]),
                 Rule::Character('b')
               ])
    );
  }

  #[test]
  fn test_match_rule()
  {
    let rules = vec!["0: 4 1 5",
                     "1: 2 3 | 3 2",
                     "2: 4 4 | 5 5",
                     "3: 4 5 | 5 4",
                     "4: \"a\"",
                     "5: \"b\""]
      .into_iter()
      .map(|s| parse_rule(s))
      .collect::<HashMap<usize, Rule>>();
    let rule0 = rules.get(&0).unwrap();

    assert!(match_rule(&rule0, "ababbb", &rules));
    assert!(!match_rule(&rule0, "bababa", &rules));
    assert!(match_rule(&rule0, "abbbab", &rules));
    assert!(!match_rule(&rule0, "aaabbb", &rules));
    assert!(!match_rule(&rule0, "aaaabbb", &rules));
  }

  #[test]
  fn test_recursive()
  {
    let rules = vec!["42: 9 14 | 10 1",
                     "9: 14 27 | 1 26",
                     "10: 23 14 | 28 1",
                     "1: \"a\"",
                     "11: 42 11 31 | 42 31",
                     "5: 1 14 | 15 1",
                     "19: 14 1 | 14 14",
                     "12: 24 14 | 19 1",
                     "16: 15 1 | 14 14",
                     "31: 14 17 | 1 13",
                     "6: 14 14 | 1 14",
                     "2: 1 24 | 14 4",
                     "0: 8 11",
                     "13: 14 3 | 1 12",
                     "15: 1 | 14",
                     "17: 14 2 | 1 7",
                     "23: 25 1 | 22 14",
                     "28: 16 1",
                     "4: 1 1",
                     "20: 14 14 | 1 15",
                     "3: 5 14 | 16 1",
                     "27: 1 6 | 14 18",
                     "14: \"b\"",
                     "21: 14 1 | 1 14",
                     "25: 1 1 | 1 14",
                     "22: 14 14",
                     "8: 42 8 | 42",
                     "26: 14 22 | 1 20",
                     "18: 15 15",
                     "7: 14 5 | 1 21",
                     "24: 14 1"]
      .into_iter()
      .map(|s| parse_rule(s))
      .collect::<HashMap<usize, Rule>>();
    let rule42 = rules.get(&42).unwrap();
    let rule31 = rules.get(&31).unwrap();

    let result: usize = vec!["aaaaabbaabaaaaababaa",
                             "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
                             "bbabbbbaabaabba",
                             "babbbbaabbbbbabbbbbbaabaaabaaa",
                             "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
                             "bbbbbbbaaaabbbbaaabbabaaa",
                             "bbbababbbbaaaaaaaabbababaaababaabab",
                             "ababaaaaaabaaab",
                             "ababaaaaabbbaba",
                             "baabbaaaabbaaaababbaababb",
                             "abbbbabbbbaaaababbbbbbaaaababb",
                             "aaaabbaaaabbaaa",
                             "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
                             "babaaabbbaaabaababbaabababaaab",
                             "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"]
      .into_iter()
      .map(|s| if match_two_rules(&rule42, &rule31, s, &rules) { 1 } else { 0 })
      .sum();
    assert_eq!(result, 12);
  }
}
