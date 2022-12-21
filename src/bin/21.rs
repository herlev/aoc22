use itertools::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Op {
  Const(i64),
  Add(String, String),
  Sub(String, String),
  Mul(String, String),
  Div(String, String),
}

impl Op {
  fn eval(&self, h: &HashMap<String, Op>) -> i64 {
    match &self {
      Op::Const(n) => *n,
      Op::Add(a, b) => h.get(a).unwrap().eval(&h) + h.get(b).unwrap().eval(&h),
      Op::Sub(a, b) => h.get(a).unwrap().eval(&h) - h.get(b).unwrap().eval(&h),
      Op::Mul(a, b) => h.get(a).unwrap().eval(&h) * h.get(b).unwrap().eval(&h),
      Op::Div(a, b) => h.get(a).unwrap().eval(&h) / h.get(b).unwrap().eval(&h),
    }
  }
}

fn parse(input: &str) -> HashMap<String, Op> {
  input
    .lines()
    .map(|line| line.split(": ").collect_tuple().unwrap())
    .map(|(name, rest)| {
      let rest = rest.split(' ').collect_vec();
      (
        name.into(),
        if rest.len() == 1 {
          Op::Const(rest[0].parse().unwrap())
        } else {
          let a = rest[0].into();
          let b = rest[2].into();
          match rest[1] {
            "+" => Op::Add(a, b),
            "-" => Op::Sub(a, b),
            "*" => Op::Mul(a, b),
            "/" => Op::Div(a, b),
            _ => panic!(),
          }
        },
      )
    })
    .collect()
}

fn part1(input: &str) -> i64 {
  let h = parse(input);
  h.get("root").unwrap().eval(&h)
}

fn part2(input: &str) -> u32 {
  todo!()
}

fn main() {
  let input = include_str!("../../inputs/21.txt");
  let tinput = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
  println!("part1: {}", part1(tinput));
  println!("part1: {}", part1(input));
  // println!("part2: {}", part2(tinput));
  // println!("part2: {}", part2(input));
  // assert_eq!(part1(tinput), 0);
  // assert_eq!(part1(input), 0);
  // assert_eq!(part2(tinput), 0);
  // assert_eq!(part2(input), 0);
}
