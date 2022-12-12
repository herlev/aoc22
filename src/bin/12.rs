use aoc22::*;
use itertools::*;
use std::collections::{HashSet, VecDeque};

type Point = Point2<i32>;

fn get_elevation(c: char) -> u32 {
  match c {
    'a'..='z' => c as u32 - 'a' as u32,
    'S' => 0,
    'E' => 'z' as u32 - 'a' as u32,
    _ => panic!(),
  }
}

enum Part {
  P1,
  P2,
}

fn solve(input: &str, part: Part) -> u32 {
  let grid = input
    .lines()
    .map(|line| line.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();
  let h = grid.len();
  let w = grid[0].len();
  let mut starts = Vec::new();
  for (y, x) in iproduct!(0..h, 0..w) {
    match part {
      Part::P1 => {
        if grid[y][x] == 'S' {
          starts.push(Point::new(x as i32, y as i32));
        }
      }
      Part::P2 => {
        if get_elevation(grid[y][x]) == 0 {
          starts.push(Point::new(x as i32, y as i32));
        }
      }
    }
  }

  let mut count = None;
  let mut q = VecDeque::from_iter(starts.iter().map(|&p| (p, 0)));
  let mut visited: HashSet<_> = HashSet::from_iter(starts.into_iter());
  'outer: while !q.is_empty() {
    let (p, c) = q.pop_front().unwrap();
    let neighbors = p.neighbors_grid(w, h);
    for n in neighbors.filter(|n| {
      get_elevation(grid[p.y as usize][p.x as usize]) + 1 >= get_elevation(grid[n.y as usize][n.x as usize])
    }) {
      if grid[n.y as usize][n.x as usize] == 'E' {
        count = Some(c + 1);
        break 'outer;
      }
      if !visited.contains(&n) {
        visited.insert(n);
        q.push_back((n, c + 1));
      }
    }
  }
  count.unwrap()
}

fn part1(input: &str) -> u32 {
  solve(input, Part::P1)
}

fn part2(input: &str) -> u32 {
  solve(input, Part::P2)
}

fn main() {
  let input = include_str!("../../inputs/12.txt");
  let tinput = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
  println!("part1: {}", part1(tinput));
  println!("part1: {}", part1(input));
  println!("part2: {}", part2(tinput));
  println!("part2: {}", part2(input));
  assert_eq!(part1(tinput), 31);
  assert_eq!(part1(input), 352);
  assert_eq!(part2(tinput), 29);
  assert_eq!(part2(input), 345);
}
