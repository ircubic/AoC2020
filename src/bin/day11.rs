use std::path::Path;
use AoC2020::utils::read_lines;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Seating
{
  Floor,
  Unoccupied,
  Occupied,
}

impl Seating
{
  fn from_char(c: &char) -> Self
  {
    match c {
      '#' => Seating::Occupied,
      'L' => Seating::Unoccupied,
      _ => Seating::Floor
    }
  }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct SeatingArrangement
{
  seats: Vec<Vec<Seating>>
}

impl SeatingArrangement
{
  fn from_strings(strings: Vec<&str>) -> Option<Self>
  {
    let length = strings.get(0)?.len();
    if !strings.iter().all(|s| s.len() == length)
    {
      None
    } else {
      Some(SeatingArrangement {
        seats: strings.iter()
          .map(|s|
            s.chars()
              .map(|c| Seating::from_char(&c))
              .collect::<Vec<_>>())
          .collect::<Vec<_>>()
      })
    }
  }

  fn get_adjacent(&self, x: usize, y: usize) -> Vec<Vec<Option<Seating>>>
  {
    let left = (x as isize) - 1;
    let top = (y as isize) - 1;
    (top..top + 3)
      .map(|y|
        (left..left + 3)
          .map(|x|
            if x < 0 || y < 0 || x >= (self.width() as isize) || y >= (self.height() as isize) {
              None
            } else {
              Some(self.seats[y as usize][x as usize].clone())
            })
          .collect::<Vec<_>>())
      .collect::<Vec<_>>()
  }

  fn is_seat_in_los(&self, x: usize, y: usize, delta: &(isize, isize)) -> bool
  {
    let x_range = 0isize..(self.width() as isize);
    let y_range = 0isize..(self.height() as isize);
    let mut i_x = (x as isize) + delta.0;
    let mut i_y = (y as isize) + delta.1;

    while x_range.contains(&i_x) && y_range.contains(&i_y) {
      match self.get_seat(i_x as usize, i_y as usize) {
        Seating::Occupied => return true,
        Seating::Unoccupied => return false,
        _ => ()
      }

      i_x = i_x + delta.0;
      i_y = i_y + delta.1;
    }

    false
  }

  fn count_in_los(&self, x: usize, y: usize) -> usize
  {
    let deltas = vec![
      (-1, -1), (0, -1), (1, -1),
      (-1, 0), (1, 0),
      (-1, 1), (0, 1), (1, 1)
    ];

    deltas.into_iter()
      .filter(|d| self.is_seat_in_los(x, y, d))
      .count()
  }

  fn width(&self) -> usize
  {
    self.seats[0].len()
  }

  fn height(&self) -> usize
  {
    self.seats.len()
  }

  fn get_seat(&self, x: usize, y: usize) -> Seating
  {
    self.seats[y][x]
  }
}

fn next_seating_adjacent(seats: &SeatingArrangement, x: usize, y: usize) -> Seating
{
  let section = seats.get_adjacent(x, y);
  let num_occupied: usize = section
    .iter()
    .flatten()
    .enumerate()
    .map(|s| match s {
      (i, &Some(Seating::Occupied)) if i != 4 => 1,
      _ => 0
    })
    .sum();

  match section[1][1] {
    Some(Seating::Occupied) if num_occupied >= 4 => Seating::Unoccupied,
    Some(Seating::Unoccupied) if num_occupied == 0 => Seating::Occupied,
    Some(x) => x,
    _ => Seating::Floor
  }
}

fn next_seating_los(seats: &SeatingArrangement, x: usize, y: usize) -> Seating
{
  match (seats.get_seat(x, y), seats.count_in_los(x, y)) {
    (Seating::Unoccupied, 0) => Seating::Occupied,
    (Seating::Occupied, num) if num >= 5 => Seating::Unoccupied,
    (seat, _) => seat
  }
}

fn do_step<F>(seats: &SeatingArrangement, f: F) -> SeatingArrangement
  where F: Fn(&SeatingArrangement, usize, usize) -> Seating
{
  SeatingArrangement {
    seats: (0..seats.height()).map(|y|
      (0..seats.width())
        .map(|x| f(&seats, x, y))
        .collect::<Vec<_>>()
    ).collect::<Vec<_>>()
  }
}

fn run_to_completion<F>(seats: &SeatingArrangement, f: F) -> SeatingArrangement
  where F: Fn(&SeatingArrangement, usize, usize) -> Seating
{
  let mut copy = (*seats).clone();
  let mut iterations = 0;

  loop {
    if iterations > 200 {
      panic!("Too many iterations");
    }
    let next = do_step(&copy, &f);
    if next == copy {
      break;
    }
    copy = next;
    iterations += 1
  }

  copy
}

fn problem1(path: &Path) -> usize
{
  let strings = read_lines(path).unwrap().map(|s| s.unwrap()).collect::<Vec<String>>();
  let arrangement = SeatingArrangement::from_strings(strings.iter().map(AsRef::as_ref).collect()).unwrap();
  run_to_completion(&arrangement, next_seating_adjacent)
    .seats
    .iter()
    .flatten()
    .filter(|&s| *s == Seating::Occupied)
    .count()
}

fn problem2(path: &Path) -> usize
{
  let strings = read_lines(path).unwrap().map(|s| s.unwrap()).collect::<Vec<String>>();
  let arrangement = SeatingArrangement::from_strings(strings.iter().map(AsRef::as_ref).collect()).unwrap();
  run_to_completion(&arrangement, next_seating_los)
    .seats
    .iter()
    .flatten()
    .filter(|&s| *s == Seating::Occupied)
    .count()
}

fn main() {
  let path = Path::new(r"data/11-1.txt");
  println!("Result of problem 1: {}", problem1(path));
  println!("Result of problem 2: {}", problem2(path));
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn test_arrangement_from_strings()
  {
    let arrangement = SeatingArrangement::from_strings(
      vec![
        "L#.L",
        "....",
        "L##L",
        ".##."
      ]
    );

    assert_eq!(arrangement, Some(SeatingArrangement {
      seats: vec![
        vec![Seating::Unoccupied, Seating::Occupied, Seating::Floor, Seating::Unoccupied],
        vec![Seating::Floor, Seating::Floor, Seating::Floor, Seating::Floor],
        vec![Seating::Unoccupied, Seating::Occupied, Seating::Occupied, Seating::Unoccupied],
        vec![Seating::Floor, Seating::Occupied, Seating::Occupied, Seating::Floor]
      ]
    }));
  }

  #[test]
  fn test_get_adjacent()
  {
    let arrangement = SeatingArrangement::from_strings(
      vec![
        "L#.L",
        "....",
        "L##L",
        ".##."
      ]
    ).unwrap();

    assert_eq!(arrangement.get_adjacent(0, 0), vec![
      vec![None, None, None],
      vec![None, Some(Seating::Unoccupied), Some(Seating::Occupied)],
      vec![None, Some(Seating::Floor), Some(Seating::Floor)]
    ]);
    assert_eq!(arrangement.get_adjacent(3, 3), vec![
      vec![Some(Seating::Occupied), Some(Seating::Unoccupied), None],
      vec![Some(Seating::Occupied), Some(Seating::Floor), None],
      vec![None, None, None]
    ]);
    assert_eq!(arrangement.get_adjacent(1, 1), vec![
      vec![Some(Seating::Unoccupied), Some(Seating::Occupied), Some(Seating::Floor)],
      vec![Some(Seating::Floor), Some(Seating::Floor), Some(Seating::Floor)],
      vec![Some(Seating::Unoccupied), Some(Seating::Occupied), Some(Seating::Occupied)]
    ]);
  }

  #[test]
  fn test_count_in_los()
  {
    let arrangement = SeatingArrangement::from_strings(
      vec![
        "L#.L",
        "....",
        "L#LL",
        ".##."
      ]
    ).unwrap();

    assert_eq!(arrangement.count_in_los(1, 1), 2);
    assert_eq!(arrangement.count_in_los(3, 2), 2);
    assert_eq!(arrangement.count_in_los(1, 2), 3);
  }

  #[test]
  fn test_next_seating() {
    let arrangement = SeatingArrangement::from_strings(
      vec![
        "L#.L",
        "....",
        "L###",
        ".##L"
      ]
    ).unwrap();

    assert_eq!(next_seating_adjacent(&arrangement, 3, 0), Seating::Occupied);
    assert_eq!(next_seating_adjacent(&arrangement, 0, 0), Seating::Unoccupied);
    assert_eq!(next_seating_adjacent(&arrangement, 1, 1), Seating::Floor);
    assert_eq!(next_seating_adjacent(&arrangement, 2, 2), Seating::Unoccupied);
    assert_eq!(next_seating_adjacent(&arrangement, 1, 2), Seating::Occupied);
  }

  #[test]
  fn test_do_step_adjacent() {
    let arrangement = SeatingArrangement::from_strings(
      vec![
        "L.LL.LL.LL",
        "LLLLLLL.LL",
        "L.L.L..L..",
        "LLLL.LL.LL",
        "L.LL.LL.LL",
        "L.LLLLL.LL",
        "..L.L.....",
        "LLLLLLLLLL",
        "L.LLLLLL.L",
        "L.LLLLL.LL"
      ]
    ).unwrap();
    let next_arrangement = SeatingArrangement::from_strings(
      vec![
        "#.##.##.##",
        "#######.##",
        "#.#.#..#..",
        "####.##.##",
        "#.##.##.##",
        "#.#####.##",
        "..#.#.....",
        "##########",
        "#.######.#",
        "#.#####.##"
      ]
    ).unwrap();
    let next_arrangement2 = SeatingArrangement::from_strings(
      vec![
        "#.LL.L#.##",
        "#LLLLLL.L#",
        "L.L.L..L..",
        "#LLL.LL.L#",
        "#.LL.LL.LL",
        "#.LLLL#.##",
        "..L.L.....",
        "#LLLLLLLL#",
        "#.LLLLLL.L",
        "#.#LLLL.##"
      ]
    ).unwrap();

    assert_eq!(do_step(&arrangement, next_seating_adjacent), next_arrangement);
    assert_eq!(do_step(&next_arrangement, next_seating_adjacent), next_arrangement2);
  }

  #[test]
  fn test_do_step_los() {
    let arrangement = SeatingArrangement::from_strings(
      vec![
        "L.LL.LL.LL",
        "LLLLLLL.LL",
        "L.L.L..L..",
        "LLLL.LL.LL",
        "L.LL.LL.LL",
        "L.LLLLL.LL",
        "..L.L.....",
        "LLLLLLLLLL",
        "L.LLLLLL.L",
        "L.LLLLL.LL"
      ]
    ).unwrap();
    let next_arrangement = SeatingArrangement::from_strings(
      vec![
        "#.##.##.##",
        "#######.##",
        "#.#.#..#..",
        "####.##.##",
        "#.##.##.##",
        "#.#####.##",
        "..#.#.....",
        "##########",
        "#.######.#",
        "#.#####.##"
      ]
    ).unwrap();
    let next_arrangement2 = SeatingArrangement::from_strings(
      vec![
        "#.LL.LL.L#",
        "#LLLLLL.LL",
        "L.L.L..L..",
        "LLLL.LL.LL",
        "L.LL.LL.LL",
        "L.LLLLL.LL",
        "..L.L.....",
        "LLLLLLLLL#",
        "#.LLLLLL.L",
        "#.LLLLL.L#",
      ]
    ).unwrap();

    assert_eq!(do_step(&arrangement, next_seating_los), next_arrangement);
    assert_eq!(do_step(&next_arrangement, next_seating_los), next_arrangement2);
  }

  #[test]
  fn test_run_to_completion_adjacent()
  {
    let arrangement = SeatingArrangement::from_strings(
      vec![
        "L.LL.LL.LL",
        "LLLLLLL.LL",
        "L.L.L..L..",
        "LLLL.LL.LL",
        "L.LL.LL.LL",
        "L.LLLLL.LL",
        "..L.L.....",
        "LLLLLLLLLL",
        "L.LLLLLL.L",
        "L.LLLLL.LL"
      ]
    ).unwrap();

    let finished_arrangement = SeatingArrangement::from_strings(
      vec![
        "#.#L.L#.##",
        "#LLL#LL.L#",
        "L.#.L..#..",
        "#L##.##.L#",
        "#.#L.LL.LL",
        "#.#L#L#.##",
        "..L.L.....",
        "#L#L##L#L#",
        "#.LLLLLL.L",
        "#.#L#L#.##"
      ]
    ).unwrap();

    assert_eq!(run_to_completion(&arrangement, next_seating_adjacent), finished_arrangement);
  }

  #[test]
  fn test_run_to_completion_los()
  {
    let arrangement = SeatingArrangement::from_strings(
      vec![
        "L.LL.LL.LL",
        "LLLLLLL.LL",
        "L.L.L..L..",
        "LLLL.LL.LL",
        "L.LL.LL.LL",
        "L.LLLLL.LL",
        "..L.L.....",
        "LLLLLLLLLL",
        "L.LLLLLL.L",
        "L.LLLLL.LL"
      ]
    ).unwrap();

    let finished_arrangement = SeatingArrangement::from_strings(
      vec![
        "#.L#.L#.L#",
        "#LLLLLL.LL",
        "L.L.L..#..",
        "##L#.#L.L#",
        "L.L#.LL.L#",
        "#.LLLL#.LL",
        "..#.L.....",
        "LLL###LLL#",
        "#.LLLLL#.L",
        "#.L#LL#.L#"
      ]
    ).unwrap();

    assert_eq!(run_to_completion(&arrangement, next_seating_los), finished_arrangement);
  }
}