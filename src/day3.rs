use std::path::Path;
use crate::utils::read_lines;

struct Forest {
  tiles: Vec<bool>,
  width: usize,
  height: usize,
}

impl Forest {
  fn detect_tree(c: char) -> bool {
    c != '.'
  }

  fn from_lines<'a, I>(lines: I) -> Self
    where I: Iterator<Item=String>
  {
    let mut width = 0;
    let tiles = lines.flat_map(|l| {
      width = l.len();
      l
        .chars()
        .map(Forest::detect_tree)
        .collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let height = tiles.len() / width;

    Forest { width, tiles, height }
  }

  fn is_tree(&self, x: usize, y: usize) -> bool {
    let i = y * self.width + (x % self.width);
    self.tiles[i]
  }
}

fn trees_for_slope(forest: &Forest, delta_x: usize, delta_y: usize) -> usize {
  (0..(forest.height / delta_y))
    .map(|i| (delta_x * i, delta_y * i))
    .filter(|(x, y)| forest.is_tree(*x, *y))
    .count()
}

pub fn problem1(path: &Path) -> usize
{
  let forest = Forest::from_lines(read_lines(path).unwrap().map(|l| l.unwrap()));
  trees_for_slope(&forest, 3, 1)
}

pub fn problem2(path: &Path) -> usize
{
  let forest = Forest::from_lines(read_lines(path).unwrap().map(|l| l.unwrap()));
  [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter()
    .map(|(x, y)| trees_for_slope(&forest, *x, *y))
    .product()
}
