use itertools::*;

struct Range {
  start: i32,
  end: i32,
}

impl Range {
  fn FromStr(s: &str) -> Range {
    let mut a = s.split("-");
    Range {
      start: a.next().unwrap().parse().unwrap(),
      end: a.next().unwrap().parse().unwrap(),
    }
  }
  fn is_fully_contained_in(&self, other: &Range) -> bool {
    self.end <= other.end && self.start >= other.start
  }
  fn overlaps(&self, other: &Range) -> bool {
    (other.end <= self.end && other.end >= self.start) || (self.end <= other.end && self.end >= other.start)
  }
}

fn part1(input: &str) -> u32 {
  input
    .lines()
    .map(|line| line.split(",").map(Range::FromStr).collect::<Vec<_>>())
    .filter(|v| v[0].is_fully_contained_in(&v[1]) || v[1].is_fully_contained_in(&v[0]))
    .count() as u32
}

fn part2(input: &str) -> u32 {
  input
    .lines()
    .map(|line| line.split(",").map(Range::FromStr).collect::<Vec<_>>())
    .filter(|v| v[0].overlaps(&v[1]))
    .count() as u32
}

fn main() {
  let input = include_str!("../../inputs/04.txt");
  let tinput = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
  println!("part1: {}", part1(tinput));
  println!("part1: {}", part1(input));
  println!("part2: {}", part2(tinput));
  println!("part2: {}", part2(input));
}

#[test]
fn test() {
  let input = "";
  // assert_eq!(part1(input), 0);
  // assert_eq!(part2(input), 0);
}
