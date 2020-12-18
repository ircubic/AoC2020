struct NumberList
{
  last_num: usize,
  last_use: Vec<Option<usize>>,
  turn: usize,
}

impl NumberList
{
  fn new(starting_numbers: &Vec<usize>) -> Self
  {
    let last_num = *starting_numbers.last().unwrap();
    let turn = starting_numbers.len();
    let mut list = NumberList { last_num, last_use: Vec::<Option<usize>>::new(), turn };
    list.last_use.resize(256, None);
    for (i, &n) in starting_numbers.iter().take(starting_numbers.len() - 1).enumerate() {
      list.set_last_used(n, i)
    }
    list
  }

  fn get_last_used(&self, n: usize) -> Option<usize>
  {
    self.last_use[n]
  }

  fn set_last_used(&mut self, n: usize, turn: usize)
  {
    self.last_use[n] = Some(turn);
  }

  fn do_turn(&mut self) -> usize
  {
    let previous_turn = self.turn - 1;
    let age = if let Some(last_used) = self.get_last_used(self.last_num) {
      previous_turn - last_used
    } else {
      0
    };
    self.set_last_used(self.last_num, previous_turn);
    self.last_num = age;
    self.turn += 1;
    age
  }

  fn run_until_turn(&mut self, turn: usize) -> usize
  {
    self.last_use.resize(turn + 1, None);
    while self.turn < (turn - 1) {
      self.do_turn();
    }
    self.do_turn()
  }
}

fn problem1() -> usize
{
  let mut list = NumberList::new(&vec![9, 6, 0, 10, 18, 2, 1]);
  list.run_until_turn(2020)
}

fn problem2() -> usize
{
  let mut list = NumberList::new(&vec![9, 6, 0, 10, 18, 2, 1]);
  list.run_until_turn(30000000)
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
  fn test_do_turn()
  {
    let mut list = NumberList::new(&vec![0, 3, 6]);

    assert_eq!(list.do_turn(), 0);
    assert_eq!(list.do_turn(), 3);
    assert_eq!(list.do_turn(), 3);
    assert_eq!(list.do_turn(), 1);
    assert_eq!(list.do_turn(), 0);
    assert_eq!(list.do_turn(), 4);
    assert_eq!(list.do_turn(), 0);
  }

  #[test]
  fn test_run_until_turn_2020()
  {
    let mut list1 = NumberList::new(&vec![0, 3, 6]);
    let mut list2 = NumberList::new(&vec![1, 3, 2]);
    let mut list3 = NumberList::new(&vec![2, 1, 3]);
    let mut list4 = NumberList::new(&vec![1, 2, 3]);
    let mut list5 = NumberList::new(&vec![2, 3, 1]);

    assert_eq!(list1.run_until_turn(2020), 436);
    assert_eq!(list2.run_until_turn(2020), 1);
    assert_eq!(list3.run_until_turn(2020), 10);
    assert_eq!(list4.run_until_turn(2020), 27);
    assert_eq!(list5.run_until_turn(2020), 78);
  }

  #[test]
  fn test_run_until_turn_30000000()
  {
    let mut list1 = NumberList::new(&vec![0, 3, 6]);
    let mut list2 = NumberList::new(&vec![1, 3, 2]);
    let mut list3 = NumberList::new(&vec![2, 1, 3]);
    let mut list4 = NumberList::new(&vec![1, 2, 3]);
    let mut list5 = NumberList::new(&vec![2, 3, 1]);

    assert_eq!(list1.run_until_turn(30000000), 175594);
    assert_eq!(list2.run_until_turn(30000000), 2578);
    assert_eq!(list3.run_until_turn(30000000), 3544142);
    assert_eq!(list4.run_until_turn(30000000), 261214);
    assert_eq!(list5.run_until_turn(30000000), 6895259);
  }
}