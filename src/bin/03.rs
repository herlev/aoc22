use itertools::*;
use std::{collections::HashSet, hash::Hash};

fn get_priority(c: char) -> u32 {
  match c {
    'a'..='z' => c as u32 - 'a' as u32 + 1,
    'A'..='Z' => c as u32 - 'A' as u32 + 27,
    _ => panic!(),
  }
}

fn intersection<T: Eq + Hash>(mut it: impl Iterator<Item = HashSet<T>>) -> HashSet<T> {
  let mut res = it.next().unwrap_or_default();
  it.for_each(|h| res.retain(|e| h.contains(e)));
  res
}

fn part1(input: &str) -> u32 {
  input
    .lines()
    .map(|line| line.split_at(line.len() / 2))
    .map(|(left, right)| intersection([left.chars().collect(), right.chars().collect()].into_iter()))
    .map(|isec| isec.into_iter().map(get_priority).sum::<u32>())
    .sum()
}

fn part2(input: &str) -> u32 {
  input
    .lines()
    .chunks(3)
    .into_iter()
    .map(|chunk| {
      intersection(chunk.map(|line| line.chars().collect()))
        .into_iter()
        .map(get_priority)
        .sum::<u32>()
    })
    .sum()
}

fn main() {
  let input = include_str!("../../inputs/03.txt");
  let tinput = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
  println!("part1: {}\n", part1(tinput));
  println!("part1: {}\n", part1(input));
  println!("part2: {}\n", part2(tinput));
  println!("part2: {}\n", part2(input));
}

#[test]
fn test() {
  assert_eq!(get_priority('a'), 1);
  assert_eq!(get_priority('z'), 26);
  assert_eq!(get_priority('A'), 27);
  assert_eq!(get_priority('Z'), 52);
}
