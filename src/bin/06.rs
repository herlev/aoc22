use std::collections::HashSet;

fn solve(input: &str, win_size: usize) -> u32 {
  for (i, w) in input.chars().collect::<Vec<_>>().windows(win_size).enumerate() {
    if w.iter().collect::<HashSet<_>>().len() == win_size {
      return (i + win_size) as u32;
    }
  }
  panic!()
}

fn solve_iter(input: &str, win_size: usize) -> u32 {
  input
    .chars()
    .collect::<Vec<_>>()
    .windows(win_size)
    .enumerate()
    .skip_while(|(_, w)| w.iter().collect::<HashSet<_>>().len() != win_size)
    .next()
    .unwrap()
    .0 as u32
    + win_size as u32
}

fn part1(input: &str) -> u32 {
  assert_eq!(solve_iter(input, 4), solve(input, 4));
  solve(input, 4)
}

fn part2(input: &str) -> u32 {
  assert_eq!(solve_iter(input, 4), solve(input, 4));
  solve(input, 14)
}

fn main() {
  let input = include_str!("../../inputs/06.txt");
  let tinput = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
  println!("part1: {}", part1(tinput));
  println!("part1: {}", part1(input));
  println!("part2: {}", part2(tinput));
  println!("part2: {}", part2(input));
  assert_eq!(part1(tinput), 7);
  assert_eq!(part2(tinput), 19);
  assert_eq!(part1(input), 1896);
  assert_eq!(part2(input), 3452);
}
