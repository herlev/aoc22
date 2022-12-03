use std::collections::BinaryHeap;

fn parse_list(s: &str) -> Vec<u32> {
  s.split("\n\n")
    .map(|g| g.lines().flat_map(str::parse::<u32>).sum())
    .collect()
}

fn part1(input: &str) -> u32 {
  parse_list(input).into_iter().max().unwrap()
}

fn part2(input: &str) -> u32 {
  let mut h = BinaryHeap::from(parse_list(input));
  (0..3).filter_map(|_| h.pop()).take(3).sum()
}

fn main() {
  let input = include_str!("../../inputs/01.txt");
  println!("part1: {}\n", part1(input));
  println!("part2: {}\n", part2(input));
}

#[test]
fn test() {
  let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
  assert_eq!(part1(input), 24000u32);
  assert_eq!(part2(input), 45000);
}
