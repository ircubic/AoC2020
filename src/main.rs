use std::path::PathBuf;

mod utils;
mod day1;
mod day2;
mod day3;

const PATH: &str = r"data";

fn main() {
  let path = PathBuf::from(PATH);
  println!("Result of 1-1: {}", day1::problem1(&path.join(r"1-1.txt")));
  println!("Result of 1-2: {}", day1::problem2(&path.join(r"1-1.txt")));

  println!("Result of 2-1: {}", day2::problem1(&path.join(r"2-1.txt")));
  println!("Result of 2-2: {}", day2::problem2(&path.join(r"2-1.txt")));

  println!("Result of 3-1: {}", day3::problem1(&path.join(r"3-1.txt")));
  println!("Result of 3-2: {}", day3::problem2(&path.join(r"3-1.txt")));
}
