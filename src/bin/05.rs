use itertools::*;
use std::collections::HashMap;
use std::collections::VecDeque;

type Stacks = HashMap<u32, VecDeque<char>>;

fn parse(s: &str) -> Stacks {
  let mut h: Stacks = HashMap::new();
  s.lines().for_each(|line| {
    for (i, c) in line.chars().enumerate() {
      if matches!(c, 'A'..='Z') {
        let idx = (i - 1) / 4 + 1;
        h.entry(idx as u32).or_default().push_front(c);
      }
    }
  });
  h
}

fn part1(input: &str) -> String {
  let (c, i) = input.split("\n\n").collect_tuple::<(&str, &str)>().unwrap();
  let mut c = parse(c);
  for line in i.lines() {
    let (count, from, to) = line
      .split(' ')
      .filter_map(|e| e.parse::<u32>().ok())
      .collect_tuple()
      .unwrap();
    for _ in 0..count {
      let e = c.entry(from).or_default().pop_back().unwrap();
      c.entry(to).or_default().push_back(e);
    }
  }
  c.iter().sorted_by_key(|&e| e.0).filter_map(|e| e.1.back()).join("")
}

fn part2(input: &str) -> String {
  let (c, i) = input.split("\n\n").collect_tuple::<(&str, &str)>().unwrap();
  let mut c = parse(c);
  for line in i.lines() {
    let (count, from, to) = line
      .split(' ')
      .filter_map(|e| e.parse::<u32>().ok())
      .collect_tuple()
      .unwrap();
    let from = &mut c.entry(from).or_default();
    let mut v = from.split_off(from.len() - count as usize);
    c.entry(to).or_default().append(&mut v);
  }
  c.iter().sorted_by_key(|&e| e.0).filter_map(|e| e.1.back()).join("")
}

fn main() {
  let input = include_str!("../../inputs/05.txt");
  let tinput = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
  println!("part1: {}", part1(tinput));
  println!("part1: {}", part1(input));
  println!("part2: {}", part2(tinput));
  println!("part2: {}", part2(input));
  assert_eq!(part1(tinput), "CMZ");
  assert_eq!(part1(input), "VPCDMSLWJ");
  assert_eq!(part2(tinput), "MCD");
  assert_eq!(part2(input), "TPWCGNCCG");
}
