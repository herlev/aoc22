use itertools::*;
use std::{collections::VecDeque, str::FromStr};

#[derive(Debug, Copy, Clone)]
enum Symbol {
  Old,
  Num(u64),
}

impl Symbol {
  fn get(self, old: u64) -> u64 {
    match self {
      Symbol::Old => old,
      Symbol::Num(n) => n,
    }
  }
}

#[derive(Debug, Copy, Clone)]
enum Operation {
  Mult(Symbol, Symbol),
  Add(Symbol, Symbol),
}

impl Operation {
  fn do_op(self, old: u64) -> u64 {
    match self {
      Self::Mult(a, b) => a.get(old) * b.get(old),
      Self::Add(a, b) => a.get(old) + b.get(old),
    }
  }
}

impl FromStr for Operation {
  type Err = ();
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let tokens = s.split(' ').collect::<Vec<_>>();
    let symb = |s| match s {
      "old" => Symbol::Old,
      s => Symbol::Num(s.parse::<u64>().unwrap()),
    };
    Ok(match tokens[1] {
      "*" => Self::Mult(symb(tokens[0]), symb(tokens[2])),
      "+" => Self::Add(symb(tokens[0]), symb(tokens[2])),
      _ => panic!(),
    })
  }
}

#[derive(Debug)]
struct Test {
  divisible_by: u64,
  if_true: u64,
  if_false: u64,
}

#[derive(Debug)]
struct Monkey {
  items: VecDeque<u64>,
  operation: Operation,
  test: Test,
}

impl FromStr for Monkey {
  type Err = ();
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut lines = s.lines();
    lines.next();
    let items = lines
      .next()
      .unwrap()
      .split(": ")
      .nth(1)
      .unwrap()
      .split(", ")
      .map(|n| n.parse::<u64>().unwrap())
      .collect();
    let operation = lines
      .next()
      .unwrap()
      .split(" = ")
      .nth(1)
      .unwrap()
      .parse::<Operation>()
      .unwrap();
    let divisible_by = lines.next().unwrap().split(' ').last().unwrap().parse::<u64>().unwrap();
    let if_true = lines.next().unwrap().split(' ').last().unwrap().parse::<u64>().unwrap();
    let if_false = lines.next().unwrap().split(' ').last().unwrap().parse::<u64>().unwrap();
    Ok(Monkey {
      items,
      operation,
      test: Test {
        divisible_by,
        if_true,
        if_false,
      },
    })
  }
}

fn part1(input: &str) -> u64 {
  let mut monkeys = input
    .split("\n\n")
    .map(|s| s.parse::<Monkey>().unwrap())
    .collect::<Vec<_>>();

  let mut count = vec![0; monkeys.len()];
  for _ in 1..=20 {
    for i in 0..monkeys.len() {
      let monkey = &mut monkeys[i];
      let items = std::mem::take(&mut monkey.items);
      for mut item in items {
        count[i] += 1;
        let monkey = &mut monkeys[i];
        item = monkey.operation.do_op(item);
        item /= 3;
        let to_monkey = if item % monkey.test.divisible_by == 0 {
          monkey.test.if_true
        } else {
          monkey.test.if_false
        };
        monkeys[to_monkey as usize].items.push_back(item);
      }
    }
  }
  count.iter().sorted().rev().take(2).product::<u64>()
}

fn part2(input: &str) -> u64 {
  let mut monkeys = input
    .split("\n\n")
    .map(|s| s.parse::<Monkey>().unwrap())
    .collect::<Vec<_>>();

  let lcm = monkeys
    .iter()
    .map(|m| m.test.divisible_by)
    .reduce(|a, b| num::integer::lcm(a, b))
    .unwrap();

  let mut count = vec![0; monkeys.len()];
  for _ in 1..=10000 {
    for i in 0..monkeys.len() {
      let monkey = &mut monkeys[i];
      let items = std::mem::replace(&mut monkey.items, VecDeque::new());
      for mut item in items {
        count[i] += 1;
        let monkey = &mut monkeys[i];
        item = monkey.operation.do_op(item);
        item %= lcm;

        let to_monkey = if item % monkey.test.divisible_by == 0 {
          monkey.test.if_true
        } else {
          monkey.test.if_false
        };
        monkeys[to_monkey as usize].items.push_back(item);
      }
    }
  }
  count.iter().sorted().rev().take(2).product::<u64>()
}

fn main() {
  let input = include_str!("../../inputs/11.txt");
  let tinput = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

  println!("part1: {}", part1(tinput));
  println!("part1: {}", part1(input));
  println!("part2: {}", part2(tinput));
  println!("part2: {}", part2(input));
  assert_eq!(part1(tinput), 10605);
  assert_eq!(part2(tinput), 2713310158);
  assert_eq!(part1(input), 69918);
  assert_eq!(part2(input), 19573408701);
}
