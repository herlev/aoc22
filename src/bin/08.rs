use itertools::*;
use std::collections::{HashMap, HashSet};

fn part1(input: &str) -> u32 {
  let v: Vec<Vec<_>> = input
    .lines()
    .map(|line| line.split("").filter_map(|s| s.parse::<u8>().ok()).collect())
    .collect();
  let mut visible = HashSet::new();
  let h = v.len();
  let w = v[0].len();

  for y in 1..(h - 1) {
    let mut max_height = v[y][0];
    for x in 1..(w - 1) {
      if v[y][x] > max_height {
        visible.insert((x, y, v[y][x]));
      }
      max_height = max([max_height, v[y][x]]).unwrap()
    }
  }
  for y in 1..(h - 1) {
    let mut max_height = v[y][w - 1];
    for x in (1..(w - 1)).rev() {
      if v[y][x] > max_height {
        visible.insert((x, y, v[y][x]));
      }
      max_height = max([max_height, v[y][x]]).unwrap()
    }
  }
  for x in 1..(w - 1) {
    let mut max_height = v[0][x];
    for y in 1..(h - 1) {
      if v[y][x] > max_height {
        visible.insert((x, y, v[y][x]));
      }
      max_height = max([max_height, v[y][x]]).unwrap()
    }
  }
  for x in 1..(w - 1) {
    let mut max_height = v[h - 1][x];
    for y in (1..(h - 1)).rev() {
      if v[y][x] > max_height {
        visible.insert((x, y, v[y][x]));
      }
      max_height = max([max_height, v[y][x]]).unwrap()
    }
  }
  (visible.len() + 2 * w + 2 * (h - 2)) as u32
}

fn calc_scenic_score(vx: usize, vy: usize, grid: &Vec<Vec<u8>>) -> u32 {
  let h = grid.len();
  let w = grid[0].len();
  let get_tree_count = |tree_heights: &mut dyn Iterator<Item = u8>| {
    tree_heights
      .enumerate()
      .skip(1)
      .find_or_last(|&(_, v)| v >= grid[vy][vx])
      .unwrap()
      .0
  };
  // right // left // up // down
  (get_tree_count(&mut (vx..w).map(|x| grid[vy][x]))
    * get_tree_count(&mut (0..=vx).rev().map(|x| grid[vy][x]))
    * get_tree_count(&mut (vy..h).map(|y| grid[y][vx]))
    * get_tree_count(&mut (0..=vy).rev().map(|y| grid[y][vx]))) as u32
}

fn part2(input: &str) -> u32 {
  let v: Vec<Vec<_>> = input
    .lines()
    .map(|line| line.split("").filter_map(|s| s.parse::<u8>().ok()).collect())
    .collect();
  let h = v.len();
  let w = v[0].len();
  iproduct!(1..(w - 1), 1..(h - 1))
    .map(|(x, y)| calc_scenic_score(x, y, &v))
    .max()
    .unwrap()
}

fn main() {
  let input = include_str!("../../inputs/08.txt");
  let tinput = "30373
25512
65332
33549
35390";
  println!("part1: {}", part1(tinput));
  println!("part1: {}", part1(input));
  println!("part2: {}", part2(tinput));
  println!("part2: {}", part2(input));
  assert_eq!(part1(tinput), 21);
  assert_eq!(part1(input), 1647);
  assert_eq!(part2(tinput), 8);
  assert_eq!(part2(input), 392080);
}
