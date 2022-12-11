use std::str::FromStr;

#[derive(Clone, Copy)]
enum Instruction {
  Noop,
  Addx(i32),
}

impl Instruction {
  fn get_cycles(self) -> usize {
    match self {
      Self::Noop => 1,
      Self::Addx(_) => 2,
    }
  }
  fn to_string(self) -> String {
    match self {
      Self::Noop => "noop".into(),
      Self::Addx(n) => format!("addx {n}"),
    }
  }
}

impl FromStr for Instruction {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut tokens = s.split(' ');
    match tokens.next().unwrap() {
      "noop" => Ok(Instruction::Noop),
      "addx" => Ok(Instruction::Addx(tokens.next().unwrap().parse::<i32>().unwrap())),
      s => panic!("Instruction {s} not valid"),
    }
  }
}

fn part1(input: &str) -> i32 {
  let instructions = input.lines().map(|line| line.parse::<Instruction>().unwrap());
  let mut cycle = 0;
  let mut x = 1i32;
  let mut ss_sum = 0;
  for instruction in instructions {
    for _ in 0..instruction.get_cycles() {
      cycle += 1;
      if (cycle - 20) % 40 == 0 {
        ss_sum += cycle as i32 * x;
        // println!("c: {cycle}, x: {x}, {}", cycle as i32 * x);
      }
    }
    if let Instruction::Addx(v) = instruction {
      x += v;
    }
  }
  ss_sum
}

fn part2(input: &str) {
  let instructions = input.lines().map(|line| line.parse::<Instruction>().unwrap());
  let mut cycle = 1;
  let mut x = 1i32;
  let mut screen: String = "".into();
  for instruction in instructions {
    // println!("x = {x}, begin executing {}", instruction.to_string());
    for _ in 0..instruction.get_cycles() {
      screen += if ((x - 1)..=(x + 1)).contains(&((cycle - 1) % 40)) {
        "#"
      } else {
        "."
      };
      if cycle % 40 == 0 {
        screen += "\n";
      }
      // println!("CRT: {}", screen);
      cycle += 1;
    }
    if let Instruction::Addx(v) = instruction {
      x += v;
    }
  }
  println!("{screen}");
}

fn main() {
  let input = include_str!("../../inputs/10.txt");
  let tinput = include_str!("../../inputs/10t.txt");
  println!("part1: {}", part1(tinput));
  println!("part1: {}", part1(input));
  part2(tinput);
  part2(input);
}
