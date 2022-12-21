use itertools::*;
use std::collections::HashMap;

type Symbol = String;

#[derive(Debug, Clone)]
struct Expression {
  lhs: Symbol,
  rhs: Symbol,
  op: Operator,
}

#[derive(Debug, Clone)]
enum Operator {
  Add,
  Sub,
  Mul,
  Div,
}

impl Operator {
  fn to_string(&self) -> String {
    match self {
      Self::Add => "+".into(),
      Self::Sub => "-".into(),
      Self::Mul => "*".into(),
      Self::Div => "/".into(),
    }
  }
}

fn parse(input: &str) -> (HashMap<Symbol, Expression>, HashMap<Symbol, i64>) {
  let mut expressions = HashMap::new();
  let mut constants = HashMap::new();
  input
    .lines()
    .map(|line| line.split(": ").collect_tuple().unwrap())
    .for_each(|(name, rest)| {
      let rest = rest.split(' ').collect_vec();
      if rest.len() == 1 {
        constants.insert(name.into(), rest[0].parse().unwrap());
      } else {
        let lhs = rest[0].into();
        let rhs = rest[2].into();
        let op = match rest[1] {
          "+" => Operator::Add,
          "-" => Operator::Sub,
          "*" => Operator::Mul,
          "/" => Operator::Div,
          _ => panic!(),
        };
        expressions.insert(name.into(), Expression { lhs, rhs, op });
      }
    });
  (expressions, constants)
}

fn reduce(sym: &Symbol, mut expressions: &mut HashMap<Symbol, Expression>, mut constants: &mut HashMap<Symbol, i64>) {
  if constants.contains_key(sym) {
    return;
  }
  if expressions.contains_key(sym) {
    let exp = (*expressions.get(sym).unwrap()).clone();
    reduce(&exp.lhs, &mut expressions, &mut constants);
    reduce(&exp.rhs, &mut expressions, &mut constants);
    if constants.contains_key(&exp.lhs) && constants.contains_key(&exp.rhs) {
      let lhs = constants.get(&exp.lhs).unwrap();
      let rhs = constants.get(&exp.rhs).unwrap();
      let val = match exp.op {
        Operator::Add => lhs + rhs,
        Operator::Sub => lhs - rhs,
        Operator::Mul => lhs * rhs,
        Operator::Div => lhs / rhs,
      };
      constants.insert(sym.into(), val);
      expressions.remove(sym);
    }
  }
}

fn part1(input: &str) -> i64 {
  let (mut exp, mut con) = parse(input);
  reduce(&"root".to_string(), &mut exp, &mut con);
  *con.get("root").unwrap()
}

fn sym_to_string(sym: &Symbol, expressions: &HashMap<Symbol, Expression>, constants: &HashMap<Symbol, i64>) -> String {
  if let Some(n) = constants.get(sym) {
    return n.to_string();
  } else if let Some(exp) = expressions.get(sym) {
    return exp.to_string(expressions, constants);
  }
  sym.to_string()
}

impl Expression {
  fn to_string(&self, expressions: &HashMap<Symbol, Expression>, constants: &HashMap<Symbol, i64>) -> String {
    format!(
      "({} {} {})",
      sym_to_string(&self.lhs, &expressions, &constants),
      self.op.to_string(),
      sym_to_string(&self.rhs, &expressions, &constants)
    )
  }
}

fn part2(input: &str) -> i64 {
  let (mut exp, mut con) = parse(input);
  con.remove("humn");
  reduce(&"root".to_string(), &mut exp, &mut con);
  let root = exp.get("root").unwrap();
  let eq = format!(
    "{} - {}",
    sym_to_string(&root.lhs, &exp, &con),
    sym_to_string(&root.rhs, &exp, &con)
  );

  // Solve equation using sympy
  let python_cmd = format!(
    "from sympy import *; humn = symbols('humn'); print(solve({}, humn)[0])",
    eq
  );
  // dbg!(eq);
  let ans = std::process::Command::new("python")
    .args(["-c", &python_cmd])
    .output()
    .unwrap();
  let s = String::from_utf8(ans.stdout).unwrap();
  s.trim().parse::<i64>().unwrap()
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
  println!("part2: {}", part2(tinput));
  println!("part2: {}", part2(input));
  assert_eq!(part1(tinput), 152);
  assert_eq!(part1(input), 10037517593724);
  assert_eq!(part2(tinput), 301);
  assert_eq!(part2(input), 3272260914328);
}
