use std::path::Path;
use std::ops::RangeInclusive;
use std::collections::HashSet;
use std::cmp::{min, max};
use AoC2020::utils::read_lines;
use std::hash::Hash;

trait Voxel
{
  fn as_vec(&self) -> Vec<isize>;
  fn from_vec(v: &Vec<isize>) -> Self;
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct VoxelCoord3D
{
  x: isize,
  y: isize,
  z: isize,
}

impl VoxelCoord3D {
  fn new() -> Self {
    VoxelCoord3D { x: 0, y: 0, z: 0 }
  }

  fn with_coords(x: isize, y: isize, z: isize) -> Self
  {
    VoxelCoord3D { x, y, z }
  }
}

impl Voxel for VoxelCoord3D {
  fn as_vec(&self) -> Vec<isize>
  {
    vec![self.x, self.y, self.z]
  }

  fn from_vec(v: &Vec<isize>) -> Self
  {
    VoxelCoord3D { x: v[0], y: v[1], z: v[2] }
  }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct VoxelCoord4D
{
  x: isize,
  y: isize,
  z: isize,
  w: isize,
}

impl VoxelCoord4D {
  fn new() -> Self {
    VoxelCoord4D { x: 0, y: 0, z: 0, w: 0 }
  }

  fn with_coords(x: isize, y: isize, z: isize, w: isize) -> Self
  {
    VoxelCoord4D { x, y, z, w }
  }
}

impl Voxel for VoxelCoord4D {
  fn as_vec(&self) -> Vec<isize>
  {
    vec![self.x, self.y, self.z, self.w]
  }

  fn from_vec(v: &Vec<isize>) -> Self
  {
    VoxelCoord4D { x: v[0], y: v[1], z: v[2], w: v[3] }
  }
}

struct RangeVec
{
  ranges: Vec<RangeInclusive<isize>>
}

impl RangeVec
{
  fn from_min_max(min: &Vec<isize>, max: &Vec<isize>) -> Self
  {
    RangeVec {
      ranges: min.iter()
        .zip(max.iter())
        .map(|(&min, &max)| RangeInclusive::new(min, max))
        .collect()
    }
  }

  fn delve(r: &Vec<RangeInclusive<isize>>) -> Vec<Vec<isize>> {
    if r.len() == 1 {
      r[0].clone().map(|i| vec![i]).collect()
    } else {
      let next_vec = r[1..].iter().cloned().collect::<Vec<_>>();
      let mut return_vec = Vec::<Vec<isize>>::new();
      for i in r[0].clone() {
        return_vec.extend(
          RangeVec::delve(&next_vec).iter()
            .map(|v| {
              let mut vv = Vec::<isize>::new();
              vv.push(i);
              vv.extend(v);
              vv
            })
            .collect::<Vec<_>>()
        );
      }
      return_vec
    }
  }

  fn get_all_included_coords(&self) -> Vec<Vec<isize>>
  {
    RangeVec::delve(&self.ranges)
  }
}

struct VoxelSpace<V>
  where V: Voxel + Hash + Eq + Copy
{
  voxels: HashSet<V>
}

impl<V> VoxelSpace<V>
  where V: Voxel + Hash + Eq + Copy
{
  fn count_active_neighbours_vec(&self, v: Vec<isize>) -> usize
  {
    let mut count = 0;
    let range_vec = RangeVec::from_min_max(&v.iter().map(|&i| i - 1).collect(), &v.iter().map(|&i| i + 1).collect());
    for c in range_vec.get_all_included_coords().into_iter()
    {
      let coord = V::from_vec(&c);
      if c != v && self.voxels.contains(&coord) {
        count += 1;
      }
    }
    count
  }

  fn get_affected_bounds(&self) -> RangeVec
  {
    let mut v_it = self.voxels.iter();
    let first = v_it.by_ref().next().unwrap().as_vec();
    let (min, max) = v_it
      .map(|v| v.as_vec())
      .fold((first.clone(), first),
            |acc, v| (
              acc.0.iter().zip(v.iter()).map(|(l, r)| *min(l, r)).collect::<Vec<isize>>(),
              acc.1.iter().zip(v.iter()).map(|(l, r)| *max(l, r)).collect::<Vec<isize>>(),
            ),
      );

    // min.iter()
    //   .zip(max.iter())
    //   .map(|(l, r)| RangeInclusive::new(l - 1, r + 1))
    //   .collect()
    RangeVec::from_min_max(&min.iter().map(|&i| i - 1).collect(), &max.iter().map(|&i| i + 1).collect())
  }
}

impl<> VoxelSpace<VoxelCoord3D> {
  fn new() -> Self
  {
    VoxelSpace { voxels: HashSet::new() }
  }

  fn set_active(&mut self, x: isize, y: isize, z: isize)
  {
    self.voxels.insert(VoxelCoord3D::with_coords(x, y, z));
  }

  fn set_inactive(&mut self, x: isize, y: isize, z: isize)
  {
    self.voxels.remove(&VoxelCoord3D::with_coords(x, y, z));
  }

  fn is_active(&self, x: isize, y: isize, z: isize) -> bool
  {
    self.voxels.contains(&VoxelCoord3D::with_coords(x, y, z))
  }

  fn count_active_neighbours(&self, x: isize, y: isize, z: isize) -> usize
  {
    self.count_active_neighbours_vec(vec![x, y, z])
  }

  // fn get_affected_bounds(&self) -> (RangeInclusive<isize>, RangeInclusive<isize>, RangeInclusive<isize>)
  // {
  //   let r = self.get_affected_bounds_vec();
  //   (r[0].clone(), r[1].clone(), r[2].clone())
  //   // let (min, max) = self.voxels.iter()
  //   //   .fold((VoxelCoord::new(), VoxelCoord::new()), |(l, r), v|
  //   //     (
  //   //       VoxelCoord::with_coords(min(l.x, v.x), min(l.y, v.y), min(l.z, v.z)),
  //   //       VoxelCoord::with_coords(max(r.x, v.x), max(r.y, v.y), max(r.z, v.z))
  //   //     ),
  //   //   );
  //   // (
  //   //   RangeInclusive::new(min.x - 1, max.x + 1),
  //   //   RangeInclusive::new(min.y - 1, max.y + 1),
  //   //   RangeInclusive::new(min.z - 1, max.z + 1),
  //   // )
  // }
}

impl<> VoxelSpace<VoxelCoord4D> {
  fn new() -> Self
  {
    VoxelSpace { voxels: HashSet::new() }
  }

  fn set_active(&mut self, x: isize, y: isize, z: isize, w: isize)
  {
    self.voxels.insert(VoxelCoord4D::with_coords(x, y, z, w));
  }

  fn set_inactive(&mut self, x: isize, y: isize, z: isize, w: isize)
  {
    self.voxels.remove(&VoxelCoord4D::with_coords(x, y, z, w));
  }

  fn is_active(&self, x: isize, y: isize, z: isize, w: isize) -> bool
  {
    self.voxels.contains(&VoxelCoord4D::with_coords(x, y, z, w))
  }

  fn count_active_neighbours(&self, x: isize, y: isize, z: isize, w: isize) -> usize
  {
    self.count_active_neighbours_vec(vec![x, y, z, w])
  }

  // fn get_affected_bounds(&self) -> (RangeInclusive<isize>, RangeInclusive<isize>, RangeInclusive<isize>)
  // {
  //   let r = self.get_affected_bounds_vec();
  //   (r[0].clone(), r[1].clone(), r[2].clone())
  //   // let (min, max) = self.voxels.iter()
  //   //   .fold((VoxelCoord::new(), VoxelCoord::new()), |(l, r), v|
  //   //     (
  //   //       VoxelCoord::with_coords(min(l.x, v.x), min(l.y, v.y), min(l.z, v.z)),
  //   //       VoxelCoord::with_coords(max(r.x, v.x), max(r.y, v.y), max(r.z, v.z))
  //   //     ),
  //   //   );
  //   // (
  //   //   RangeInclusive::new(min.x - 1, max.x + 1),
  //   //   RangeInclusive::new(min.y - 1, max.y + 1),
  //   //   RangeInclusive::new(min.z - 1, max.z + 1),
  //   // )
  // }
}

fn do_step(space: &VoxelSpace<VoxelCoord3D>) -> VoxelSpace<VoxelCoord3D>
{
  let r = space.get_affected_bounds();
  let mut new_space = VoxelSpace::<VoxelCoord3D>::new();

  for i in r.get_all_included_coords()
  {
    let (x, y, z) = (i[0], i[1], i[2]);
    match (space.is_active(x, y, z), space.count_active_neighbours(x, y, z))
    {
      (true, 2) | (true, 3) | (false, 3) => new_space.set_active(x, y, z),
      _ => ()
    }
  }

  new_space
}

fn do_step_4d(space: &VoxelSpace<VoxelCoord4D>) -> VoxelSpace<VoxelCoord4D>
{
  let r = space.get_affected_bounds();
  let mut new_space = VoxelSpace::<VoxelCoord4D>::new();

  for i in r.get_all_included_coords()
  {
    let (x, y, z,w) = (i[0], i[1], i[2], i[3]);
    match (space.is_active(x, y, z, w), space.count_active_neighbours(x, y, z, w))
    {
      (true, 2) | (true, 3) | (false, 3) => new_space.set_active(x, y, z, w),
      _ => ()
    }
  }

  new_space
}

fn file_to_space(path: &Path) -> VoxelSpace<VoxelCoord3D>
{
  let mut space = VoxelSpace::<VoxelCoord3D>::new();
  for (y, l) in read_lines(&path).unwrap().enumerate().map(|(i, s)| (i, s.unwrap()))
  {
    for x in l.chars().enumerate().filter_map(|(i, c)| if c == '#' { Some(i) } else { None })
    {
      space.set_active(x as isize, y as isize, 0);
    }
  }
  space
}

fn file_to_space_4d(path: &Path) -> VoxelSpace<VoxelCoord4D>
{
  let mut space = VoxelSpace::<VoxelCoord4D>::new();
  for (y, l) in read_lines(&path).unwrap().enumerate().map(|(i, s)| (i, s.unwrap()))
  {
    for x in l.chars().enumerate().filter_map(|(i, c)| if c == '#' { Some(i) } else { None })
    {
      space.set_active(x as isize, y as isize, 0, 0);
    }
  }
  space
}

fn problem1(path: &Path) -> usize
{
  let mut space = file_to_space(&path);
  for _ in 0..6 {
    space = do_step(&space);
  }

  space.voxels.len()
}

fn problem2(path: &Path) -> usize
{
  let mut space = file_to_space_4d(&path);
  for _ in 0..6 {
    space = do_step_4d(&space);
  }

  space.voxels.len()
}

fn main() {
  let path = Path::new(r"data/17-1.txt");
  println!("Result of problem 1: {}", problem1(path));
  println!("Result of problem 2: {}", problem2(path));
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn test_range_vec() {
    let r = RangeVec::from_min_max(&vec![0, 0, 0], &vec![2, 2, 2]);

    assert_eq!(r.get_all_included_coords(), vec![
      vec![0, 0, 0], vec![0, 0, 1], vec![0, 0, 2],
      vec![0, 1, 0], vec![0, 1, 1], vec![0, 1, 2],
      vec![0, 2, 0], vec![0, 2, 1], vec![0, 2, 2],
      vec![1, 0, 0], vec![1, 0, 1], vec![1, 0, 2],
      vec![1, 1, 0], vec![1, 1, 1], vec![1, 1, 2],
      vec![1, 2, 0], vec![1, 2, 1], vec![1, 2, 2],
      vec![2, 0, 0], vec![2, 0, 1], vec![2, 0, 2],
      vec![2, 1, 0], vec![2, 1, 1], vec![2, 1, 2],
      vec![2, 2, 0], vec![2, 2, 1], vec![2, 2, 2],
    ])
  }

  #[test]
  fn test_set_space_active() {
    let mut space = VoxelSpace::<VoxelCoord3D>::new();

    space.set_active(1, 1, 0);
    space.set_active(-1, 1, -2);

    assert_eq!(space.voxels.len(), 2);
    assert!(space.is_active(1, 1, 0));
    assert!(space.is_active(-1, 1, -2));

    space.set_active(-1, 1, -2);
    assert_eq!(space.voxels.len(), 2);
    assert!(space.is_active(1, 1, 0));
    assert!(space.is_active(-1, 1, -2));
  }

  #[test]
  fn test_set_space_inactive() {
    let mut space = VoxelSpace::<VoxelCoord3D>::new();

    space.set_active(1, 1, 0);
    space.set_active(-1, 1, -2);

    space.set_inactive(-1, 1, -2);
    space.set_inactive(-1, 2, -1);

    assert_eq!(space.voxels.len(), 1);
    assert!(space.is_active(1, 1, 0));
  }

  #[test]
  fn test_count_active_neighbours() {
    let mut space = VoxelSpace::<VoxelCoord3D>::new();
    space.set_active(0, -1, 0);
    space.set_active(1, 0, 0);
    space.set_active(-1, -1, 0);
    space.set_active(0, -1, 1);
    space.set_active(0, -1, -1);
    space.set_active(1, 2, 3);

    assert_eq!(space.count_active_neighbours(0, 0, 0), 5);
    assert_eq!(space.count_active_neighbours(-1, -1, 0), 3);
    assert_eq!(space.count_active_neighbours(1, 2, 3), 0);
  }

  #[test]
  fn test_get_affected_bounds()
  {
    let mut space = VoxelSpace::<VoxelCoord3D>::new();
    space.set_active(0, -1, 0);
    space.set_active(1, 0, 0);
    space.set_active(-1, -1, 0);
    space.set_active(0, -1, 1);
    space.set_active(0, -1, -1);

    let r = space.get_affected_bounds();
    assert_eq!(r.ranges[0], RangeInclusive::new(-2, 2));
    assert_eq!(r.ranges[1], RangeInclusive::new(-2, 1));
    assert_eq!(r.ranges[2], RangeInclusive::new(-2, 2));
  }

  #[test]
  fn test_do_step()
  {
    // Glider setup
    let mut space = VoxelSpace::<VoxelCoord3D>::new();
    space.set_active(0, -1, 0);
    space.set_active(1, 0, 0);
    space.set_active(-1, 1, 0);
    space.set_active(0, 1, 0);
    space.set_active(1, 1, 0);

    space = do_step(&space);
    assert!(space.is_active(-1, 0, -1));
    assert!(space.is_active(1, 1, -1));
    assert!(space.is_active(0, 2, -1));

    assert!(space.is_active(-1, 0, 0));
    assert!(space.is_active(1, 0, 0));
    assert!(space.is_active(0, 1, 0));
    assert!(space.is_active(1, 1, 0));
    assert!(space.is_active(0, 2, 0));

    assert!(space.is_active(-1, 0, 1));
    assert!(space.is_active(1, 1, 1));
    assert!(space.is_active(0, 2, 1));

    assert_eq!(space.voxels.len(), 11);

    for _ in 0..5
    {
      space = do_step(&space);
    }

    assert_eq!(space.voxels.len(), 112);
  }

  #[test]
  fn test_do_step_4d()
  {
    // Glider setup
    let mut space = VoxelSpace::<VoxelCoord4D>::new();
    space.set_active(0, -1, 0, 0);
    space.set_active(1, 0, 0, 0);
    space.set_active(-1, 1, 0, 0);
    space.set_active(0, 1, 0, 0);
    space.set_active(1, 1, 0, 0);

    space = do_step_4d(&space);
    assert!(space.is_active(-1, 0, -1, -1));
    assert!(space.is_active(1, 1, -1, -1));
    assert!(space.is_active(0, 2, -1, -1));

    assert!(space.is_active(-1, 0, -1, 0));
    assert!(space.is_active(1, 1, -1, 0));
    assert!(space.is_active(0, 2, -1, 0));

    assert!(space.is_active(-1, 0, -1, 1));
    assert!(space.is_active(1, 1, -1, 1));
    assert!(space.is_active(0, 2, -1, 1));

    assert!(space.is_active(-1, 0, 0, -1));
    assert!(space.is_active(1, 1, 0, -1));
    assert!(space.is_active(0, 2, 0, -1));

    assert!(space.is_active(-1, 0, 0,  0));
    assert!(space.is_active(1, 0, 0, 0));
    assert!(space.is_active(0, 1, 0, 0));
    assert!(space.is_active(1, 1, 0, 0));
    assert!(space.is_active(0, 2, 0, 0));

    assert!(space.is_active(-1, 0, 1, 1));
    assert!(space.is_active(1, 1, 1, 1));
    assert!(space.is_active(0, 2, 1, 1));

    assert!(space.is_active(-1, 0, 1, -1));
    assert!(space.is_active(1, 1, 1, -1));
    assert!(space.is_active(0, 2, 1, -1));

    assert!(space.is_active(-1, 0, 1, 0));
    assert!(space.is_active(1, 1, 1, 0));
    assert!(space.is_active(0, 2, 1, 0));

    assert!(space.is_active(-1, 0, 1, 1));
    assert!(space.is_active(1, 1, 1, 1));
    assert!(space.is_active(0, 2, 1, 1));

    assert_eq!(space.voxels.len(), 29);

    for _ in 0..5
    {
      space = do_step_4d(&space);
    }

    assert_eq!(space.voxels.len(), 848);
  }
}