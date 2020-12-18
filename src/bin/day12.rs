use std::path::Path;
use AoC2020::utils::read_lines;

fn heading_to_delta(heading: isize) -> (isize, isize)
{
  let abs_heading = (360 + heading) % 360;

  (
    match abs_heading {
      1..=179 => 1,
      181..=359 => -1,
      _ => 0
    },
    match abs_heading {
      0..=89 | 271..=359 => 1,
      91..=269 => -1,
      _ => 0
    }
  )
}

#[derive(Debug, Eq, PartialEq)]
enum Instruction
{
  North(usize),
  East(usize),
  South(usize),
  West(usize),
  Right(usize),
  Left(usize),
  Forward(usize),
}

#[derive(Debug, Eq, PartialEq)]
struct Ship
{
  x: isize,
  y: isize,
  // degrees rotated clockwise from facing north
  heading: isize,
}

impl Ship {
  fn new() -> Self
  {
    Ship { x: 0, y: 0, heading: 90 }
  }

  fn drive(&mut self, heading: isize, amount: usize)
  {
    let (dx, dy) = heading_to_delta(heading);
    self.x += dx * (amount as isize);
    self.y += dy * (amount as isize);
  }

  fn rotate(&mut self, degrees: isize)
  {
    self.heading += degrees;
  }

  fn follow_instruction(&mut self, instruction: Instruction)
  {
    match instruction {
      Instruction::Right(r) => self.rotate(r as isize),
      Instruction::Left(l) => self.rotate(-(l as isize)),
      Instruction::Forward(n) => self.drive(self.heading, n),
      Instruction::North(n) => self.drive(0, n),
      Instruction::East(n) => self.drive(90, n),
      Instruction::South(n) => self.drive(180, n),
      Instruction::West(n) => self.drive(270, n)
    }
  }

  fn drive_to_waypoint(&mut self, wp: &Waypoint, n: usize)
  {
    self.x += wp.x * (n as isize);
    self.y += wp.y * (n as isize);
  }
}

// I know the Waypoint is just a standin for a transformation matrix, but ugh, lazy
#[derive(Debug, Eq, PartialEq)]
struct Waypoint
{
  x: isize,
  y: isize,
}

impl Waypoint
{
  fn new(x: isize, y: isize) -> Self
  {
    Waypoint { x, y }
  }

  fn rotate(&mut self, degrees: isize)
  {
    let radians = (-degrees as f64).to_radians();
    let cos = radians.cos() as isize;
    let sin = radians.sin() as isize;

    let x = self.x * cos - self.y * sin;
    self.y = self.x * sin + self.y * cos;
    self.x = x;
  }

  fn translate(&mut self, x: isize, y: isize)
  {
    self.x += x;
    self.y += y;
  }

  fn follow_instruction(&mut self, instruction: Instruction)
  {
    match instruction {
      Instruction::Right(r) => self.rotate(r as isize),
      Instruction::Left(l) => self.rotate(-(l as isize)),
      Instruction::Forward(_) => (),
      Instruction::North(n) => self.translate(0, n as isize),
      Instruction::East(n) => self.translate(n as isize, 0),
      Instruction::South(n) => self.translate(0, -(n as isize)),
      Instruction::West(n) => self.translate(-(n as isize), 0)
    }
  }
}


fn parse_instruction(instruction: &str) -> Instruction
{
  let c = instruction.chars().nth(0).unwrap();
  let n = instruction.chars().skip(1).collect::<String>().parse::<usize>().unwrap_or(0);

  match c {
    'L' => Instruction::Left(n),
    'R' => Instruction::Right(n),
    'F' => Instruction::Forward(n),
    'N' => Instruction::North(n),
    'E' => Instruction::East(n),
    'S' => Instruction::South(n),
    'W' => Instruction::West(n),
    _ => panic!()
  }
}

fn problem1(path: &Path) -> usize
{
  let mut ship = Ship::new();
  for i in read_lines(path).unwrap().map(|s| parse_instruction(&s.unwrap()))
  {
    ship.follow_instruction(i);
  }

  (ship.x.abs() + ship.y.abs()) as usize
}

fn problem2(path: &Path) -> usize
{
  let mut waypoint = Waypoint::new(10, 1);
  let mut ship = Ship::new();
  for i in read_lines(path).unwrap().map(|s| parse_instruction(&s.unwrap()))
  {
    match i {
      Instruction::Forward(n) => ship.drive_to_waypoint(&waypoint, n),
      i => waypoint.follow_instruction(i)
    }
  }

  (ship.x.abs() + ship.y.abs()) as usize
}

fn main() {
  let path = Path::new(r"data/12-1.txt");
  println!("Result of problem 1: {}", problem1(path));
  println!("Result of problem 2: {}", problem2(path));
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn test_heading_to_delta() {
    assert_eq!(heading_to_delta(90), (1, 0));
    assert_eq!(heading_to_delta(450), (1, 0));
    assert_eq!(heading_to_delta(45), (1, 1));
    assert_eq!(heading_to_delta(-45), (-1, 1));
    assert_eq!(heading_to_delta(270), (-1, 0));
    assert_eq!(heading_to_delta(0), (0, 1));
    assert_eq!(heading_to_delta(360), (0, 1));
    assert_eq!(heading_to_delta(180), (0, -1));
    assert_eq!(heading_to_delta(-90), (-1, 0));
  }

  #[test]
  fn test_follow_instruction() {
    let mut ship = Ship::new();

    ship.follow_instruction(Instruction::East(10));
    assert_eq!(ship, Ship { x: 10, y: 0, heading: 90 });

    ship.follow_instruction(Instruction::Forward(10));
    assert_eq!(ship, Ship { x: 20, y: 0, heading: 90 });

    ship.follow_instruction(Instruction::Right(90));
    assert_eq!(ship, Ship { x: 20, y: 0, heading: 180 });

    ship.follow_instruction(Instruction::Forward(10));
    assert_eq!(ship, Ship { x: 20, y: -10, heading: 180 });

    ship.follow_instruction(Instruction::Left(90));
    assert_eq!(ship, Ship { x: 20, y: -10, heading: 90 });

    ship.follow_instruction(Instruction::North(5));
    assert_eq!(ship, Ship { x: 20, y: -5, heading: 90 });

    ship.follow_instruction(Instruction::West(5));
    assert_eq!(ship, Ship { x: 15, y: -5, heading: 90 });

    ship.follow_instruction(Instruction::South(5));
    assert_eq!(ship, Ship { x: 15, y: -10, heading: 90 });
  }

  #[test]
  fn test_parse_instruction()
  {
    assert_eq!(parse_instruction("L10"), Instruction::Left(10));
    assert_eq!(parse_instruction("R5"), Instruction::Right(5));
    assert_eq!(parse_instruction("F15"), Instruction::Forward(15));
    assert_eq!(parse_instruction("E4"), Instruction::East(4));
    assert_eq!(parse_instruction("W20"), Instruction::West(20));
    assert_eq!(parse_instruction("S25"), Instruction::South(25));
    assert_eq!(parse_instruction("N22"), Instruction::North(22));
  }

  #[test]
  fn test_waypoint_follow_instructions()
  {
    let mut waypoint = Waypoint::new(0, 0);

    waypoint.follow_instruction(Instruction::East(10));
    assert_eq!(waypoint, Waypoint::new(10, 0));
    waypoint.follow_instruction(Instruction::North(4));
    assert_eq!(waypoint, Waypoint::new(10, 4));
    waypoint.follow_instruction(Instruction::Right(90));
    assert_eq!(waypoint, Waypoint::new(4, -10));
    waypoint.follow_instruction(Instruction::Right(90));
    assert_eq!(waypoint, Waypoint::new(-10, -4));
    waypoint.follow_instruction(Instruction::Left(270));
    assert_eq!(waypoint, Waypoint::new(-4, 10));
    waypoint.follow_instruction(Instruction::West(8));
    assert_eq!(waypoint, Waypoint::new(-12, 10));
    waypoint.follow_instruction(Instruction::South(7));
    assert_eq!(waypoint, Waypoint::new(-12, 3));
  }

  #[test]
  fn test_move_ship_to_waypoint()
  {
    let mut waypoint = Waypoint::new(8, 2);
    let mut ship = Ship::new();

    ship.drive_to_waypoint(&waypoint, 1);
    assert_eq!(ship, Ship { x: 8, y: 2, heading: ship.heading });

    ship.drive_to_waypoint(&waypoint, 2);
    assert_eq!(ship, Ship { x: 24, y: 6, heading: ship.heading });

    waypoint.rotate(90); // (8, 2) -> (2, -8)
    ship.drive_to_waypoint(&waypoint, 10);
    assert_eq!(ship, Ship { x: 44, y: -74, heading: ship.heading });
  }
}
