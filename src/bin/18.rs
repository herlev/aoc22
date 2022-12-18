use itertools::*;
use std::{collections::VecDeque, ops::RangeInclusive};

use aoc22::*;

type Point = Point3<i32>;

fn parse(s: &str) -> Point {
  let mut i = s.split(',').map(|n| n.parse::<i32>().unwrap());
  Point {
    x: i.next().unwrap(),
    y: i.next().unwrap(),
    z: i.next().unwrap(),
  }
}

fn to_range(m: MinMaxResult<i32>) -> RangeInclusive<i32> {
  match m {
    MinMaxResult::MinMax(a, b) => RangeInclusive::new(a, b),
    _ => panic!(),
  }
}

fn part1(input: &str) -> u32 {
  let points = input.lines().map(parse).collect_vec();
  let x_range = to_range(points.iter().map(|p| p.x).minmax());
  let y_range = to_range(points.iter().map(|p| p.y).minmax());
  let z_range = to_range(points.iter().map(|p| p.z).minmax());
  let mut grid = Grid3::new(x_range, y_range, z_range);
  for p in &points {
    *grid.at_point(&p).unwrap() = true;
  }
  let mut c = 0;
  for p in points {
    c += 6;
    for p in p.neighbors().iter() {
      if let Some(v) = grid.at_point(p) {
        if *v {
          c -= 1;
        }
      }
    }
  }
  c
}

fn set_blob_to(g: &mut Grid3<u32>, p: Point, v: u32) {
  let mut q = VecDeque::new();
  q.push_back(p);
  while let Some(p) = q.pop_front() {
    if let Some(val) = g.at_point(&p) {
      if *val != 0 {
        continue;
      }
      *val = v;
      q.extend(p.neighbors().iter());
    }
  }
}

// https://en.wikipedia.org/wiki/Connected-component_labeling
fn segment(mut g: &mut Grid3<u32>) {
  let mut c = 2;
  for p in g.iter() {
    if *g.at_point(&p).unwrap() == 0 {
      set_blob_to(&mut g, p, c);
      c += 1;
    }
  }
}

fn part2(input: &str) -> u32 {
  let points = input.lines().map(parse).collect_vec();
  let x_range = to_range(points.iter().map(|p| p.x).minmax());
  let y_range = to_range(points.iter().map(|p| p.y).minmax());
  let z_range = to_range(points.iter().map(|p| p.z).minmax());
  let mut grid = Grid3::new(x_range, y_range, z_range);
  for p in &points {
    *grid.at_point(&p).unwrap() = 1u32;
  }
  // grid vals:
  // 0: not used
  // 1: lava droplet
  // 2: air
  // 3: air segment 1
  // 4: air segment 2
  // ...
  segment(&mut grid);
  let mut c = 0;
  for p in points {
    for p in p.neighbors().iter() {
      if let Some(v) = grid.at_point(p) {
        if *v == 2 {
          c += 1;
        }
      } else {
        // points outside grid is air as well
        c += 1
      }
    }
  }
  c
}

fn main() {
  let input = include_str!("../../inputs/18.txt");
  let tinput = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
  println!("part1: {}", part1(tinput));
  println!("part1: {}", part1(input));
  println!("part2: {}", part2(tinput));
  println!("part2: {}", part2(input));
  assert_eq!(part1(tinput), 64);
  assert_eq!(part1(input), 3564);
  assert_eq!(part2(tinput), 58);
  assert_eq!(part2(input), 2106);
}
