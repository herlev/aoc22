use aoc22::*;
use itertools::*;
use lending_iterator::prelude::*;
use std::collections::HashSet;

type Point = Point2<i32>;

fn parse_dir(s: &str) -> Direction {
  match s {
    "U" => Direction::Up,
    "D" => Direction::Down,
    "R" => Direction::Right,
    "L" => Direction::Left,
    _ => panic!(),
  }
}

fn parse_instructions(s: &str) -> Vec<(Direction, usize)> {
  s.lines()
    .map(|line| line.split(" ").collect_tuple().unwrap())
    .map(|(dir, c)| (parse_dir(dir), c.parse::<usize>().unwrap()))
    .collect::<Vec<_>>()
}

fn part1(input: &str) -> u32 {
  let instructions = parse_instructions(input);
  let mut head = Point::new(0, 0);
  let mut tail = Point::new(0, 0);
  let mut visited_by_t = HashSet::from([tail]);
  for i in instructions {
    for _ in 0..i.1 {
      head += i.0.to_point();
      let d = head - tail;
      if d.x.abs() == 2 || d.y.abs() == 2 {
        tail += d.signum();
      }
      visited_by_t.insert(tail);
    }
  }
  visited_by_t.len() as u32
}

fn print_grid(w: usize, h: usize, points: &Vec<Point>) {
  let mut grid = vec![vec!['.'; w]; h];
  for (i, p) in points.iter().enumerate().rev() {
    if 0 <= p.x && p.x < w as i32 && 0 <= p.y && p.y < h as i32 {
      grid[h - 1 - p.y as usize][p.x as usize] = i.to_string().chars().next().unwrap();
    }
  }
  println!("{}", grid.into_iter().map(|row| row.iter().join("")).join("\n"));
}

fn part2(input: &str) -> u32 {
  let instructions = parse_instructions(input);
  let mut visited_by_t = HashSet::from([Point::new(0, 0)]);
  let mut rope = [Point::new(0, 0); 10];
  for i in instructions {
    for _ in 0..i.1 {
      rope[0] += i.0.to_point();
      let mut iter = rope.windows_mut::<2>();
      while let Some([head, tail]) = iter.next() {
        let d = *head - *tail;
        if d.x.abs() == 2 || d.y.abs() == 2 {
          *tail += d.signum();
        }
      }
      visited_by_t.insert(rope[9]);
    }
  }
  visited_by_t.len() as u32
}

fn main() {
  let input = include_str!("../../inputs/09.txt");
  let tinput = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
  let tinput2 = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
  println!("part1: {}", part1(tinput));
  println!("part1: {}", part1(input));
  println!("part2: {}", part2(tinput2));
  println!("part2: {}", part2(tinput));
  println!("part2: {}", part2(input));
  assert_eq!(part1(input), 6354);
  assert_eq!(part2(input), 2651);
}
