use std::path::Path;
use std::collections::{HashMap, HashSet};
use AoC2020::utils::read_lines;

/// This is in relation to the node given by "left".
///
/// An edge pointing from right to left has the direction IN
#[derive(Debug, PartialEq, Clone)]
enum BagDirection {
  OUT,
  IN,
  BOTH,
}

#[derive(Clone, PartialEq, Debug)]
struct BagEdge
{
  left: String,
  right: String,
  weight: usize,
  direction: BagDirection,
}

impl BagEdge {
  fn get_flipped(&self) -> Self
  {
    BagEdge {
      left: self.right.clone(),
      right: self.left.clone(),
      weight: self.weight,
      direction: match self.direction {
        BagDirection::OUT => BagDirection::IN,
        BagDirection::IN => BagDirection::OUT,
        _ => BagDirection::BOTH
      },
    }
  }
}

struct BagDag {
  nodes: HashMap<String, Vec<usize>>,
  edges: Vec<BagEdge>,
}

/// DAG implementation that only barely works for this specific problem
///
/// Uses an adjacency list of indices into a vector of edge objects.
impl BagDag {
  fn new() -> Self {
    BagDag { nodes: HashMap::new(), edges: vec![] }
  }

  fn insert_node(&mut self, node: &str)
  {
    if !self.nodes.contains_key(node) {
      self.nodes.insert(node.to_string(), vec![]);
    }
  }

  fn insert_edge(&mut self, edge: BagEdge)
  {
    self.insert_node(&edge.left);
    self.insert_node(&edge.right);

    match self.get_edge_index(&edge.left, &edge.right) {
      Some(e) => self.edges[e] = edge,
      None => {
        let idx = self.edges.len();
        self.nodes.get_mut(&edge.left).unwrap().push(idx);
        self.nodes.get_mut(&edge.right).unwrap().push(idx);
        self.edges.push(edge);
      }
    }
  }

  fn get_edge_indices<'a>(&'a self, node: &'a str) -> impl Iterator<Item=usize> + 'a
  {
    let edges = self.nodes.get(node).unwrap();
    edges.iter()
      .filter(move |&e| {
        let edge = &self.edges[*e];
        edge.left == node || edge.right == node
      })
      .map(|x| *x)
  }

  fn get_edge_index(&self, left: &str, right: &str) -> Option<usize>
  {
    self.get_edge_indices(&left)
      .find(|e| self.edges[*e].right == right)
  }

  fn get_edges<'a>(&'a self, node: &'a str) -> impl Iterator<Item=BagEdge> + 'a
  {
    self.get_edge_indices(node)
      .map(move |i| {
        let edge = &self.edges[i];
        if &edge.left == node {
          edge.clone()
        } else {
          edge.get_flipped()
        }
      })
  }

  fn get_edge(&self, left: &str, right: &str) -> Option<BagEdge>
  {
    self.get_edges(&left)
      .find(|e| e.right == right)
  }

  fn get_nodes(&self) -> impl Iterator<Item=&str>
  {
    self.nodes.keys().map(|str| str.as_str())
  }
}

fn parse_edges(line: &str) -> Vec<BagEdge>
{
  if !line.contains(&"no other bags") {
    let mut parts = line.split(&"contain");
    let container = parts.next().unwrap().trim();
    let container = container[0..(container.find(" bag").unwrap())].to_string();
    parts
      .next().unwrap()
      .trim()
      .split(",")
      .map(|s| {
        let s = s.trim();
        let num = s[0..1].parse::<usize>().unwrap();
        let location = s.find(" bag").unwrap();
        BagEdge { left: container.clone(), right: s[2..location].to_string(), weight: num, direction: BagDirection::OUT }
      })
      .collect()
  } else {
    vec![]
  }
}

fn count_leaves_up(start: &str, dag: &BagDag) -> usize
{
  let mut visited = HashSet::<String>::new();
  let mut next = dag.get_edges(start).collect::<Vec<_>>();
  let mut count = 0usize;
  let mut i = 0;

  visited.insert(start.to_string());

  while i < next.len() {
    let e = next[i].clone();
    i += 1;

    if e.direction == BagDirection::IN && !visited.contains(&e.right) {
      count += 1;
      visited.insert(e.right.to_string());
      next.extend(dag.get_edges(&e.right));
    }
  }

  count
}

fn count_contained_inner(start: &str, dag: &BagDag, value: usize) -> usize
{
  let res = dag.get_edges(start)
    .filter(|e| e.direction == BagDirection::OUT)
    .map(|e| e.weight * count_contained_inner(&e.right, dag, 1))
    .collect::<Vec<usize>>();
  value + res.iter().sum::<usize>()
}

fn count_contained(start: &str, dag: &BagDag) -> usize
{
  count_contained_inner(start, dag, 0)
}

/// Breadth-first search in the upward (inwards edge) direction.
///
/// Count the amount of visited nodes
fn problem1(path: &Path) -> usize
{
  let mut dag = BagDag::new();
  for e in read_lines(path).unwrap().flat_map(|x| parse_edges(&x.unwrap()))
  {
    dag.insert_edge(e);
  }
  count_leaves_up("shiny gold", &dag)
}

/// Depth-first search in the downward (outwards edge) direction.
///
/// Count the product of child weights
fn problem2(path: &Path) -> usize
{
  let mut dag = BagDag::new();
  for e in read_lines(path).unwrap().flat_map(|x| parse_edges(&x.unwrap()))
  {
    dag.insert_edge(e);
  }
  count_contained("shiny gold", &dag)
}

fn main() {
  let path = Path::new(r"data/7-1.txt");
  println!("Result of problem 1: {}", problem1(path));
  println!("Result of problem 2: {}", problem2(path));
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn test_insert_node()
  {
    let mut dag = BagDag::new();

    dag.insert_node("test");
    dag.insert_node("test2");

    let nodes: Vec<&str> = dag.get_nodes().collect();
    assert!(nodes.contains(&"test"));
    assert!(nodes.contains(&"test2"))
  }

  #[test]
  fn test_insert_edge()
  {
    let mut dag = BagDag::new();

    dag.insert_edge(BagEdge { left: "test".to_string(), right: "test2".to_string(), weight: 1, direction: BagDirection::OUT });
    dag.insert_edge(BagEdge { left: "test2".to_string(), right: "test3".to_string(), weight: 2, direction: BagDirection::IN });

    let nodes: Vec<&str> = dag.get_nodes().collect();
    assert!(nodes.contains(&"test"));
    assert!(nodes.contains(&"test2"));
    assert!(nodes.contains(&"test3"));

    let edge = &dag.get_edges(&"test").collect::<Vec<_>>()[0];
    assert_eq!(edge.left, "test");
    assert_eq!(edge.right, "test2");
    assert_eq!(edge.weight, 1);
    assert_eq!(edge.direction, BagDirection::OUT);

    let edge = &dag.get_edges(&"test3").collect::<Vec<_>>()[0];
    assert_eq!(edge.left, "test3");
    assert_eq!(edge.right, "test2");
    assert_eq!(edge.weight, 2);
    assert_eq!(edge.direction, BagDirection::OUT);

    let edges = dag.get_edges(&"test2").collect::<Vec<_>>();
    assert_eq!(edges.len(), 2);
    let edge = &edges[0];
    assert_eq!(edge.left, "test2");
    assert_eq!(edge.right, "test");
    assert_eq!(edge.weight, 1);
    assert_eq!(edge.direction, BagDirection::IN);
    let edge = &edges[1];
    assert_eq!(edge.left, "test2");
    assert_eq!(edge.right, "test3");
    assert_eq!(edge.weight, 2);
    assert_eq!(edge.direction, BagDirection::IN);
  }

  #[test]
  fn test_parse_edges()
  {
    let edges0 = parse_edges(&"faded blue bags contain no other bags.");
    let edges1 = parse_edges(&"bright white bags contain 1 shiny gold bag.");
    let edges2 = parse_edges(&"light red bags contain 1 bright white bag, 2 muted yellow bags.");

    assert_eq!(edges0.len(), 0);
    assert_eq!(edges1.len(), 1);
    assert_eq!(edges2.len(), 2);

    assert_eq!(edges1[0], BagEdge { left: "bright white".to_string(), right: "shiny gold".to_string(), weight: 1, direction: BagDirection::OUT });

    assert_eq!(edges2[0], BagEdge { left: "light red".to_string(), right: "bright white".to_string(), weight: 1, direction: BagDirection::OUT });
    assert_eq!(edges2[1], BagEdge { left: "light red".to_string(), right: "muted yellow".to_string(), weight: 2, direction: BagDirection::OUT });
  }

  #[test]
  fn test_count_leaves_up()
  {
    let mut dag = BagDag::new();
    let text = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    for e in text.split("\n").flat_map(|l| parse_edges(l)) {
      dag.insert_edge(e)
    }

    let nodes = dag.nodes.keys().collect::<Vec<_>>();

    assert_eq!(count_leaves_up("shiny gold", &dag), 4);
  }

  #[test]
  fn test_count_contained()
  {
    let mut dag = BagDag::new();
    let text = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    for e in text.split("\n").flat_map(|l| parse_edges(l)) {
      dag.insert_edge(e)
    }

    let nodes = dag.nodes.keys().collect::<Vec<_>>();

    assert_eq!(count_contained("shiny gold", &dag), 32);

    let mut dag = BagDag::new();
    let text = r"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
    for e in text.split("\n").flat_map(|l| parse_edges(l)) {
      dag.insert_edge(e)
    }

    let nodes = dag.nodes.keys().collect::<Vec<_>>();

    assert_eq!(count_contained("shiny gold", &dag), 126);
  }
}